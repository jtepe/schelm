use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DetailEnum {
    /// Choose the detail level automatically.
    Auto,
    /// Allows the model to "see" a higher-resolution version of the image, usually increasing input token costs.
    High,
    /// Restricts the model to a lower-resolution version of the image.
    Low,
}

pub type ImageDetail = DetailEnum;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    /// End-user input in the conversation.
    User,
    /// Model-generated content in the conversation.
    Assistant,
    /// System-level instructions that set global behavior.
    System,
    /// Developer-supplied guidance that shapes the assistant’s behavior.
    Developer,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    /// Model is currently sampling this item.
    InProgress,
    /// Model has finished sampling this item.
    Completed,
    /// Model was interrupted from sampling this item partway through.
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallStatus {
    /// Model is currently sampling this item.
    InProgress,
    /// Model has finished sampling this item.
    Completed,
    /// Model was interrupted from sampling this item partway through.
    Incomplete,
}

pub type FunctionCallItemStatus = FunctionCallStatus;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallOutputStatusEnum {
    InProgress,
    Completed,
    Incomplete,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncludeEnum {
    /// includes encrypted reasoning content so that it may be rehydrated on a subsequent request.
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    /// includes sampled logprobs in assistant messages.
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceValueEnum {
    /// Restrict the model from calling any tools.
    None,
    /// Let the model choose the tools from among the provided set.
    Auto,
    /// Require the model to call a tool.
    Required,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerbosityEnum {
    /// Instruct the model to emit less verbose final responses.
    Low,
    /// Use the model's default verbosity setting.
    Medium,
    /// Instruct the model to emit more verbose final responses.
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffortEnum {
    /// Restrict the model from performing any reasoning before emitting a final answer.
    None,
    /// Use a lower reasoning effort for faster responses.
    Low,
    /// Use a balanced reasoning effort.
    Medium,
    /// Use a higher reasoning effort to improve answer quality.
    High,
    /// Use the maximum reasoning effort available.
    Xhigh,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningSummaryEnum {
    /// Emit concise summaries of reasoning content.
    Concise,
    /// Emit details summaries of reasoning content.
    Detailed,
    /// Allow the model to decide when to summarize.
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TruncationEnum {
    /// Let the service decide how to truncate.
    Auto,
    /// Disable service truncation. Context over the model's context limit will result in a 400 error.
    Disabled,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTierEnum {
    /// Choose a service tier automatically based on current account state.
    Auto,
    /// Choose the default service tier.
    Default,
    /// Choose the flex service tier.
    Flex,
    /// Choose the priority service tier.
    Priority,
}

/// An internal identifier for an item to reference.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemReferenceParam {
    /// The ID of the item to reference.
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningSummaryContentParam {
    /// The content type. Always `summary_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The reasoning summary text.
    pub text: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItemParam {
    /// The unique ID of this reasoning item.
    pub id: Option<String>,
    /// Reasoning summary content associated with this item.
    pub summary: Vec<ReasoningSummaryContentParam>,
    pub content: Option<serde_json::Value>,
    /// An encrypted representation of the reasoning content.
    pub encrypted_content: Option<String>,
}

/// A text input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTextContentParam {
    /// The text input to the model.
    pub text: String,
}

/// An image input to the model. Learn about [image inputs](/docs/guides/vision)
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImageContentParamAutoParam {
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    pub image_url: Option<String>,
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    pub detail: Option<ImageDetail>,
}

/// A file input to the model.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFileContentParam {
    /// The name of the file to be sent to the model.
    pub filename: Option<String>,
    /// The base64-encoded data of the file to be sent to the model.
    pub file_data: Option<String>,
    /// The URL of the file to be sent to the model.
    pub file_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserMessageContentPart {
    InputText(InputTextContentParam),
    InputImage(InputImageContentParamAutoParam),
    InputFile(InputFileContentParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum UserMessageContent {
    /// A piece of message content, such as text, an image, or a file.
    Array(Vec<UserMessageContentPart>),
    /// The message content, as a single string.
    String(String),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserMessageItemParam {
    /// The unique ID of this message item.
    pub id: Option<String>,
    /// The message role. Always `user`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: UserMessageContent,
    /// The status of the message item.
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SystemMessageContent {
    Array(Vec<InputTextContentParam>),
    /// The message content, as a single string.
    String(String),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SystemMessageItemParam {
    /// The unique ID of this message item.
    pub id: Option<String>,
    /// The message role. Always `system`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: SystemMessageContent,
    /// The status of the message item.
    pub status: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeveloperMessageItemParam {
    /// The unique ID of this message item.
    pub id: Option<String>,
    /// The message role. Always `developer`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: SystemMessageContent,
    /// The status of the message item.
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UrlCitationParam {
    /// The citation type. Always `url_citation`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The index of the first character of the citation in the message.
    pub start_index: i32,
    /// The index of the last character of the citation in the message.
    pub end_index: i32,
    /// The URL of the cited resource.
    pub url: String,
    /// The title of the cited resource.
    pub title: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTextContentParam {
    /// The text content.
    pub text: String,
    /// Citations associated with the text content.
    pub annotations: Option<Vec<UrlCitationParam>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RefusalContentParam {
    /// The refusal text.
    pub refusal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssistantMessageContentPart {
    OutputText(OutputTextContentParam),
    Refusal(RefusalContentParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum AssistantMessageContent {
    /// A piece of assistant message content, such as text or a refusal.
    Array(Vec<AssistantMessageContentPart>),
    /// The message content, as a single string.
    String(String),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantMessageItemParam {
    /// The unique ID of this message item.
    pub id: Option<String>,
    /// The role of the message author. Always `assistant`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: AssistantMessageContent,
    /// The status of the message item.
    pub status: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallItemParam {
    /// The unique ID of this function tool call.
    pub id: Option<String>,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to call.
    pub name: String,
    /// The function arguments as a JSON string.
    pub arguments: String,
    /// The status of the function tool call.
    pub status: Option<FunctionCallStatus>,
}

/// A content block representing a video input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputVideoContent {
    /// A base64 or remote url that resolves to a video file.
    pub video_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FunctionCallOutputPart {
    InputText(InputTextContentParam),
    InputImage(InputImageContentParamAutoParam),
    InputFile(InputFileContentParam),
    InputVideo(InputVideoContent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum FunctionCallOutput {
    /// A JSON string of the output of the function tool call.
    String(String),
    /// An array of content outputs (text, image, file) for the function tool call.
    Array(Vec<FunctionCallOutputPart>),
}

/// The output of a function tool call.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallOutputItemParam {
    /// The unique ID of the function tool call output. Populated when this item is returned via API.
    pub id: Option<String>,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// Text, image, or file output of the function tool call.
    pub output: FunctionCallOutput,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    pub status: Option<FunctionCallStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemParam {
    ItemReference(ItemReferenceParam),
    Reasoning(ReasoningItemParam),
    Message(MessageItemParam),
    FunctionCall(FunctionCallItemParam),
    FunctionCallOutput(FunctionCallOutputItemParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MessageItemParam {
    User(UserMessageItemParam),
    System(SystemMessageItemParam),
    Developer(DeveloperMessageItemParam),
    Assistant(AssistantMessageItemParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EmptyModelParam {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolParam {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub strict: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponsesToolParam {
    Function(FunctionToolParam),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SpecificFunctionParam {
    /// The tool to call. Always `function`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The name of the function tool to call.
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SpecificToolChoiceParam {
    SpecificFunction(SpecificFunctionParam),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AllowedToolsParam {
    /// The tool choice type. Always `allowed_tools`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The list of tools that are permitted for this request.
    pub tools: Vec<SpecificToolChoiceParam>,
    /// How to select a tool from the allowed set.
    pub mode: Option<ToolChoiceValueEnum>,
}

/// Controls which tool the model should use, if any.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ToolChoiceParam {
    SpecificToolChoice(SpecificToolChoiceParam),
    ToolChoiceValue(ToolChoiceValueEnum),
    AllowedTools(AllowedToolsParam),
}

/// Set of 16 key-value pairs that can be attached to an object.
pub type MetadataParam = HashMap<String, String>;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextParam {
    /// The format configuration for text output.
    pub format: Option<serde_json::Value>,
    /// Controls the level of detail in generated text output.
    pub verbosity: Option<VerbosityEnum>,
}

/// Options that control streamed response behavior.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StreamOptionsParam {
    /// Whether to obfuscate sensitive information in streamed output. Defaults to `true`.
    pub include_obfuscation: Option<bool>,
}

/// **gpt-5 and o-series models only** Configuration options for [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningParam {
    /// Controls the level of reasoning effort the model should apply. Higher effort may increase latency and cost.
    pub effort: Option<ReasoningEffortEnum>,
    /// Controls whether the response includes a reasoning summary.
    pub summary: Option<ReasoningSummaryEnum>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateResponseBody {
    /// The model to use for this request, e.g. 'gpt-5.2'.
    pub model: Option<String>,
    /// Context to provide to the model for the scope of this request. May either be a string or an array of input items. If a string is provided, it is interpreted as a user message.
    pub input: Option<CreateResponseInput>,
    /// The ID of the response to use as the prior turn for this request.
    pub previous_response_id: Option<String>,
    pub include: Option<Vec<IncludeEnum>>,
    /// A list of tools that the model may call while generating the response.
    pub tools: Option<Vec<ResponsesToolParam>>,
    /// Controls which tool the model should use, if any.
    pub tool_choice: Option<ToolChoiceParam>,
    /// Set of 16 key-value pairs that can be attached to an object.
    pub metadata: Option<MetadataParam>,
    /// Configuration options for text output.
    pub text: Option<TextParam>,
    /// Sampling temperature to use, between 0 and 2. Higher values make the output more random.
    pub temperature: Option<f64>,
    /// Nucleus sampling parameter, between 0 and 1. The model considers only the tokens with the top cumulative probability.
    pub top_p: Option<f64>,
    /// Penalizes new tokens based on whether they appear in the text so far.
    pub presence_penalty: Option<f64>,
    /// Penalizes new tokens based on their frequency in the text so far.
    pub frequency_penalty: Option<f64>,
    /// Whether the model may call multiple tools in parallel.
    pub parallel_tool_calls: Option<bool>,
    /// Whether to stream response events as server-sent events.
    pub stream: Option<bool>,
    /// Options that control streamed response behavior.
    pub stream_options: Option<StreamOptionsParam>,
    /// Whether to run the request in the background and return immediately.
    pub background: Option<bool>,
    /// The maximum number of tokens the model may generate for this response.
    pub max_output_tokens: Option<i32>,
    /// The maximum number of tool calls the model may make while generating the response.
    pub max_tool_calls: Option<i32>,
    /// Configuration options for reasoning behavior.
    pub reasoning: Option<ReasoningParam>,
    /// A stable identifier used for safety monitoring and abuse detection.
    pub safety_identifier: Option<String>,
    /// A key to use when reading from or writing to the prompt cache.
    pub prompt_cache_key: Option<String>,
    /// Controls how the service truncates the input when it exceeds the model context window.
    pub truncation: Option<TruncationEnum>,
    /// Additional instructions to guide the model for this request.
    pub instructions: Option<String>,
    /// Whether to store the response so it can be retrieved later.
    pub store: Option<bool>,
    /// The service tier to use for this request.
    pub service_tier: Option<ServiceTierEnum>,
    /// The number of most likely tokens to return at each position, along with their log probabilities.
    pub top_logprobs: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CreateResponseInput {
    String(String),
    Array(Vec<ItemParam>),
}

/// Details about why the response was incomplete.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IncompleteDetails {
    /// The reason the response could not be completed.
    pub reason: String,
}

/// A text input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTextContent {
    /// The text input to the model.
    pub text: String,
}

/// A citation for a web resource used to generate a model response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UrlCitationBody {
    /// The URL of the web resource.
    pub url: String,
    /// The index of the first character of the URL citation in the message.
    pub start_index: i32,
    /// The index of the last character of the URL citation in the message.
    pub end_index: i32,
    /// The title of the web resource.
    pub title: String,
}

/// An annotation that applies to a span of output text.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    UrlCitation(UrlCitationBody),
}

/// The top log probability of a token.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<i32>,
}

/// The log probability of a token.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<i32>,
    pub top_logprobs: Vec<TopLogProb>,
}

/// A text output from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTextContent {
    /// The text output from the model.
    pub text: String,
    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,
    pub logprobs: Vec<LogProb>,
}

/// A text content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextContent {
    pub text: String,
}

/// A summary text from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummaryTextContent {
    /// A summary of the reasoning output from the model so far.
    pub text: String,
}

/// Reasoning text from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningTextContent {
    /// The reasoning text from the model.
    pub text: String,
}

/// A refusal from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RefusalContent {
    /// The refusal explanation from the model.
    pub refusal: String,
}

/// An image input to the model. Learn about [image inputs](/docs/guides/vision).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImageContent {
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    pub image_url: Option<String>,
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    pub detail: ImageDetail,
}

/// A file input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFileContent {
    /// The name of the file to be sent to the model.
    pub filename: String,
    /// The URL of the file to be sent to the model.
    pub file_url: String,
}

/// A message to or from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// The unique ID of the message.
    pub id: String,
    /// The status of item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    pub status: MessageStatus,
    /// The role of the message. One of `unknown`, `user`, `assistant`, `system`, `critic`, `discriminator`, `developer`, or `tool`.
    pub role: MessageRole,
    /// The content of the message
    pub content: Vec<MessageContentPart>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContentPart {
    InputText(InputTextContent),
    OutputText(OutputTextContent),
    Text(TextContent),
    SummaryText(SummaryTextContent),
    ReasoningText(ReasoningTextContent),
    Refusal(RefusalContent),
    InputImage(InputImageContent),
    InputFile(InputFileContent),
    InputVideo(InputVideoContent),
}

/// A function tool call that was generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCall {
    /// The unique ID of the function call item.
    pub id: String,
    /// The unique ID of the function tool call that was generated.
    pub call_id: String,
    /// The name of the function that was called.
    pub name: String,
    /// The arguments JSON string that was generated.
    pub arguments: String,
    /// The status of the function call item that was recorded.
    pub status: FunctionCallStatus,
}

/// A function tool call output that was returned by the tool.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallOutputResource {
    /// The unique ID of the function tool call output. Populated when this item is returned via API.
    pub id: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// Text, image, or file output of the function tool call.
    pub output: FunctionCallOutput,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    pub status: FunctionCallOutputStatusEnum,
}

/// A reasoning item that was generated by the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningBody {
    /// The unique ID of the reasoning item.
    pub id: String,
    /// The reasoning content that was generated.
    pub content: Option<Vec<MessageContentPart>>,
    /// The reasoning summary content that was generated.
    pub summary: Vec<MessageContentPart>,
    /// The encrypted reasoning content that was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

/// An item representing a message, tool call, tool output, reasoning, or other response element.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemField {
    Message(Message),
    FunctionCall(FunctionCall),
    FunctionCallOutput(FunctionCallOutputResource),
    Reasoning(ReasoningBody),
}

/// An error that occurred while generating the response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Error {
    /// A machine-readable error code that was returned.
    pub code: String,
    /// A human-readable description of the error that was returned.
    pub message: String,
}

/// Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionTool {
    /// The name of the function to call.
    pub name: String,
    /// A description of the function. Used by the model to determine whether or not to call the function.
    pub description: Option<String>,
    /// A JSON schema object describing the parameters of the function.
    pub parameters: serde_json::Value,
    /// Whether to enforce strict parameter validation. Default `true`.
    pub strict: bool,
}

/// A tool that can be used to generate a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    Function(FunctionTool),
}

/// Token usage statistics that were recorded for the response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Usage {
    /// The number of input tokens that were used to generate the response.
    pub input_tokens: i32,
    /// The number of output tokens that were generated by the model.
    pub output_tokens: i32,
    /// The total number of tokens that were used.
    pub total_tokens: i32,
    /// A breakdown of input token usage that was recorded.
    pub input_tokens_details: InputTokensDetails,
    /// A breakdown of output token usage that was recorded.
    pub output_tokens_details: OutputTokensDetails,
}

/// A breakdown of input token usage that was recorded.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTokensDetails {
    /// The number of input tokens that were served from cache.
    pub cached_tokens: i32,
}

/// A breakdown of output token usage that was recorded.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTokensDetails {
    /// The number of output tokens that were attributed to reasoning.
    pub reasoning_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextField {
    /// The format configuration for text output that was used.
    pub format: serde_json::Value,
    /// Configuration options for text output that were used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityEnum>,
}

/// Reasoning configuration and metadata that were used for the response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Reasoning {
    /// The reasoning effort that was requested for the model, if specified.
    pub effort: Option<ReasoningEffortEnum>,
    /// A model-generated summary of its reasoning that was produced, if available.
    pub summary: Option<ReasoningSummaryEnum>,
}

/// The complete response object that was returned by the Responses API.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseResource {
    /// The unique ID of the response that was created.
    pub id: String,
    /// The object type, which was always `response`.
    pub object: String,
    /// The Unix timestamp (in seconds) for when the response was created.
    pub created_at: i64,
    /// The Unix timestamp (in seconds) for when the response was completed, if it was completed.
    pub completed_at: Option<i64>,
    /// The status that was set for the response.
    pub status: String,
    /// Details about why the response was incomplete, if applicable.
    pub incomplete_details: Option<IncompleteDetails>,
    /// The model that generated this response.
    pub model: String,
    /// The ID of the previous response in the chain that was referenced, if any.
    pub previous_response_id: Option<String>,
    /// Additional instructions that were used to guide the model for this response.
    pub instructions: Option<String>,
    /// The output items that were generated by the model.
    pub output: Vec<ItemField>,
    /// The error that occurred, if the response failed.
    pub error: Option<Error>,
    /// The tools that were available to the model during response generation.
    pub tools: Vec<Tool>,
    pub tool_choice: Option<serde_json::Value>,
    /// How the input was truncated by the service when it exceeded the model context window.
    pub truncation: TruncationEnum,
    /// Whether the model was allowed to call multiple tools in parallel.
    pub parallel_tool_calls: bool,
    /// Configuration options for text output that were used.
    pub text: TextField,
    /// The nucleus sampling parameter that was used for this response.
    pub top_p: f64,
    /// The presence penalty that was used to penalize new tokens based on whether they appear in the text so far.
    pub presence_penalty: f64,
    /// The frequency penalty that was used to penalize new tokens based on their frequency in the text so far.
    pub frequency_penalty: f64,
    /// The number of most likely tokens that were returned at each position, along with their log probabilities.
    pub top_logprobs: i32,
    /// The sampling temperature that was used for this response.
    pub temperature: f64,
    /// Reasoning configuration and outputs that were produced for this response.
    pub reasoning: Option<Reasoning>,
    /// Token usage statistics that were recorded for the response, if available.
    pub usage: Option<Usage>,
    /// The maximum number of tokens the model was allowed to generate for this response.
    pub max_output_tokens: Option<i32>,
    /// The maximum number of tool calls the model was allowed to make while generating the response.
    pub max_tool_calls: Option<i32>,
    /// Whether this response was stored so it can be retrieved later.
    pub store: bool,
    /// Whether this request was run in the background.
    pub background: bool,
    /// The service tier that was used for this response.
    pub service_tier: String,
    /// Developer-defined metadata that was associated with the response.
    pub metadata: serde_json::Value,
    /// A stable identifier that was used for safety monitoring and abuse detection.
    pub safety_identifier: Option<String>,
    /// A key that was used to read from or write to the prompt cache.
    pub prompt_cache_key: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamingEvent {
    /// A streaming event that indicated the response was created.
    #[serde(rename = "response.created")]
    ResponseCreated {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated the response was queued.
    #[serde(rename = "response.queued")]
    ResponseQueued {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated the response was in progress.
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated the response was completed.
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated the response had failed.
    #[serde(rename = "response.failed")]
    ResponseFailed {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated the response was incomplete.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The response snapshot that was emitted with the event.
        response: ResponseResource,
    },
    /// A streaming event that indicated an output item was added to the response.
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The index of the output item that was added.
        output_index: i32,
        /// An item representing a message, tool call, tool output, reasoning, or other response element.
        item: Option<ItemField>,
    },
    /// A streaming event that indicated an output item was completed.
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The index of the output item that was completed.
        output_index: i32,
        /// An item representing a message, tool call, tool output, reasoning, or other response element.
        item: Option<ItemField>,
    },
    /// A streaming event that indicated a content part was added.
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was added.
        content_index: i32,
        /// A content part that makes up an input or output item.
        part: MessageContentPart,
    },
    /// A streaming event that indicated a content part was completed.
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was completed.
        content_index: i32,
        /// A content part that makes up an input or output item.
        part: MessageContentPart,
    },
    /// A streaming event that indicated output text was incrementally added.
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was updated.
        content_index: i32,
        /// The text delta that was appended.
        delta: String,
        /// The token log probabilities that were emitted with the delta, if any.
        logprobs: Vec<LogProb>,
        /// An obfuscation string that was added to pad the event payload.
        #[serde(skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    /// A streaming event that indicated output text was completed.
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was completed.
        content_index: i32,
        /// The final text that was emitted.
        text: String,
        /// The token log probabilities that were emitted with the final text, if any.
        logprobs: Vec<LogProb>,
    },
    /// A streaming event that indicated a reasoning summary part was added.
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the summary part that was added.
        summary_index: i32,
        /// A summary content part that was added.
        part: MessageContentPart,
    },
    /// A streaming event that indicated a reasoning summary part was completed.
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the summary part that was completed.
        summary_index: i32,
        /// A summary content part that was completed.
        part: MessageContentPart,
    },
    /// A streaming event that indicated refusal text was incrementally added.
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was updated.
        content_index: i32,
        /// The refusal text delta that was appended.
        delta: String,
    },
    /// A streaming event that indicated refusal text was completed.
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was completed.
        content_index: i32,
        /// The final refusal text that was emitted.
        refusal: String,
    },
    /// A streaming event that indicated reasoning text was incrementally added.
    #[serde(rename = "response.reasoning.delta")]
    ResponseReasoningDelta {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was updated.
        content_index: i32,
        /// The reasoning text delta that was appended.
        delta: String,
    },
    /// A streaming event that indicated reasoning text was completed.
    #[serde(rename = "response.reasoning.done")]
    ResponseReasoningDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was completed.
        content_index: i32,
        /// The final reasoning text that was emitted.
        text: String,
    },
    /// A streaming event that indicated reasoning summary text was incrementally added.
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryDelta {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the summary content that was updated.
        summary_index: i32,
        /// The reasoning summary text delta that was appended.
        delta: String,
    },
    /// A streaming event that indicated reasoning summary text was completed.
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the summary content that was completed.
        summary_index: i32,
        /// The final reasoning summary text that was emitted.
        text: String,
    },
    /// A streaming event that indicated an output text annotation was added.
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The index of the content part that was updated.
        content_index: i32,
        /// The index of the annotation that was added.
        annotation_index: i32,
        /// The annotation that was added.
        annotation: Annotation,
    },
    /// A streaming event that indicated function call arguments were incrementally added.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The function call arguments delta that was appended.
        delta: String,
    },
    /// A streaming event that indicated function call arguments were completed.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The ID of the item that was updated.
        item_id: String,
        /// The index of the output item that was updated.
        output_index: i32,
        /// The final function call arguments that were emitted.
        arguments: String,
    },
    /// A streaming event that indicated an error was emitted.
    #[serde(rename = "error")]
    Error {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The error payload that was emitted.
        error: ErrorPayload,
    },

    /// A streaming event with an unrecognized type value.
    ///
    /// Acts as a catch-all for forward compatibility when the server sends
    /// event types this SDK version does not know about.
    #[serde(untagged)]
    Unknown(UnknownEvent),
}

