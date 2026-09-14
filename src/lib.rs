#![allow(unused_imports)]

//! Lingya Agents OpenAPI 的强类型服务端 SDK。
//!
//! 本 crate 只适用于可信服务端。OpenAPI secret 不得进入客户端应用、日志或异常。

pub mod client;
pub mod events;
pub mod models;
pub mod sse;

pub use client::{
    LingyaAgentsClient, LingyaAgentsUserClient, LingyaError, OpenApiCredentials, QueryParameter,
};
pub use events::{
    decode_ai_chat_brief_event, decode_tool_extension, LingyaAiChatBriefEvent, LingyaToolExtension,
};
pub use reqwest::Method as HttpMethod;
