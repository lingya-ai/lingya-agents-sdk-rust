use lingya_agents_sdk::models::generate_pre_signed_url_input::Module;
use lingya_agents_sdk::models::generate_pre_signed_url_output::UploadMethod;
use lingya_agents_sdk::models::{GeneratePreSignedUrlInput, GeneratePreSignedUrlOutput};
use serde_json::json;

#[test]
fn presign_input_serializes_size_bytes() {
    let mut input = GeneratePreSignedUrlInput::new(
        "agent-sft-overall-strategy.md".to_owned(),
        Module::AiChatAttachments,
        "BGN0aTOgnUtqM8UMJesATw==".to_owned(),
    );
    input.size_bytes = Some(Some(2048));
    let value = serde_json::to_value(&input).unwrap();
    assert_eq!(value["sizeBytes"], json!(2048));
}

#[test]
fn presign_output_restores_upload_protocol() {
    let output: GeneratePreSignedUrlOutput = serde_json::from_value(json!({
        "support": true,
        "fileUk": "file-key",
        "url": "https://objects.example.com/upload",
        "uploadMethod": "POST_FORM",
        "formFields": {"key": "uploads/agent-sft-overall-strategy.md"},
        "maximumBytes": 4096,
    }))
    .unwrap();
    assert_eq!(output.upload_method, Some(UploadMethod::PostForm));
    assert_eq!(
        output.form_fields.as_ref().unwrap()["key"],
        "uploads/agent-sft-overall-strategy.md"
    );
    assert_eq!(output.maximum_bytes, Some(Some(4096)));
}