/// Private helper enum: all known variants only (no Unknown fallback).
///
/// Used by the custom `Deserialize` impl on `StreamingEvent` to distinguish
/// "truly unknown type" (→ `Unknown`) from "known type with bad fields" (→ error).
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum KnownStreamingEvent {
    #[serde(rename = "response.created")]
    ResponseCreated {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.queued")]
    ResponseQueued {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.failed")]
    ResponseFailed {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        sequence_number: i32,
        response: ResponseResource,
    },
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded {
        sequence_number: i32,
        output_index: i32,
        item: Option<ItemField>,
    },
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone {
        sequence_number: i32,
        output_index: i32,
        item: Option<ItemField>,
    },
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContentPart,
    },
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        part: MessageContentPart,
    },
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
        logprobs: Vec<LogProb>,
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
        logprobs: Vec<LogProb>,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: MessageContentPart,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        part: MessageContentPart,
    },
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
    },
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        refusal: String,
    },
    #[serde(rename = "response.reasoning.delta")]
    ResponseReasoningDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        delta: String,
    },
    #[serde(rename = "response.reasoning.done")]
    ResponseReasoningDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        text: String,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        delta: String,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        summary_index: i32,
        text: String,
    },
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        content_index: i32,
        annotation_index: i32,
        annotation: Annotation,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        delta: String,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone {
        sequence_number: i32,
        item_id: String,
        output_index: i32,
        arguments: String,
    },
    #[serde(rename = "error")]
    Error {
        sequence_number: i32,
        error: ErrorPayload,
    },
}

