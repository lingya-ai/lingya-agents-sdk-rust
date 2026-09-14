use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use futures_util::StreamExt;
use lingya_agents_sdk::models::{
    AiChatSubmission, ConversationShareCreated, GeneratePreSignedUrlOutput,
};
use lingya_agents_sdk::sse::SseDecoder;
use lingya_agents_sdk::{
    LingyaAgentsClient, LingyaAgentsUserClient, LingyaError, OpenApiCredentials, QueryParameter,
};
use regex::Regex;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::json;

const BASE_PATH: &str = "/api/agents/channel/openapi/v1/{channelId}/chat";

#[tokio::test]
async fn all_46_real_endpoints() -> Result<(), Box<dyn Error>> {
    let Some(access_key) = env("OPENAPI_AK") else {
        eprintln!("skipped: live environment variables are required");
        return Ok(());
    };
    let secret_key = required_env("OPENAPI_SK")?;
    let base_url = required_env("LINGYA_LIVE_BASE_URL")?;
    let channel_id = required_env("LINGYA_LIVE_CHANNEL_ID")?;
    let client = LingyaAgentsClient::new(
        base_url,
        channel_id,
        OpenApiCredentials::new(access_key, secret_key),
    )?;
    let user = client.for_user(
        env("LINGYA_LIVE_EXTERNAL_USER_ID")
            .unwrap_or_else(|| "lingya-rust-sdk-all-endpoints".into()),
    )?;
    let mut coverage = Coverage::new(user);
    let first: AiChatSubmission = coverage
        .model(
            Method::POST,
            "",
            Some(json!({"query": "仅回复英文 OK"}).to_string()),
        )
        .await?;

    let scenario = run_scenario(&mut coverage, &first).await;
    let cleanup = coverage
        .success(
            Method::DELETE,
            &format!("/conversations/{}", first.conversation_id),
            None,
            vec![],
            "application/json",
        )
        .await;
    coverage.write_report()?;
    scenario?;
    cleanup?;

    assert_eq!(coverage.seen.len(), 46);
    assert_eq!(coverage.seen, published_endpoints()?);
    Ok(())
}

