use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use futures_util::StreamExt;
use lingya_agents_sdk::models::generate_pre_signed_url_input::Module;
use lingya_agents_sdk::models::{
    AiChatEventsBatchInput, AiChatInput, AiChatStreamInput, AiChatSubmission, ChatStreamProbeInput,
    ConfirmUploadInput, ConversationActivityBatchInput, ConversationReadReceiptInput,
    ConversationShareInput, ConversationStatusInput, ConversationTitleInput, CreateFileInput,
    GeneratePreSignedUrlInput, PlanApprovalInput, UserInputAnswerInput,
};
use lingya_agents_sdk::{
    CompactConversationOptions, ExportSqlQueryResultOptions, FileExistsByContentMd5Options,
    GetChatEventsOptions, GetSqlQueryResultOptions, GetUserInputStatusOptions,
    GetWorkspaceFilePreviewOptions, LingyaAgentsClient, LingyaAgentsUserClient, LingyaError,
    ListConversationAsyncTasksOptions, ListConversationMessagesOptions, ListConversationsOptions,
    ListWorkspaceArtifactsOptions, OpenApiCredentials, ProbeEventStreamOptions, SqlExportFormat,
    StreamChatEventsOptions,
};
use regex::Regex;
use reqwest::Method;

const BASE_PATH: &str = "/api/agents/channel/openapi/v1/{channelId}/chat";

macro_rules! success {
    ($coverage:expr, $method:expr, $suffix:expr, $future:expr) => {{
        let value = $future.await?;
        $coverage.record($method, $suffix, 200, "通过")?;
        value
    }};
    ($coverage:expr, $method:expr, $suffix:expr, $status:expr, $future:expr) => {{
        let value = $future.await?;
        $coverage.record($method, $suffix, $status, "通过")?;
        value
    }};
}

macro_rules! domain {
    ($coverage:expr, $method:expr, $suffix:expr, $future:expr) => {{
        match $future.await {
            Err(LingyaError::Http { status, .. })
                if matches!(status.as_u16(), 400 | 403 | 404 | 409 | 422) =>
            {
                $coverage.record(
                    $method,
                    $suffix,
                    status.as_u16(),
                    "环境能力受限，参数与错误响应已验证",
                )?;
            }
            Err(error) => return Err(error.into()),
            Ok(_) => {
                return Err(LingyaError::InvalidInput(format!(
                    "{} {} unexpectedly succeeded",
                    $method.as_str(),
                    $suffix
                ))
                .into());
            }
        }
    }};
}

#[tokio::test]
async fn all_46_real_endpoints() -> Result<(), Box<dyn Error>> {
    let Some(access_key) = env("OPENAPI_AK") else {
        eprintln!("skipped: live environment variables are required");
        return Ok(());
    };
    let client = LingyaAgentsClient::new(
        required_env("LINGYA_LIVE_BASE_URL")?,
        required_env("LINGYA_LIVE_CHANNEL_ID")?,
        OpenApiCredentials::new(access_key, required_env("OPENAPI_SK")?),
    )?;
    let user = client.for_user(
        env("LINGYA_LIVE_EXTERNAL_USER_ID")
            .unwrap_or_else(|| "lingya-rust-sdk-all-endpoints".into()),
    )?;
    let mut coverage = Coverage::new(user);
    let first_input = AiChatInput::new("仅回复英文 OK".into());
    let first: AiChatSubmission = success!(
        coverage,
        Method::POST,
        "",
        201,
        coverage.user.chat().create_chat(&first_input)
    );

    let scenario = run_scenario(&mut coverage, &first).await;
    let cleanup_result = coverage
        .user
        .conversations()
        .delete_conversation(&first.conversation_id)
        .await;
    if cleanup_result.is_ok() {
        coverage.record(
            Method::DELETE,
            "/conversations/{conversationId}",
            200,
            "通过",
        )?;
    }
    coverage.write_report()?;
    scenario?;
    cleanup_result?;

    assert_eq!(coverage.seen.len(), 46);
    assert_eq!(coverage.seen, published_endpoints()?);
    Ok(())
}