impl From<KnownStreamingEvent> for StreamingEvent {
    fn from(known: KnownStreamingEvent) -> Self {
        match known {
            KnownStreamingEvent::ResponseCreated {
                sequence_number,
                response,
            } => StreamingEvent::ResponseCreated {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseQueued {
                sequence_number,
                response,
            } => StreamingEvent::ResponseQueued {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseInProgress {
                sequence_number,
                response,
            } => StreamingEvent::ResponseInProgress {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseCompleted {
                sequence_number,
                response,
            } => StreamingEvent::ResponseCompleted {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseFailed {
                sequence_number,
                response,
            } => StreamingEvent::ResponseFailed {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseIncomplete {
                sequence_number,
                response,
            } => StreamingEvent::ResponseIncomplete {
                sequence_number,
                response,
            },
            KnownStreamingEvent::ResponseOutputItemAdded {
                sequence_number,
                output_index,
                item,
            } => StreamingEvent::ResponseOutputItemAdded {
                sequence_number,
                output_index,
                item,
            },
            KnownStreamingEvent::ResponseOutputItemDone {
                sequence_number,
                output_index,
                item,
            } => StreamingEvent::ResponseOutputItemDone {
                sequence_number,
                output_index,
                item,
            },
            KnownStreamingEvent::ResponseContentPartAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            } => StreamingEvent::ResponseContentPartAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            },
            KnownStreamingEvent::ResponseContentPartDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            } => StreamingEvent::ResponseContentPartDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                part,
            },
            KnownStreamingEvent::ResponseOutputTextDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                logprobs,
                obfuscation,
            } => StreamingEvent::ResponseOutputTextDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
                logprobs,
                obfuscation,
            },
            KnownStreamingEvent::ResponseOutputTextDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
                logprobs,
            } => StreamingEvent::ResponseOutputTextDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
                logprobs,
            },
            KnownStreamingEvent::ResponseReasoningSummaryPartAdded {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            } => StreamingEvent::ResponseReasoningSummaryPartAdded {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            },
            KnownStreamingEvent::ResponseReasoningSummaryPartDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            } => StreamingEvent::ResponseReasoningSummaryPartDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                part,
            },
            KnownStreamingEvent::ResponseRefusalDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            } => StreamingEvent::ResponseRefusalDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            },
            KnownStreamingEvent::ResponseRefusalDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                refusal,
            } => StreamingEvent::ResponseRefusalDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                refusal,
            },
            KnownStreamingEvent::ResponseReasoningDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            } => StreamingEvent::ResponseReasoningDelta {
                sequence_number,
                item_id,
                output_index,
                content_index,
                delta,
            },
            KnownStreamingEvent::ResponseReasoningDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
            } => StreamingEvent::ResponseReasoningDone {
                sequence_number,
                item_id,
                output_index,
                content_index,
                text,
            },
            KnownStreamingEvent::ResponseReasoningSummaryDelta {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                delta,
            } => StreamingEvent::ResponseReasoningSummaryDelta {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                delta,
            },
            KnownStreamingEvent::ResponseReasoningSummaryDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                text,
            } => StreamingEvent::ResponseReasoningSummaryDone {
                sequence_number,
                item_id,
                output_index,
                summary_index,
                text,
            },
            KnownStreamingEvent::ResponseOutputTextAnnotationAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                annotation_index,
                annotation,
            } => StreamingEvent::ResponseOutputTextAnnotationAdded {
                sequence_number,
                item_id,
                output_index,
                content_index,
                annotation_index,
                annotation,
            },
            KnownStreamingEvent::ResponseFunctionCallArgumentsDelta {
                sequence_number,
                item_id,
                output_index,
                delta,
            } => StreamingEvent::ResponseFunctionCallArgumentsDelta {
                sequence_number,
                item_id,
                output_index,
                delta,
            },
            KnownStreamingEvent::ResponseFunctionCallArgumentsDone {
                sequence_number,
                item_id,
                output_index,
                arguments,
            } => StreamingEvent::ResponseFunctionCallArgumentsDone {
                sequence_number,
                item_id,
                output_index,
                arguments,
            },
            KnownStreamingEvent::Error {
                sequence_number,
                error,
            } => StreamingEvent::Error {
                sequence_number,
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
                // Determine whether this is a truly unknown type or a known type
                // that failed field validation.
                let event_type = value.get("type").and_then(|v| v.as_str()).unwrap_or("");

                if is_known_event_type(event_type) {
                    // Known type, but required fields are missing/wrong — real error.
                    Err(serde::de::Error::custom(known_err))
                } else {
                    // Truly unknown type — forward-compatible fallback.
                    serde_json::from_value::<UnknownEvent>(value)
                        .map(StreamingEvent::Unknown)
                        .map_err(serde::de::Error::custom)
                }
            }
        }
    }
}

