use serde::{Deserialize, Serialize};
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemReferenceParam {
    /// The type of item to reference. Always `item_reference`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningItemParam {
    /// The unique ID of this reasoning item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item type. Always `reasoning`.
    #[serde(rename = "type")]
    pub ty: String,
    /// Reasoning summary content associated with this item.
    pub summary: Vec<ReasoningSummaryContentParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    /// An encrypted representation of the reasoning content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

/// A text input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputTextContentParam {
    /// The type of the input item. Always `input_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The text input to the model.
    pub text: String,
}

/// An image input to the model. Learn about [image inputs](/docs/guides/vision)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImageContentParamAutoParam {
    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// A file input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFileContentParam {
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The name of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The base64-encoded data of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The URL of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserMessageItemParam {
    /// The unique ID of this message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item type. Always `message`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The message role. Always `user`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: UserMessageContent,
    /// The status of the message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SystemMessageContent {
    Array(Vec<InputTextContentParam>),
    /// The message content, as a single string.
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SystemMessageItemParam {
    /// The unique ID of this message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item type. Always `message`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The message role. Always `system`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: SystemMessageContent,
    /// The status of the message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DeveloperMessageItemParam {
    /// The unique ID of this message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item type. Always `message`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The message role. Always `developer`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: SystemMessageContent,
    /// The status of the message item.
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OutputTextContentParam {
    /// The content type. Always `output_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The text content.
    pub text: String,
    /// Citations associated with the text content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<UrlCitationParam>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RefusalContentParam {
    /// The content type. Always `refusal`.
    #[serde(rename = "type")]
    pub ty: String,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AssistantMessageItemParam {
    /// The unique ID of this message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item type. Always `message`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The role of the message author. Always `assistant`.
    pub role: String,
    /// The message content, as an array of content parts.
    pub content: AssistantMessageContent,
    /// The status of the message item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallItemParam {
    /// The unique ID of this function tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The item type. Always `function_call`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The name of the function to call.
    pub name: String,
    /// The function arguments as a JSON string.
    pub arguments: String,
    /// The status of the function tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FunctionCallStatus>,
}

/// A content block representing a video input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputVideoContent {
    /// The type of the input content. Always `input_video`.
    #[serde(rename = "type")]
    pub ty: String,
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionCallOutputItemParam {
    /// The unique ID of the function tool call output. Populated when this item is returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The type of the function tool call output. Always `function_call_output`.
    #[serde(rename = "type")]
    pub ty: String,
    /// Text, image, or file output of the function tool call.
    pub output: FunctionCallOutput,
    /// The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FunctionToolParam {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub strict: Option<bool>,
    #[serde(rename = "type")]
    pub ty: String,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AllowedToolsParam {
    /// The tool choice type. Always `allowed_tools`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The list of tools that are permitted for this request.
    pub tools: Vec<SpecificToolChoiceParam>,
    /// How to select a tool from the allowed set.
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextParam {
    /// The format configuration for text output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<serde_json::Value>,
    /// Controls the level of detail in generated text output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityEnum>,
}

/// Options that control streamed response behavior.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StreamOptionsParam {
    /// Whether to obfuscate sensitive information in streamed output. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
}

/// **gpt-5 and o-series models only** Configuration options for [reasoning models](https://platform.openai.com/docs/guides/reasoning).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningParam {
    /// Controls the level of reasoning effort the model should apply. Higher effort may increase latency and cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffortEnum>,
    /// Controls whether the response includes a reasoning summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryEnum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateResponseBody {
    /// The model to use for this request, e.g. 'gpt-5.2'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Context to provide to the model for the scope of this request. May either be a string or an array of input items. If a string is provided, it is interpreted as a user message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<CreateResponseInput>,
    /// The ID of the response to use as the prior turn for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<IncludeEnum>>,
    /// A list of tools that the model may call while generating the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ResponsesToolParam>>,
    /// Controls which tool the model should use, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceParam>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataParam>,
    /// Configuration options for text output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextParam>,
    /// Sampling temperature to use, between 0 and 2. Higher values make the output more random.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Nucleus sampling parameter, between 0 and 1. The model considers only the tokens with the top cumulative probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Penalizes new tokens based on whether they appear in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Penalizes new tokens based on their frequency in the text so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Whether the model may call multiple tools in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Whether to stream response events as server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Options that control streamed response behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptionsParam>,
    /// Whether to run the request in the background and return immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// The maximum number of tokens the model may generate for this response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    /// The maximum number of tool calls the model may make while generating the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i32>,
    /// Configuration options for reasoning behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningParam>,
    /// A stable identifier used for safety monitoring and abuse detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// A key to use when reading from or writing to the prompt cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Controls how the service truncates the input when it exceeds the model context window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationEnum>,
    /// Additional instructions to guide the model for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Whether to store the response so it can be retrieved later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// The service tier to use for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTierEnum>,
    /// The number of most likely tokens to return at each position, along with their log probabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// The type of the input item. Always `input_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The text input to the model.
    pub text: String,
}

/// A citation for a web resource used to generate a model response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UrlCitationBody {
    /// The type of the URL citation. Always `url_citation`.
    #[serde(rename = "type")]
    pub ty: String,
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
    /// The type of the output text. Always `output_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The text output from the model.
    pub text: String,
    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,
    pub logprobs: Vec<LogProb>,
}

/// A text content.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub ty: String,
    pub text: String,
}

/// A summary text from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SummaryTextContent {
    /// The type of the object. Always `summary_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// A summary of the reasoning output from the model so far.
    pub text: String,
}

/// Reasoning text from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReasoningTextContent {
    /// The type of the reasoning text. Always `reasoning_text`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The reasoning text from the model.
    pub text: String,
}

/// A refusal from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RefusalContent {
    /// The type of the refusal. Always `refusal`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The refusal explanation from the model.
    pub refusal: String,
}

/// An image input to the model. Learn about [image inputs](/docs/guides/vision).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputImageContent {
    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    pub image_url: Option<String>,
    /// The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`.
    pub detail: ImageDetail,
}

/// A file input to the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputFileContent {
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub ty: String,
    /// The name of the file to be sent to the model.
    pub filename: String,
    /// The URL of the file to be sent to the model.
    pub file_url: String,
}

/// A message to or from the model.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    /// The type of the message. Always set to `message`.
    #[serde(rename = "type")]
    pub ty: String,
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
    /// The type of the item. Always `function_call`.
    #[serde(rename = "type")]
    pub ty: String,
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
    /// The type of the function tool call output. Always `function_call_output`.
    #[serde(rename = "type")]
    pub ty: String,
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
    /// The type of the item. Always `reasoning`.
    #[serde(rename = "type")]
    pub ty: String,
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
    /// The type of the function tool. Always `function`.
    #[serde(rename = "type")]
    pub ty: String,
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
    pub reasoning: Reasoning,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    /// A streaming event that indicated an error was emitted.
    #[serde(rename = "error")]
    Error {
        /// The sequence number of the event that was emitted.
        sequence_number: i32,
        /// The error payload that was emitted.
        error: ErrorPayload,
    },
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
