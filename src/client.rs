//! HMAC 请求构造与可信服务端客户端。

use std::fmt;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

use async_stream::try_stream;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use futures_util::{Stream, StreamExt};
use hmac::{Hmac, Mac};
use reqwest::{Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};
use thiserror::Error;
use url::form_urlencoded;

use crate::events::{decode_ai_chat_brief_event, LingyaAiChatBriefEvent};
use crate::sse::SseDecoder;

type HmacSha256 = Hmac<Sha256>;

/// 服务端 OpenAPI 凭证；调试输出始终隐藏 secret。
#[derive(Clone)]
pub struct OpenApiCredentials {
    /// 公开访问标识。
    pub access_key: String,
    secret_key: String,
}

impl OpenApiCredentials {
    /// 创建一组服务端凭证。
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }
}

impl fmt::Debug for OpenApiCredentials {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenApiCredentials")
            .field("access_key", &self.access_key)
            .field("secret_key", &"<redacted>")
            .finish()
    }
}

/// 保留顺序并允许重复名称的查询参数。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryParameter {
    /// 查询参数名称。
    pub name: String,
    /// 已编码前的查询参数值。
    pub value: String,
}

impl QueryParameter {
    /// 创建查询参数。
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// SDK 请求、协议或解码错误；错误文本不包含 secret。
#[derive(Debug, Error)]
pub enum LingyaError {
    /// 服务端返回非 2xx 状态。
    #[error("{method} {path} returned HTTP {status}")]
    Http {
        /// HTTP 方法。
        method: String,
        /// 相对路由，不包含凭证与查询值。
        path: String,
        /// HTTP 状态码。
        status: StatusCode,
        /// 用于诊断的响应正文，最多保留 64 KiB。
        response_body: String,
    },
    /// HTTP transport 失败。
    #[error("HTTP transport failed")]
    Transport(#[from] reqwest::Error),
    /// JSON 序列化或反序列化失败。
    #[error("JSON processing failed")]
    Json(#[from] serde_json::Error),
    /// SSE 中存在无效 UTF-8。
    #[error("SSE stream contains invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    /// 调用参数违反协议限制。
    #[error("invalid SDK input: {0}")]
    InvalidInput(String),
}

/// Lingya Agents SDK 入口。
#[derive(Clone, Debug)]
pub struct LingyaAgentsClient {
    base_url: String,
    channel_id: String,
    credentials: OpenApiCredentials,
}

impl LingyaAgentsClient {
    /// 创建客户端。base URL 不应包含 `/api/.../chat` 路径。
    pub fn new(
        base_url: impl Into<String>,
        channel_id: impl Into<String>,
        credentials: OpenApiCredentials,
    ) -> Result<Self, LingyaError> {
        let channel_id = channel_id.into();
        if channel_id.is_empty() {
            return Err(LingyaError::InvalidInput(
                "channel_id must not be empty".into(),
            ));
        }
        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            channel_id,
            credentials,
        })
    }