/// Returns `true` if `ty` matches a type string handled by a known `StreamingEvent` variant.
fn is_known_event_type(ty: &str) -> bool {
    matches!(
        ty,
        "response.created"
            | "response.queued"
            | "response.in_progress"
            | "response.completed"
            | "response.failed"
            | "response.incomplete"
            | "response.output_item.added"
            | "response.output_item.done"
            | "response.content_part.added"
            | "response.content_part.done"
            | "response.output_text.delta"
            | "response.output_text.done"
            | "response.reasoning_summary_part.added"
            | "response.reasoning_summary_part.done"
            | "response.refusal.delta"
            | "response.refusal.done"
            | "response.reasoning.delta"
            | "response.reasoning.done"
            | "response.reasoning_summary_text.delta"
            | "response.reasoning_summary_text.done"
            | "response.output_text.annotation.added"
            | "response.function_call_arguments.delta"
            | "response.function_call_arguments.done"
            | "error"
    )
}

/// An unknown streaming event type not recognized by this version of the SDK.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UnknownEvent {
    /// The event type string from the server (e.g., `"response.reasoning.delta"`).
    #[serde(rename = "type")]
    pub event_type: String,
    /// The remaining JSON fields from the event payload (excludes the `"type"` field).
    #[serde(flatten)]
    pub payload: serde_json::Map<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_serializing_none() {
        let body = CreateResponseBody {
            model: Some("test".into()),
            input: Some(CreateResponseInput::String("hi".into())),
            previous_response_id: None,
            include: None,
            tools: None,
            tool_choice: None,
            metadata: None,
            text: None,
            temperature: None,
            top_p: None,
            presence_penalty: None,
            frequency_penalty: None,
            parallel_tool_calls: None,
            stream: None,
            stream_options: None,
            background: None,
            max_output_tokens: None,
            max_tool_calls: None,
            reasoning: None,
            safety_identifier: None,
            prompt_cache_key: None,
            truncation: None,
            instructions: None,
            store: None,
            service_tier: None,
            top_logprobs: None,
        };
        let json = serde_json::to_string_pretty(&body).unwrap();
        println!("SERIALIZED JSON:\n{}", json);
        // None fields must NOT appear in the JSON
        assert!(
            !json.contains("\"tools\""),
            "tools should be omitted when None"
        );
        assert!(
            !json.contains("\"stream\""),
            "stream should be omitted when None"
        );
        assert!(
            !json.contains("\"store\""),
            "store should be omitted when None"
        );
        assert!(
            !json.contains("\"text\""),
            "text should be omitted when None"
        );
    }

    #[test]
    fn unknown_event_type_deserializes() {
        let json = r#"{"type":"response.new_type","sequence_number":5,"content":"thinking..."}"#;
        let event: StreamingEvent = serde_json::from_str(json).unwrap();
        match event {
            StreamingEvent::Unknown(ref u) => {
                assert_eq!(u.event_type, "response.new_type");
                assert_eq!(
                    u.payload.get("sequence_number").unwrap(),
                    &serde_json::json!(5)
                );
                assert_eq!(
                    u.payload.get("content").unwrap(),
                    &serde_json::json!("thinking...")
                );
                // "type" is extracted into event_type, not duplicated in payload
                assert!(!u.payload.contains_key("type"));
            }
            other => panic!("expected Unknown, got: {other:?}"),
        }
    }

    #[test]
    fn known_event_type_still_matches_specific_variant() {
        let json = serde_json::json!({
            "type": "error",
            "sequence_number": 1,
            "error": {
                "type": "server_error",
                "message": "boom",
                "code": null,
                "param": null,
                "headers": null
            }
        });
        let event: StreamingEvent = serde_json::from_value(json).unwrap();
        assert!(
            matches!(event, StreamingEvent::Error { .. }),
            "expected Error variant, got: {event:?}"
        );
    }

    #[test]
    fn reasoning_delta_event_deserializes() {
        let json = serde_json::json!({
            "type": "response.reasoning.delta",
            "sequence_number": 9,
            "item_id": "rs_123",
            "output_index": 0,
            "content_index": 1,
            "delta": "thinking"
        });
        let event: StreamingEvent = serde_json::from_value(json).unwrap();
        match event {
            StreamingEvent::ResponseReasoningDelta { delta, .. } => assert_eq!(delta, "thinking"),
            other => panic!("expected ResponseReasoningDelta, got: {other:?}"),
        }
    }

    #[test]
    fn function_call_arguments_done_event_deserializes() {
        let json = serde_json::json!({
            "type": "response.function_call_arguments.done",
            "sequence_number": 4,
            "item_id": "fc_123",
            "output_index": 0,
            "arguments": "{\"city\":\"Berlin\"}"
        });
        let event: StreamingEvent = serde_json::from_value(json).unwrap();
        match event {
            StreamingEvent::ResponseFunctionCallArgumentsDone { arguments, .. } => {
                assert_eq!(arguments, "{\"city\":\"Berlin\"}")
            }
            other => panic!("expected ResponseFunctionCallArgumentsDone, got: {other:?}"),
        }
    }

    #[test]
    fn unknown_event_round_trips() {
        let json = r#"{"type":"response.new_thing","seq":42,"nested":{"a":1}}"#;
        let event: StreamingEvent = serde_json::from_str(json).unwrap();
        let reserialized = serde_json::to_value(&event).unwrap();
        let original: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(reserialized, original);
    }

    #[test]
    fn unknown_event_with_no_extra_fields() {
        let json = r#"{"type":"response.heartbeat"}"#;
        let event: StreamingEvent = serde_json::from_str(json).unwrap();
        match event {
            StreamingEvent::Unknown(ref u) => {
                assert_eq!(u.event_type, "response.heartbeat");
                assert!(u.payload.is_empty());
            }
            other => panic!("expected Unknown, got: {other:?}"),
        }
    }

    #[test]
    fn known_event_type_with_missing_fields_returns_error() {
        // When a known type tag matches but required fields are missing,
        // deserialization returns an error instead of silently falling through
        // to Unknown. This ensures real deserialization bugs are caught.
        let json = serde_json::json!({
            "type": "response.output_text.delta",
            "sequence_number": 1,
            "item_id": "msg_001",
            // missing: output_index, content_index, delta, logprobs
        });
        let result = serde_json::from_value::<StreamingEvent>(json);
        assert!(
            result.is_err(),
            "expected error for known type with missing fields"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("missing field"),
            "error should mention missing field, got: {err_msg}"
        );
    }
}

/// An error payload that was emitted for a streaming error event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ErrorPayload {
    /// The error type that was emitted.
    #[serde(rename = "type")]
    pub ty: String,
    /// The error code that was emitted, if any.
    pub code: Option<String>,
    /// The human-readable error message that was emitted.
    pub message: String,
    /// The parameter name that was associated with the error, if any.
    pub param: Option<String>,
    /// The response headers that were emitted with the error, if any.
    pub headers: Option<HashMap<String, String>>,
}
