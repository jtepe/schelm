use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Messages API role for input turns.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
}

/// Reasons generation may stop.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
    PauseTurn,
    Refusal,
}

/// Service tier selection for create message requests.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RequestServiceTier {
    Auto,
    StandardOnly,
}

/// Service tier reported in response usage.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UsageServiceTier {
    Standard,
    Priority,
    Batch,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageBatchProcessingStatus {
    InProgress,
    Canceling,
    Ended,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceType {
    Auto,
    Any,
    Tool,
    None,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CacheControlEphemeral {
    #[serde(rename = "type")]
    pub ty: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextBlockParam {
    #[serde(rename = "type")]
    pub ty: String,
    pub text: String,
    pub citations: Option<Vec<serde_json::Value>>,
    pub cache_control: Option<CacheControlEphemeral>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SystemPrompt {
    String(String),
    Blocks(Vec<TextBlockParam>),
}

/// Generic content block for message input/output.
///
/// Claude supports many evolving block kinds. This representation keeps all fields
/// while preserving the `type` discriminator.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageContent {
    String(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageParam {
    pub role: MessageRole,
    pub content: MessageContent,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolDefinition {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub ty: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ToolChoice {
    #[serde(rename = "type")]
    pub ty: ToolChoiceType,
    pub name: Option<String>,
    pub disable_parallel_tool_use: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ThinkingConfig {
    #[serde(rename = "type")]
    pub ty: String,
    pub budget_tokens: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputConfig {
    pub format: Option<serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMessageBody {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<MessageParam>,
    pub cache_control: Option<CacheControlEphemeral>,
    pub container: Option<String>,
    pub inference_geo: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub output_config: Option<OutputConfig>,
    pub service_tier: Option<RequestServiceTier>,
    pub stop_sequences: Option<Vec<String>>,
    pub stream: Option<bool>,
    pub system: Option<SystemPrompt>,
    pub temperature: Option<f32>,
    pub thinking: Option<ThinkingConfig>,
    pub tool_choice: Option<ToolChoice>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Container {
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CacheCreation {
    pub ephemeral_5m_input_tokens: Option<u64>,
    pub ephemeral_1h_input_tokens: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerToolUsage {
    pub web_search_requests: Option<u64>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation: Option<CacheCreation>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
    pub inference_geo: Option<String>,
    pub server_tool_use: Option<ServerToolUsage>,
    pub service_tier: Option<UsageServiceTier>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub role: String,
    pub content: Vec<ContentBlock>,
    pub model: String,
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub usage: Usage,
    pub container: Option<Container>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDelta {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
    pub container: Option<Container>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageDeltaUsage {
    pub output_tokens: u64,
    pub input_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
    pub server_tool_use: Option<ServerToolUsage>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorInfo {
    #[serde(rename = "type")]
    pub ty: String,
    pub message: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub ty: String,
    pub error: ErrorInfo,
    pub request_id: Option<String>,
}

/// Streaming event emitted by the Messages API SSE stream.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StreamingEvent {
    MessageStart {
        #[serde(rename = "type")]
        event_type: String,
        message: Message,
    },
    MessageDelta {
        #[serde(rename = "type")]
        event_type: String,
        delta: MessageDelta,
        usage: MessageDeltaUsage,
    },
    MessageStop {
        #[serde(rename = "type")]
        event_type: String,
    },
    ContentBlockStart {
        #[serde(rename = "type")]
        event_type: String,
        index: u64,
        content_block: ContentBlock,
    },
    ContentBlockDelta {
        #[serde(rename = "type")]
        event_type: String,
        index: u64,
        delta: ContentBlock,
    },
    ContentBlockStop {
        #[serde(rename = "type")]
        event_type: String,
        index: u64,
    },
    Ping {
        #[serde(rename = "type")]
        event_type: String,
    },
    Error {
        #[serde(rename = "type")]
        event_type: String,
        error: ErrorInfo,
    },
    Unknown(UnknownEvent),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum KnownStreamingEvent {
    MessageStart {
        message: Message,
    },
    MessageDelta {
        delta: MessageDelta,
        usage: MessageDeltaUsage,
    },
    MessageStop,
    ContentBlockStart {
        index: u64,
        content_block: ContentBlock,
    },
    ContentBlockDelta {
        index: u64,
        delta: ContentBlock,
    },
    ContentBlockStop {
        index: u64,
    },
    Ping,
    Error {
        error: ErrorInfo,
    },
}

impl From<KnownStreamingEvent> for StreamingEvent {
    fn from(value: KnownStreamingEvent) -> Self {
        match value {
            KnownStreamingEvent::MessageStart { message } => StreamingEvent::MessageStart {
                event_type: "message_start".to_string(),
                message,
            },
            KnownStreamingEvent::MessageDelta { delta, usage } => StreamingEvent::MessageDelta {
                event_type: "message_delta".to_string(),
                delta,
                usage,
            },
            KnownStreamingEvent::MessageStop => StreamingEvent::MessageStop {
                event_type: "message_stop".to_string(),
            },
            KnownStreamingEvent::ContentBlockStart {
                index,
                content_block,
            } => StreamingEvent::ContentBlockStart {
                event_type: "content_block_start".to_string(),
                index,
                content_block,
            },
            KnownStreamingEvent::ContentBlockDelta { index, delta } => {
                StreamingEvent::ContentBlockDelta {
                    event_type: "content_block_delta".to_string(),
                    index,
                    delta,
                }
            }
            KnownStreamingEvent::ContentBlockStop { index } => StreamingEvent::ContentBlockStop {
                event_type: "content_block_stop".to_string(),
                index,
            },
            KnownStreamingEvent::Ping => StreamingEvent::Ping {
                event_type: "ping".to_string(),
            },
            KnownStreamingEvent::Error { error } => StreamingEvent::Error {
                event_type: "error".to_string(),
                error,
            },
        }
    }
}

impl<'de> Deserialize<'de> for StreamingEvent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        match serde_json::from_value::<KnownStreamingEvent>(value.clone()) {
            Ok(known) => Ok(known.into()),
            Err(known_err) => {
                let event_type = value.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if is_known_event_type(event_type) {
                    Err(serde::de::Error::custom(known_err))
                } else {
                    serde_json::from_value::<UnknownEvent>(value)
                        .map(StreamingEvent::Unknown)
                        .map_err(serde::de::Error::custom)
                }
            }
        }
    }
}

fn is_known_event_type(ty: &str) -> bool {
    matches!(
        ty,
        "message_start"
            | "message_delta"
            | "message_stop"
            | "content_block_start"
            | "content_block_delta"
            | "content_block_stop"
            | "ping"
            | "error"
    )
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UnknownEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(flatten)]
    pub payload: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageBatchRequestCounts {
    pub canceled: u64,
    pub errored: u64,
    pub expired: u64,
    pub processing: u64,
    pub succeeded: u64,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageBatch {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub created_at: String,
    pub expires_at: String,
    pub processing_status: MessageBatchProcessingStatus,
    pub request_counts: MessageBatchRequestCounts,
    pub archived_at: Option<String>,
    pub cancel_initiated_at: Option<String>,
    pub ended_at: Option<String>,
    pub results_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageBatchItem {
    pub custom_id: String,
    pub params: CreateMessageBody,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMessageBatchBody {
    pub requests: Vec<MessageBatchItem>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListMessageBatchesQuery {
    pub before_id: Option<String>,
    pub after_id: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageBatchList {
    pub data: Vec<MessageBatch>,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageBatchResult {
    Succeeded { message: Box<Message> },
    Errored { error: ErrorResponse },
    Canceled,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageBatchIndividualResponse {
    pub custom_id: String,
    pub result: MessageBatchResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeletedMessageBatch {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CountTokensBody {
    pub model: String,
    pub messages: Vec<MessageParam>,
    pub cache_control: Option<CacheControlEphemeral>,
    pub output_config: Option<OutputConfig>,
    pub system: Option<SystemPrompt>,
    pub thinking: Option<ThinkingConfig>,
    pub tool_choice: Option<ToolChoice>,
    pub tools: Option<Vec<ToolDefinition>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageTokensCount {
    pub input_tokens: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_message() -> Message {
        Message {
            id: "msg_123".to_string(),
            ty: "message".to_string(),
            role: "assistant".to_string(),
            content: vec![ContentBlock {
                ty: "text".to_string(),
                data: serde_json::Map::from_iter([(
                    "text".to_string(),
                    serde_json::Value::String("Hello".to_string()),
                )]),
            }],
            model: "claude-sonnet-4-5".to_string(),
            stop_reason: Some(StopReason::EndTurn),
            stop_sequence: None,
            usage: Usage {
                input_tokens: 10,
                output_tokens: 4,
                cache_creation: None,
                cache_creation_input_tokens: None,
                cache_read_input_tokens: None,
                inference_geo: None,
                server_tool_use: None,
                service_tier: Some(UsageServiceTier::Standard),
            },
            container: None,
        }
    }

    #[test]
    fn test_skip_serializing_none_in_create_message_body() {
        let body = CreateMessageBody {
            model: "claude-sonnet-4-5".into(),
            max_tokens: 256,
            messages: vec![MessageParam {
                role: MessageRole::User,
                content: MessageContent::String("hi".into()),
            }],
            cache_control: None,
            container: None,
            inference_geo: None,
            metadata: None,
            output_config: None,
            service_tier: None,
            stop_sequences: None,
            stream: None,
            system: None,
            temperature: None,
            thinking: None,
            tool_choice: None,
            tools: None,
            top_k: None,
            top_p: None,
        };
        let json = serde_json::to_string_pretty(&body).unwrap();
        assert!(!json.contains("\"stream\""));
        assert!(!json.contains("\"system\""));
        assert!(!json.contains("\"tools\""));
    }

    #[test]
    fn stream_known_event_deserializes() {
        let json = serde_json::json!({
            "type": "message_delta",
            "delta": {
                "stop_reason": "end_turn"
            },
            "usage": {
                "output_tokens": 12
            }
        });
        let event: StreamingEvent = serde_json::from_value(json).unwrap();
        match event {
            StreamingEvent::MessageDelta { delta, usage, .. } => {
                assert_eq!(delta.stop_reason, Some(StopReason::EndTurn));
                assert_eq!(usage.output_tokens, 12);
            }
            other => panic!("expected MessageDelta, got: {other:?}"),
        }
    }

    #[test]
    fn stream_unknown_event_deserializes() {
        let json = r#"{"type":"future_event","x":1}"#;
        let event: StreamingEvent = serde_json::from_str(json).unwrap();
        match event {
            StreamingEvent::Unknown(u) => {
                assert_eq!(u.event_type, "future_event");
                assert_eq!(u.payload.get("x").unwrap(), &serde_json::json!(1));
            }
            other => panic!("expected Unknown, got: {other:?}"),
        }
    }

    #[test]
    fn stream_known_event_missing_fields_errors() {
        let json = serde_json::json!({
            "type": "content_block_start",
            "index": 0
        });
        let result = serde_json::from_value::<StreamingEvent>(json);
        assert!(result.is_err());
    }

    #[test]
    fn message_batch_result_succeeded_deserializes() {
        let json = serde_json::json!({
            "custom_id": "req-1",
            "result": {
                "type": "succeeded",
                "message": Box::new(minimal_message())
            }
        });
        let line: MessageBatchIndividualResponse = serde_json::from_value(json).unwrap();
        assert_eq!(line.custom_id, "req-1");
        assert!(matches!(line.result, MessageBatchResult::Succeeded { .. }));
    }

    #[test]
    fn message_batch_result_errored_deserializes() {
        let json = serde_json::json!({
            "custom_id": "req-2",
            "result": {
                "type": "errored",
                "error": {
                    "type": "error",
                    "error": {
                        "type": "invalid_request_error",
                        "message": "bad request"
                    }
                }
            }
        });
        let line: MessageBatchIndividualResponse = serde_json::from_value(json).unwrap();
        match line.result {
            MessageBatchResult::Errored { error } => {
                assert_eq!(error.ty, "error");
                assert_eq!(error.error.ty, "invalid_request_error");
            }
            other => panic!("expected Errored, got: {other:?}"),
        }
    }

    #[test]
    fn count_tokens_response_deserializes() {
        let json = r#"{"input_tokens":1234}"#;
        let count: MessageTokensCount = serde_json::from_str(json).unwrap();
        assert_eq!(count.input_tokens, 1234);
    }
}