    /// 绑定调用方系统中的外部用户身份。
    pub fn for_user(
        &self,
        external_user_id: impl AsRef<str>,
    ) -> Result<LingyaAgentsUserClient, LingyaError> {
        LingyaAgentsUserClient::new(self, external_user_id.as_ref())
    }
}

/// 已绑定外部用户、可执行 HMAC 请求的异步客户端。
#[derive(Clone, Debug)]
pub struct LingyaAgentsUserClient {
    root: String,
    credentials: OpenApiCredentials,
    encoded_user: String,
    http: reqwest::Client,
}

impl LingyaAgentsUserClient {
    fn new(client: &LingyaAgentsClient, external_user_id: &str) -> Result<Self, LingyaError> {
        let bytes = external_user_id.as_bytes();
        if bytes.is_empty() || bytes.len() > 256 || bytes.contains(&0) {
            return Err(LingyaError::InvalidInput(
                "external_user_id must contain 1 to 256 UTF-8 bytes and no NUL".into(),
            ));
        }
        let channel =
            form_urlencoded::byte_serialize(client.channel_id.as_bytes()).collect::<String>();
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            root: format!(
                "{}/api/agents/channel/openapi/v1/{channel}/chat",
                client.base_url
            ),
            credentials: client.credentials.clone(),
            encoded_user: URL_SAFE_NO_PAD.encode(bytes),
            http,
        })
    }

    /// 调用 GET JSON 接口并反序列化为明确响应类型。
    pub async fn get_model<T: DeserializeOwned>(
        &self,
        suffix: &str,
        query: &[QueryParameter],
    ) -> Result<T, LingyaError> {
        let response = self
            .raw_response(Method::GET, suffix, None, query, "application/json")
            .await?;
        Ok(response.json().await?)
    }

    /// 调用带 JSON 请求体的接口并反序列化为明确响应类型。
    pub async fn send_model<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        suffix: &str,
        body: &B,
        query: &[QueryParameter],
    ) -> Result<T, LingyaError> {
        let body = serde_json::to_string(body)?;
        let response = self
            .raw_response(method, suffix, Some(&body), query, "application/json")
            .await?;
        Ok(response.json().await?)
    }

    /// 下载完整二进制响应。
    pub async fn request_bytes(
        &self,
        suffix: &str,
        query: &[QueryParameter],
        accept: &str,
    ) -> Result<Vec<u8>, LingyaError> {
        let response = self
            .raw_response(Method::GET, suffix, None, query, accept)
            .await?;
        Ok(response.bytes().await?.to_vec())
    }

    /// 发送受控底层请求。请求体必须是最终签名所用的 UTF-8 JSON 字节。
    pub async fn raw_response(
        &self,
        method: Method,
        suffix: &str,
        body: Option<&str>,
        query: &[QueryParameter],
        accept: &str,
    ) -> Result<Response, LingyaError> {
        let method_name = method.as_str().to_owned();
        let request = self.signed_request(method, suffix, body, query, accept)?;
        let response = self.http.execute(request).await?;
        if response.status().is_success() {
            return Ok(response);
        }
        let status = response.status();
        let mut response_body = response.text().await.unwrap_or_default();
        let mut boundary = response_body.len().min(65_536);
        while !response_body.is_char_boundary(boundary) {
            boundary -= 1;
        }
        response_body.truncate(boundary);
        Err(LingyaError::Http {
            method: method_name,
            path: suffix.to_owned(),
            status,
            response_body,
        })
    }

    /// 订阅会话 SSE，逐项返回 15 种强类型事件或未知 raw JSON fallback。
    pub async fn stream_chat_events(
        &self,
        conversation_id: &str,
        message_id: &str,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<LingyaAiChatBriefEvent, LingyaError>> + Send>>,
        LingyaError,
    > {
        #[derive(Serialize)]
        struct StreamInput<'a> {
            #[serde(rename = "messageId")]
            message_id: &'a str,
        }

        let conversation =
            form_urlencoded::byte_serialize(conversation_id.as_bytes()).collect::<String>();
        let suffix = format!("/conversations/{conversation}/stream");
        let body = serde_json::to_string(&StreamInput { message_id })?;
        let response = self
            .raw_response(Method::POST, &suffix, Some(&body), &[], "text/event-stream")
            .await?;
        let mut chunks = response.bytes_stream();
        let stream = try_stream! {
            let mut decoder = SseDecoder::default();
            while let Some(chunk) = chunks.next().await {
                for data in decoder.push(&chunk?)? {
                    yield decode_ai_chat_brief_event(&data)?;
                }
            }
            for data in decoder.finish()? {
                yield decode_ai_chat_brief_event(&data)?;
            }
        };
        Ok(Box::pin(stream))
    }

    fn signed_request(
        &self,
        method: Method,
        suffix: &str,
        body: Option<&str>,
        query: &[QueryParameter],
        accept: &str,
    ) -> Result<reqwest::Request, LingyaError> {
        let body_bytes = body.unwrap_or_default().as_bytes();
        if body_bytes.len() > 2 * 1024 * 1024 {
            return Err(LingyaError::InvalidInput(
                "request body exceeds the 2 MiB signing limit".into(),
            ));
        }
        let mut query_serializer = form_urlencoded::Serializer::new(String::new());
        for item in query {
            query_serializer.append_pair(&item.name, &item.value);
        }
        let raw_query = query_serializer.finish();
        let url_text = format!(
            "{}{}{}",
            self.root,
            suffix,
            if raw_query.is_empty() {
                String::new()
            } else {
                format!("?{raw_query}")
            }
        );
        let parsed_url = url::Url::parse(&url_text)
            .map_err(|error| LingyaError::InvalidInput(format!("invalid request URL: {error}")))?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| LingyaError::InvalidInput("system clock is before Unix epoch".into()))?
            .as_secs()
            .to_string();
        let mut nonce_bytes = [0_u8; 16];
        getrandom::fill(&mut nonce_bytes).map_err(|error| {
            LingyaError::InvalidInput(format!("nonce generation failed: {error}"))
        })?;
        let nonce = URL_SAFE_NO_PAD.encode(nonce_bytes);
        let content_type = if body.is_some() {
            "application/json"
        } else {
            ""
        };
        let body_hash = hex::encode(Sha256::digest(body_bytes));
        let canonical = [
            "OPENAPI-HMAC-SHA256-V1",
            &self.credentials.access_key,
            &timestamp,
            &nonce,
            method.as_str(),
            parsed_url.path(),
            &raw_query,
            &self.encoded_user,
            content_type,
            &body_hash,
        ]
        .join("\n");
        let signature = sign_canonical(&self.credentials.secret_key, &canonical)?;
        let mut builder = self
            .http
            .request(method, parsed_url)
            .header("Accept", accept)
            .header("X-OpenAPI-AK", &self.credentials.access_key)
            .header("X-OpenAPI-Timestamp", timestamp)
            .header("X-OpenAPI-Nonce", nonce)
            .header("X-OpenAPI-User", &self.encoded_user)
            .header("X-OpenAPI-Signature", signature);
        if body.is_some() {
            builder = builder
                .header("Content-Type", content_type)
                .body(body_bytes.to_vec());
        }
        Ok(builder.build()?)
    }
}

/// 对固定 canonical string 计算 HMAC，供协议 golden vector 验证。
pub fn sign_canonical(secret_key: &str, canonical: &str) -> Result<String, LingyaError> {
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .map_err(|_| LingyaError::InvalidInput("invalid HMAC key".into()))?;
    mac.update(canonical.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}