async fn run_scenario(
    coverage: &mut Coverage,
    first: &AiChatSubmission,
) -> Result<(), Box<dyn Error>> {
    coverage
        .success(Method::GET, "/config", None, vec![], "application/json")
        .await?;
    let mut stream = coverage
        .user
        .stream_chat_events(&first.conversation_id, &first.message_id)
        .await?;
    let mut count = 0;
    let mut saw_end = false;
    while let Some(event) = stream.next().await {
        let event = event?;
        count += 1;
        saw_end |= event.event_type() == "end";
    }
    drop(stream);
    assert!(count > 0 && saw_end);
    coverage.record(
        Method::POST,
        "/conversations/{conversationId}/stream",
        200,
        "流式响应完成",
    )?;
    let conversation = &first.conversation_id;
    let message = &first.message_id;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/config"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/context-usage"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/conversations",
            None,
            query(&[("current", "0"), ("size", "5")]),
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/conversations/active",
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/conversations/unread",
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::POST,
            "/conversations/activity/query",
            Some(json!({"conversationIds": [conversation]}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::PUT,
            &format!("/conversations/{conversation}/read-receipt"),
            Some(json!({"messageId": message}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/conversations/stats",
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::PATCH,
            &format!("/conversations/{conversation}/title"),
            Some(json!({"title": "Rust SDK 全接口测试"}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/title"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/messages"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/messages/{message}"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/events",
            None,
            vec![
                QueryParameter::new("conversationId", conversation),
                QueryParameter::new("messageId", message),
            ],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::POST,
            "/events/batch",
            Some(json!({"conversationId": conversation, "messageIds": [message]}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    let second: AiChatSubmission = coverage
        .model(
            Method::POST,
            &format!("/conversations/{conversation}"),
            Some(json!({"query": "再次仅回复英文 OK"}).to_string()),
        )
        .await?;
    let mut second_stream = coverage
        .user
        .stream_chat_events(&second.conversation_id, &second.message_id)
        .await?;
    while let Some(event) = second_stream.next().await {
        event?;
    }
    drop(second_stream);
    coverage
        .success(
            Method::DELETE,
            &format!("/conversations/{conversation}/interrupt"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::POST,
            &format!("/conversations/{conversation}/compact"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/async-tasks"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/async-tasks/missing-async-task"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::DELETE,
            &format!("/conversations/{conversation}/messages/{message}/queue"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    let share: ConversationShareCreated = coverage
        .model(
            Method::POST,
            &format!("/conversations/{conversation}/shares"),
            Some("{}".into()),
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/shares"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::DELETE,
            &format!("/conversations/{conversation}/shares/{}", share.share_id),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::POST,
            "/plan/approve",
            Some(
                json!({"conversationId": conversation, "messageId": message, "approved": false})
                    .to_string(),
            ),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/plan/missing-plan/status",
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            "/user-input/missing-question/status",
            None,
            vec![
                QueryParameter::new("conversationId", conversation),
                QueryParameter::new("messageId", message),
            ],
            "application/json",
        )
        .await?;
    coverage.success(Method::POST, "/user-input/answer", Some(json!({"conversationId": conversation, "messageId": message, "questionId": "missing-question", "selectedOptions": [], "customInput": "not pending"}).to_string()), vec![], "application/json").await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/sql-query-results/missing-result"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/sql-query-results/missing-result/chart-data"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/sql-query-results/missing-result/export"),
            None,
            query(&[("format", "CSV")]),
            "text/csv",
        )
        .await?;
    let md5 = "17/2WOZXDPjhZzwMQCHrDg==";
    coverage
        .success(
            Method::GET,
            "/files/meta/contentMd5",
            None,
            vec![QueryParameter::new("contentMd5", md5)],
            "application/json",
        )
        .await?;
    let upload: GeneratePreSignedUrlOutput = coverage.model(Method::POST, "/files/pre-signed-url/write", Some(json!({"fileName": "lingya-sdk-endpoint-test.txt", "module": "ai-chat-attachments", "contentMd5": md5}).to_string())).await?;
    coverage
        .domain(
            Method::POST,
            "/files/pre-signed-url/confirm",
            Some(json!({"fileUk": upload.file_uk, "contentMd5": md5}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::POST,
            "/files/contentMd5",
            Some(
                json!({"fileName": "lingya-sdk-endpoint-test.txt", "contentMd5": md5}).to_string(),
            ),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/files/9223372036854775807/preview"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage.domain(Method::GET, &format!("/conversations/{conversation}/messages/{message}/plan-intermediate-files/9223372036854775807/preview"), None, vec![], "application/json").await?;
    coverage
        .success(
            Method::POST,
            "/knowledge-bases/citations/metadata",
            Some("[]".into()),
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            "/knowledge-bases/citations/CHUNK/9223372036854775807/metadata",
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .success(
            Method::GET,
            &format!("/conversations/{conversation}/workspace/files"),
            None,
            vec![],
            "application/json",
        )
        .await?;
    coverage
        .domain(
            Method::GET,
            &format!("/conversations/{conversation}/workspace/files/preview"),
            None,
            query(&[("path", "missing-file.txt")]),
            "application/json",
        )
        .await?;
    let probe_body = json!({"probeId": format!("all-{}", uuid::Uuid::new_v4())}).to_string();
    let probe = coverage
        .user
        .raw_response(
            Method::POST,
            "/stream-probe",
            Some(&probe_body),
            &[],
            "text/event-stream",
        )
        .await?;
    let status = probe.status().as_u16();
    let mut decoder = SseDecoder::default();
    let events = decoder.push(&probe.bytes().await?)?;
    assert_eq!(events.len(), 4);
    coverage.record(Method::POST, "/stream-probe", status, "流式响应完成")?;
    coverage
        .success(
            Method::PATCH,
            &format!("/conversations/{conversation}/status"),
            Some(json!({"status": "ARCHIVED"}).to_string()),
            vec![],
            "application/json",
        )
        .await?;
    Ok(())
}

struct Coverage {
    user: LingyaAgentsUserClient,
    results: Vec<ResultRow>,
    seen: BTreeSet<String>,
}

struct ResultRow {
    method: String,
    path: String,
    status: u16,
    outcome: String,
}

impl Coverage {
    fn new(user: LingyaAgentsUserClient) -> Self {
        Self {
            user,
            results: vec![],
            seen: BTreeSet::new(),
        }
    }

    async fn success(
        &mut self,
        method: Method,
        suffix: &str,
        body: Option<String>,
        query: Vec<QueryParameter>,
        accept: &str,
    ) -> Result<(), LingyaError> {
        let response = self
            .user
            .raw_response(method.clone(), suffix, body.as_deref(), &query, accept)
            .await?;
        self.record(method, suffix, response.status().as_u16(), "通过")
            .map_err(LingyaError::InvalidInput)
    }

    async fn model<T: DeserializeOwned>(
        &mut self,
        method: Method,
        suffix: &str,
        body: Option<String>,
    ) -> Result<T, LingyaError> {
        let response = self
            .user
            .raw_response(
                method.clone(),
                suffix,
                body.as_deref(),
                &[],
                "application/json",
            )
            .await?;
        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        self.record(method, suffix, status, "通过")
            .map_err(LingyaError::InvalidInput)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn domain(
        &mut self,
        method: Method,
        suffix: &str,
        body: Option<String>,
        query: Vec<QueryParameter>,
        accept: &str,
    ) -> Result<(), LingyaError> {
        match self
            .user
            .raw_response(method.clone(), suffix, body.as_deref(), &query, accept)
            .await
        {
            Err(LingyaError::Http { status, .. })
                if matches!(status.as_u16(), 400 | 403 | 404 | 409 | 422) =>
            {
                self.record(
                    method,
                    suffix,
                    status.as_u16(),
                    "环境能力受限，参数与错误响应已验证",
                )
                .map_err(LingyaError::InvalidInput)
            }
            Err(error) => Err(error),
            Ok(response) => Err(LingyaError::InvalidInput(format!(
                "expected domain error but got {}",
                response.status()
            ))),
        }
    }

    fn record(
        &mut self,
        method: Method,
        suffix: &str,
        status: u16,
        outcome: &str,
    ) -> Result<(), String> {
        let path = format!("{BASE_PATH}{}", canonical_suffix(suffix));
        let key = format!("{} {path}", method.as_str());
        if !self.seen.insert(key.clone()) {
            return Err(format!("duplicate endpoint: {key}"));
        }
        self.results.push(ResultRow {
            method: method.to_string(),
            path,
            status,
            outcome: outcome.into(),
        });
        Ok(())
    }

    fn write_report(&self) -> Result<(), std::io::Error> {
        let directory = PathBuf::from("build/reports/live-api");
        fs::create_dir_all(&directory)?;
        let mut report = String::from("# Rust SDK 真实环境全接口测试报告\n\n| 请求标识 | Method | Path | HTTP | 结果 |\n|---|---|---|---:|---|\n");
        for (index, row) in self.results.iter().enumerate() {
            report.push_str(&format!(
                "| local-{:03} | {} | `{}` | {} | {} |\n",
                index + 1,
                row.method,
                row.path,
                row.status,
                row.outcome
            ));
        }
        fs::write(directory.join("all-endpoints.md"), report)
    }
}

fn canonical_suffix(suffix: &str) -> String {
    let mut value = suffix.to_owned();
    if let Some(rest) = suffix.strip_prefix("/conversations/") {
        let segment = rest.split('/').next().unwrap_or_default();
        if !matches!(segment, "active" | "unread" | "stats" | "activity") {
            value = format!(
                "/conversations/{{conversationId}}{}",
                rest.strip_prefix(segment).unwrap_or_default()
            );
        }
    }
    for (pattern, replacement) in [
        (r"/async-tasks/[^/]+", "/async-tasks/{asyncTaskId}"),
        (r"/messages/[^/]+", "/messages/{messageId}"),
        (
            r"/plan-intermediate-files/[^/]+",
            "/plan-intermediate-files/{fileId}",
        ),
        (
            r"^/conversations/\{conversationId\}/files/[^/]+",
            "/conversations/{conversationId}/files/{fileId}",
        ),
        (r"/shares/[^/]+", "/shares/{shareId}"),
        (r"/sql-query-results/[^/]+", "/sql-query-results/{resultId}"),
        (r"^/plan/[^/]+/status$", "/plan/{planId}/status"),
        (
            r"^/user-input/[^/]+/status$",
            "/user-input/{questionId}/status",
        ),
        (
            r"^/knowledge-bases/citations/[^/]+/[^/]+/metadata$",
            "/knowledge-bases/citations/{citationType}/{referenceId}/metadata",
        ),
    ] {
        value = Regex::new(pattern)
            .unwrap()
            .replace(&value, replacement)
            .into_owned();
    }
    value
}

fn published_endpoints() -> Result<BTreeSet<String>, Box<dyn Error>> {
    let contract = std::env::var_os("LINGYA_CONTRACT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../lingya-agents-openapi")
        });
    let yaml = fs::read_to_string(contract.join("openapi/lingya-agents-v1.yaml"))?;
    let path_re = Regex::new(r"^  (/api/agents/channel/openapi/[^:]+):$")?;
    let method_re = Regex::new(r"^    (get|post|put|patch|delete):$")?;
    let mut current_path = String::new();
    let mut endpoints = BTreeSet::new();
    for line in yaml.lines() {
        if let Some(captures) = path_re.captures(line) {
            current_path = captures[1].into();
        }
        if let Some(captures) = method_re.captures(line) {
            endpoints.insert(format!("{} {}", captures[1].to_uppercase(), current_path));
        }
    }
    Ok(endpoints)
}

fn query(items: &[(&str, &str)]) -> Vec<QueryParameter> {
    items
        .iter()
        .map(|(name, value)| QueryParameter::new(*name, *value))
        .collect()
}

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn required_env(name: &str) -> Result<String, Box<dyn Error>> {
    env(name).ok_or_else(|| format!("{name} is required").into())
}
