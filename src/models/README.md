```meraid
graph TB
    subgraph Enums["Enums"]
        DetailEnum["DetailEnum<br/>(Auto, High, Low)"]
        MessageRole["MessageRole<br/>(User, Assistant, System, Developer)"]
        MessageStatus["MessageStatus<br/>(InProgress, Completed, Incomplete)"]
        FunctionCallStatus["FunctionCallStatus<br/>(InProgress, Completed, Incomplete)"]
        ToolChoiceValueEnum["ToolChoiceValueEnum<br/>(None, Auto, Required)"]
        VerbosityEnum["VerbosityEnum<br/>(Low, Medium, High)"]
        ReasoningEffortEnum["ReasoningEffortEnum<br/>(None, Low, Medium, High, Xhigh)"]
    end

    subgraph InputContent["Input Content Types"]
        InputTextContentParam["InputTextContentParam<br/>type, text"]
        InputImageContentParamAutoParam["InputImageContentParamAutoParam<br/>type, image_url, detail"]
        InputFileContentParam["InputFileContentParam<br/>type, filename, file_data, file_url"]
        UserMessageContentPart["UserMessageContentPart<br/>(enum)"]
    end

    subgraph OutputContent["Output Content Types"]
        OutputTextContent["OutputTextContent<br/>type, text, annotations, logprobs"]
        TextContent["TextContent<br/>type, text"]
        ReasoningTextContent["ReasoningTextContent<br/>type, text"]
        SummaryTextContent["SummaryTextContent<br/>type, text"]
        RefusalContent["RefusalContent<br/>type, refusal"]
    end

    subgraph Messages["Message Types"]
        UserMessageItemParam["UserMessageItemParam<br/>id, type, role, content, status"]
        Message["Message<br/>type, id, status, role, content"]
        MessageContentPart["MessageContentPart<br/>(enum)"]
    end

    subgraph Items["Response Items"]
        FunctionCall["FunctionCall<br/>type, id, call_id, name, arguments, status"]
        FunctionCallOutputResource["FunctionCallOutputResource<br/>type, id, call_id, output, status"]
        ReasoningBody["ReasoningBody<br/>type, id, content, summary, encrypted_content"]
        ItemField["ItemField<br/>(enum)"]
    end

    subgraph ReasoningGroup["Reasoning"]
        ReasoningItemParam["ReasoningItemParam<br/>id, type, summary, content, encrypted_content"]
        ReasoningSummaryContentParam["ReasoningSummaryContentParam<br/>type, text"]
        ReasoningConfig["Reasoning<br/>effort, summary"]
    end

    subgraph Response["Response"]
        ResponseResource["ResponseResource<br/>id, object, created_at, status<br/>output, error, tools, truncation<br/>usage, reasoning, metadata"]
        TextField["TextField<br/>format, verbosity"]
        Usage["Usage<br/>input_tokens, output_tokens"]
    end

    subgraph Tools["Tools"]
        FunctionTool["FunctionTool<br/>type, name, description<br/>parameters, strict"]
        Tool["Tool<br/>(enum)"]
    end

    subgraph Streaming["Streaming Events"]
        StreamingEvent["StreamingEvent<br/>(enum)"]
        ErrorPayload["ErrorPayload<br/>type, code, message, param, headers"]
    end

    InputTextContentParam --> UserMessageContentPart
    InputImageContentParamAutoParam --> UserMessageContentPart
    InputFileContentParam --> UserMessageContentPart
    UserMessageContentPart --> UserMessageItemParam

    OutputTextContent --> MessageContentPart
    TextContent --> MessageContentPart
    ReasoningTextContent --> MessageContentPart
    SummaryTextContent --> MessageContentPart
    RefusalContent --> MessageContentPart
    MessageContentPart --> Message

    Message --> ItemField
    FunctionCall --> ItemField
    FunctionCallOutputResource --> ItemField
    ReasoningBody --> ItemField

    ReasoningSummaryContentParam --> ReasoningItemParam
    ReasoningItemParam -.->|similar to| ReasoningBody

    FunctionTool --> Tool
    Tool --> ResponseResource
    ItemField --> ResponseResource
    TextField --> ResponseResource
    Usage --> ResponseResource
    ReasoningConfig --> ResponseResource

    MessageStatus -.->|used in| Message
    MessageRole -.->|used in| Message
    FunctionCallStatus -.->|used in| FunctionCall
    DetailEnum -.->|used in| InputImageContentParamAutoParam
    VerbosityEnum -.->|used in| TextField
    ReasoningEffortEnum -.->|used in| ReasoningConfig

    StreamingEvent -->|contains| ResponseResource
    StreamingEvent -->|contains| ItemField
    StreamingEvent -->|contains| MessageContentPart
    ErrorPayload -->|in| StreamingEvent

    style DetailEnum fill:#1a1a2e,stroke:#fff,color:#fff
    style MessageRole fill:#1a1a2e,stroke:#fff,color:#fff
    style MessageStatus fill:#1a1a2e,stroke:#fff,color:#fff
    style FunctionCallStatus fill:#1a1a2e,stroke:#fff,color:#fff
    style ToolChoiceValueEnum fill:#1a1a2e,stroke:#fff,color:#fff
    style VerbosityEnum fill:#1a1a2e,stroke:#fff,color:#fff
    style ReasoningEffortEnum fill:#1a1a2e,stroke:#fff,color:#fff
    
    style InputTextContentParam fill:#0d2636,stroke:#fff,color:#fff
    style InputImageContentParamAutoParam fill:#0d2636,stroke:#fff,color:#fff
    style InputFileContentParam fill:#0d2636,stroke:#fff,color:#fff
    style UserMessageContentPart fill:#0d2636,stroke:#fff,color:#fff
    
    style OutputTextContent fill:#1a3d3a,stroke:#fff,color:#fff
    style TextContent fill:#1a3d3a,stroke:#fff,color:#fff
    style ReasoningTextContent fill:#1a3d3a,stroke:#fff,color:#fff
    style SummaryTextContent fill:#1a3d3a,stroke:#fff,color:#fff
    style RefusalContent fill:#1a3d3a,stroke:#fff,color:#fff
    style MessageContentPart fill:#1a3d3a,stroke:#fff,color:#fff
    
    style UserMessageItemParam fill:#2d2e35,stroke:#fff,color:#fff
    style Message fill:#2d2e35,stroke:#fff,color:#fff
    
    style FunctionCall fill:#1a2d3d,stroke:#fff,color:#fff
    style FunctionCallOutputResource fill:#1a2d3d,stroke:#fff,color:#fff
    style ReasoningBody fill:#1a2d3d,stroke:#fff,color:#fff
    style ItemField fill:#1a2d3d,stroke:#fff,color:#fff
    
    style ReasoningItemParam fill:#3d2d1a,stroke:#fff,color:#fff
    style ReasoningSummaryContentParam fill:#3d2d1a,stroke:#fff,color:#fff
    style ReasoningConfig fill:#3d2d1a,stroke:#fff,color:#fff
    
    style ResponseResource fill:#2d3d1a,stroke:#fff,color:#fff
    style TextField fill:#2d3d1a,stroke:#fff,color:#fff
    style Usage fill:#2d3d1a,stroke:#fff,color:#fff
    
    style FunctionTool fill:#3d2d3d,stroke:#fff,color:#fff
    style Tool fill:#3d2d3d,stroke:#fff,color:#fff
    
    style StreamingEvent fill:#1a3d3d,stroke:#fff,color:#fff
    style ErrorPayload fill:#1a3d3d,stroke:#fff,color:#fff
```
