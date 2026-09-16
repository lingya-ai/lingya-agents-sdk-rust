#![allow(unused_imports)]

//! Lingya Agents OpenAPI 的强类型服务端 SDK。
//!
//! 本 crate 只适用于可信服务端。OpenAPI secret 不得进入客户端应用、日志或异常。

pub mod bound_api;
pub mod client;
pub mod events;
pub mod models;
pub mod sse;

pub use bound_api::*;
#[deprecated(note = "use AgentsClient")]
pub use client::LingyaAgentsClient;
#[deprecated(note = "use AgentsUserClient")]
pub use client::LingyaAgentsUserClient;
#[deprecated(note = "use ApiError")]
pub use client::LingyaError;
pub use client::{AgentsClient, AgentsUserClient, ApiError, OpenApiCredentials, QueryParameter};
#[deprecated(note = "use AiChatBriefEvent")]
pub use events::LingyaAiChatBriefEvent;
#[deprecated(note = "use ToolExtension")]
pub use events::LingyaToolExtension;
pub use events::{
    decode_ai_chat_brief_event, decode_tool_extension, AiChatBriefEvent, ToolExtension,
};
pub use reqwest::Method as HttpMethod;