async fn run_scenario(
    coverage: &mut Coverage,
    first: &AiChatSubmission,
) -> Result<(), Box<dyn Error>> {
    success!(
        coverage,
        Method::GET,
        "/config",
        coverage.user.configuration().get_agents_config()
    );
    let stream_input = AiChatStreamInput::new(first.message_id.clone());
    let mut stream = coverage
        .user
        .chat()
        .stream_chat_events(
            &first.conversation_id,
            &stream_input,
            &StreamChatEventsOptions::default(),
        )
        .await?;
    let mut saw_end = false;
    while let Some(event) = stream.next().await {
        saw_end |= event?.event_type() == "end";
    }
    drop(stream);
    assert!(saw_end);
    coverage.record(
        Method::POST,
        "/conversations/{conversationId}/stream",
        200,
        "流式响应完成",
    )?;

    let conversation = first.conversation_id.as_str();
    let message = first.message_id.as_str();
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/config",
        coverage
            .user
            .configuration()
            .get_conversation_config(conversation)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/context-usage",
        coverage
            .user
            .conversations()
            .get_conversation_context_usage(conversation)
    );
    let list_options = ListConversationsOptions {
        current: Some(0),
        size: Some(5),
        ..Default::default()
    };
    success!(
        coverage,
        Method::GET,
        "/conversations",
        coverage
            .user
            .conversations()
            .list_conversations(&list_options)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/active",
        coverage.user.conversations().list_active_conversations()
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/unread",
        coverage.user.conversations().list_unread_conversations()
    );
    let activity_input = ConversationActivityBatchInput::new(vec![conversation.into()]);
    success!(
        coverage,
        Method::POST,
        "/conversations/activity/query",
        coverage
            .user
            .conversations()
            .query_conversation_activities(&activity_input)
    );
    let receipt_input = ConversationReadReceiptInput::new(message.into());
    success!(
        coverage,
        Method::PUT,
        "/conversations/{conversationId}/read-receipt",
        coverage
            .user
            .conversations()
            .mark_conversation_read(conversation, &receipt_input)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/stats",
        coverage.user.conversations().get_conversation_stats()
    );
    let title_input = ConversationTitleInput::new("Rust SDK 全接口测试".into());
    success!(
        coverage,
        Method::PATCH,
        "/conversations/{conversationId}/title",
        coverage
            .user
            .conversations()
            .update_conversation_title(conversation, &title_input)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/title",
        coverage
            .user
            .conversations()
            .get_conversation_title(conversation)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/messages",
        coverage
            .user
            .messages()
            .list_conversation_messages(conversation, &ListConversationMessagesOptions::default())
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/messages/{messageId}",
        coverage
            .user
            .messages()
            .get_conversation_message(conversation, message)
    );
    let event_options = GetChatEventsOptions {
        conversation_id: conversation.into(),
        message_id: message.into(),
    };
    success!(
        coverage,
        Method::GET,
        "/events",
        coverage.user.events().get_chat_events(&event_options)
    );
    let batch_input = AiChatEventsBatchInput::new(conversation.into(), vec![message.into()]);
    success!(
        coverage,
        Method::POST,
        "/events/batch",
        coverage.user.events().get_chat_events_batch(&batch_input)
    );

    let second_input = AiChatInput::new("再次仅回复英文 OK".into());
    let second: AiChatSubmission = success!(
        coverage,
        Method::POST,
        "/conversations/{conversationId}",
        201,
        coverage
            .user
            .chat()
            .continue_chat(conversation, &second_input)
    );
    let second_stream_input = AiChatStreamInput::new(second.message_id.clone());
    let mut second_stream = coverage
        .user
        .chat()
        .stream_chat_events(
            &second.conversation_id,
            &second_stream_input,
            &StreamChatEventsOptions::default(),
        )
        .await?;
    while let Some(event) = second_stream.next().await {
        event?;
    }
    drop(second_stream);
    success!(
        coverage,
        Method::DELETE,
        "/conversations/{conversationId}/interrupt",
        coverage.user.chat().interrupt_conversation(conversation)
    );
    success!(
        coverage,
        Method::POST,
        "/conversations/{conversationId}/compact",
        coverage
            .user
            .chat()
            .compact_conversation(conversation, &CompactConversationOptions::default())
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/async-tasks",
        coverage.user.messages().list_conversation_async_tasks(
            conversation,
            &ListConversationAsyncTasksOptions::default()
        )
    );
    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/async-tasks/{asyncTaskId}",
        coverage
            .user
            .messages()
            .get_conversation_async_task(conversation, "missing-async-task")
    );
    domain!(
        coverage,
        Method::DELETE,
        "/conversations/{conversationId}/messages/{messageId}/queue",
        coverage
            .user
            .messages()
            .cancel_queued_message(conversation, message)
    );

    let share_input = ConversationShareInput::new();
    let share = success!(
        coverage,
        Method::POST,
        "/conversations/{conversationId}/shares",
        201,
        coverage
            .user
            .conversations()
            .create_conversation_share(conversation, &share_input)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/shares",
        coverage
            .user
            .conversations()
            .list_conversation_shares(conversation)
    );
    success!(
        coverage,
        Method::DELETE,
        "/conversations/{conversationId}/shares/{shareId}",
        coverage
            .user
            .conversations()
            .revoke_conversation_share(conversation, share.share_id)
    );

    let approval = PlanApprovalInput::new(conversation.into(), message.into(), false);
    success!(
        coverage,
        Method::POST,
        "/plan/approve",
        coverage.user.interactions().approve_plan(&approval)
    );
    success!(
        coverage,
        Method::GET,
        "/plan/{planId}/status",
        coverage.user.interactions().get_plan_status("missing-plan")
    );
    let user_status = GetUserInputStatusOptions {
        conversation_id: conversation.into(),
        message_id: message.into(),
    };
    success!(
        coverage,
        Method::GET,
        "/user-input/{questionId}/status",
        coverage
            .user
            .interactions()
            .get_user_input_status("missing-question", &user_status)
    );
    let mut answer = UserInputAnswerInput::new(
        conversation.into(),
        message.into(),
        "missing-question".into(),
        vec![],
    );
    answer.custom_input = Some(Some("not pending".into()));
    success!(
        coverage,
        Method::POST,
        "/user-input/answer",
        coverage.user.interactions().answer_user_input(&answer)
    );

    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/sql-query-results/{resultId}",
        coverage.user.sql().get_sql_query_result(
            conversation,
            "missing-result",
            &GetSqlQueryResultOptions::default()
        )
    );
    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/sql-query-results/{resultId}/chart-data",
        coverage
            .user
            .sql()
            .get_sql_query_chart_data(conversation, "missing-result")
    );
    let export_options = ExportSqlQueryResultOptions {
        format: SqlExportFormat::Csv,
        accept: Some("text/csv".into()),
    };
    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/sql-query-results/{resultId}/export",
        coverage.user.sql().export_sql_query_result(
            conversation,
            "missing-result",
            &export_options
        )
    );

    let md5 = "17/2WOZXDPjhZzwMQCHrDg==";
    let exists_options = FileExistsByContentMd5Options {
        content_md5: md5.into(),
    };
    success!(
        coverage,
        Method::GET,
        "/files/meta/contentMd5",
        coverage
            .user
            .files()
            .file_exists_by_content_md5(&exists_options)
    );
    let upload_input = GeneratePreSignedUrlInput::new(
        "lingya-sdk-endpoint-test.txt".into(),
        Module::AiChatAttachments,
        md5.into(),
    );
    let upload = success!(
        coverage,
        Method::POST,
        "/files/pre-signed-url/write",
        coverage
            .user
            .files()
            .create_pre_signed_upload(&upload_input)
    );
    let file_uk = upload
        .file_uk
        .flatten()
        .ok_or_else(|| LingyaError::InvalidInput("pre-signed upload omitted fileUk".into()))?;
    let confirm = ConfirmUploadInput::new(file_uk, md5.into());
    domain!(
        coverage,
        Method::POST,
        "/files/pre-signed-url/confirm",
        coverage.user.files().confirm_pre_signed_upload(&confirm)
    );
    let create_file = CreateFileInput::new("lingya-sdk-endpoint-test.txt".into(), md5.into());
    domain!(
        coverage,
        Method::POST,
        "/files/contentMd5",
        coverage
            .user
            .files()
            .create_file_by_content_md5(&create_file)
    );
    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/files/{fileId}/preview",
        coverage
            .user
            .files()
            .get_conversation_file_preview(conversation, i64::MAX)
    );
    domain!(coverage, Method::GET, "/conversations/{conversationId}/messages/{messageId}/plan-intermediate-files/{fileId}/preview", coverage.user.files().get_plan_intermediate_file_preview(conversation, message, i64::MAX));

    success!(
        coverage,
        Method::POST,
        "/knowledge-bases/citations/metadata",
        coverage.user.knowledge().get_citation_metadata_batch(&[])
    );
    domain!(
        coverage,
        Method::GET,
        "/knowledge-bases/citations/{citationType}/{referenceId}/metadata",
        coverage
            .user
            .knowledge()
            .get_citation_metadata("CHUNK", i64::MAX)
    );
    success!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/workspace/files",
        coverage
            .user
            .workspace()
            .list_workspace_artifacts(conversation, &ListWorkspaceArtifactsOptions::default())
    );
    let preview_options = GetWorkspaceFilePreviewOptions {
        path: "missing-file.txt".into(),
    };
    domain!(
        coverage,
        Method::GET,
        "/conversations/{conversationId}/workspace/files/preview",
        coverage
            .user
            .workspace()
            .get_workspace_file_preview(conversation, &preview_options)
    );

    let probe_input = ChatStreamProbeInput::new(format!("all-{}", uuid::Uuid::new_v4()));
    let mut probe = coverage
        .user
        .chat()
        .probe_event_stream(&probe_input, &ProbeEventStreamOptions::default())
        .await?;
    let mut probe_count = 0;
    while let Some(event) = probe.next().await {
        event?;
        probe_count += 1;
    }
    drop(probe);
    assert_eq!(probe_count, 4);
    coverage.record(Method::POST, "/stream-probe", 200, "流式响应完成")?;
    let status_input = ConversationStatusInput::new("ARCHIVED".into());
    success!(
        coverage,
        Method::PATCH,
        "/conversations/{conversationId}/status",
        coverage
            .user
            .conversations()
            .update_conversation_status(conversation, &status_input)
    );
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

    fn record(
        &mut self,
        method: Method,
        suffix: &str,
        status: u16,
        outcome: &str,
    ) -> Result<(), LingyaError> {
        let path = format!("{BASE_PATH}{suffix}");
        let key = format!("{} {path}", method.as_str());
        if !self.seen.insert(key.clone()) {
            return Err(LingyaError::InvalidInput(format!(
                "duplicate endpoint: {key}"
            )));
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
        let mut report = String::from("# Rust SDK 真实环境全接口测试报告\n\n报告不含凭证、请求正文或响应正文。\n\n| 请求标识 | Method | Path | HTTP | 结果 |\n|---|---|---|---:|---|\n");
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

fn env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

fn required_env(name: &str) -> Result<String, Box<dyn Error>> {
    env(name).ok_or_else(|| format!("{name} is required").into())
}
