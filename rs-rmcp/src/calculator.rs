use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::{
        AnnotateAble, CallToolResult, Content, GetPromptRequestMethod, GetPromptRequestParam,
        GetPromptResult, Implementation, InitializeRequestParam, InitializeResult, JsonObject,
        ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult, PaginatedRequestParam,
        Prompt, PromptArgument, PromptMessage, PromptMessageContent, PromptMessageRole,
        RawResource, RawResourceTemplate, ReadResourceRequestParam, ReadResourceResult,
        ResourceContents, ServerCapabilities, ServerInfo,
    },
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::extract::Path;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SumRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SubRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    #[schemars(description = "the right hand side number")]
    pub b: i32,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct SubResponse {
    #[schemars(description = "the result")]
    pub result: i32,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct EchoRequest {
    #[schemars(description = "the left hand side number")]
    pub a: i32,
    #[schemars(description = "the right hand side number")]
    pub b: i32,
}

#[derive(Debug, Clone)]
pub struct Calculator {
    tool_router: ToolRouter<Self>,
}

/// tool
#[tool_router]
impl Calculator {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Say hello to the client")]
    fn say_hello(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text("hello")]))
    }

    // MCP Inspector 无法显示出请求结构
    #[tool(description = "Repeat what you say")]
    fn echo(&self, Parameters(object): Parameters<JsonObject>) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::Value::Object(object).to_string(),
        )]))
    }

    #[tool(description = "Repeat what you say")]
    fn echo2(&self, Parameters(req): Parameters<EchoRequest>) -> Result<CallToolResult, McpError> {
        let binding = serde_json::to_value(req).unwrap();
        let object = binding.as_object().unwrap();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::Value::Object(object.clone()).to_string(),
        )]))
    }

    #[tool(name = "sum", description = "Calculate the sum of two numbers")]
    fn sum(&self, Parameters(SumRequest { a, b }): Parameters<SumRequest>) -> String {
        (a + b).to_string()
    }

    // 返回的 Json<T> 不兼容性写法, MCP Inspector 解析异常
    // #[tool(description = "Calculate the difference of two numbers")]
    // fn sub(&self, Parameters(SubRequest { a, b }): Parameters<SubRequest>) -> Json<i32> {
    //     Json(a - b)
    // }

    #[tool(description = "Calculate the difference of two numbers")]
    fn sub2(
        &self,
        Parameters(SubRequest { a, b }): Parameters<SubRequest>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::json(SubResponse {
            result: a - b,
        })?]))
    }
}

/// resource/prompt
impl Calculator {
    /// Static resource example - exposing a README file
    pub fn readme(&self, uri: String) -> Result<ReadResourceResult, McpError> {
        // 读取 README.md 文件内容
        let text = std::fs::read_to_string("./README.md").unwrap();
        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(text, uri)],
        })
    }

    /// 读取 report.pdf 文件内容
    pub fn report_pdf(&self, uri: String) -> Result<ReadResourceResult, McpError> {
        let text = "this is a report.pdf contents".to_string();
        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(text, uri)],
        })
    }

    /// Dynamic resource example - user profiles by ID
    pub fn dynamic_resource_by_id(
        &self,
        uri: &str,
        id: i32,
    ) -> Result<ReadResourceResult, McpError> {
        let text = format!("read: //dynamic/resource/{id}").to_string();

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(text, uri)],
        })
    }

    pub fn dynamic_resource_by_name(
        &self,
        uri: &str,
        name: &str,
    ) -> Result<ReadResourceResult, McpError> {
        let text = format!("read: /documents/{name}.text").to_string();

        Ok(ReadResourceResult {
            contents: vec![ResourceContents::text(text, uri)],
        })
    }

    /// Dynamic resource example - user profiles by ID
    pub fn code_review(
        &self,
        params: &Option<Map<String, Value>>,
    ) -> Result<GetPromptResult, McpError> {
        Ok(GetPromptResult {
            description: Some("this is a review description".to_string()),
            messages: vec![PromptMessage {
                role: PromptMessageRole::User,
                content: PromptMessageContent::text(format!(
                    "this is a review message, params: {params:?}"
                )),
            }],
        })
    }
}

#[tool_handler]
impl ServerHandler for Calculator {
    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        if let Some(http_request_part) = context.extensions.get::<axum::http::request::Parts>() {
            let initialize_headers = &http_request_part.headers;
            let initialize_uri = &http_request_part.uri;
            tracing::info!(?initialize_headers, %initialize_uri, "initialize from http server");
        }
        Ok(self.get_info())
    }

    /// 服务器信息
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple calculator".into()),
            server_info: Implementation::from_build_env(),
            capabilities: ServerCapabilities::builder()
                // .enable_logging()
                .enable_experimental()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .enable_tool_list_changed()
                .build(),
            ..Default::default()
        }
    }

    /// 动态 resources
    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            next_cursor: None,
            resources: vec![
                RawResource {
                    uri: "docs://readme".to_string(),
                    name: "Project README".to_string(),
                    description: Some("The project's README file".to_string()),
                    mime_type: Some("text/markdown".to_string()),
                    size: None,
                },
                RawResource {
                    uri: "file:///documents/report.pdf".to_string(),
                    name: "Project README".to_string(),
                    description: Some("The project's README file".to_string()),
                    mime_type: Some("text/plain".to_string()),
                    size: None,
                },
            ]
            .into_iter()
            .map(|v| v.no_annotation())
            .collect(),
        })
    }

    /// 静态 resources
    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: vec![
                RawResourceTemplate {
                    uri_template: "test://dynamic/resource/{id}".to_string(),
                    name: "Dynamic Resource".to_string(),
                    description: Some("A dynamic resource template. This resource contains the URI parameter `{id}` in its name".to_string()),
                    mime_type: Some("text/plain".to_string()),
                },
                RawResourceTemplate {
                    uri_template: "file:///documents/{name}.text".to_string(),
                    name: "read file".to_string(),
                    description: Some("".to_string()),
                    mime_type: Some("text/plain".to_string()),
                },
            ]
            .into_iter()
            .map(|v| v.no_annotation())
            .collect(),
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        if let Ok((id,)) = Path::<(i32,)>::extract(&uri, "test://dynamic/resource/{id}") {
            return self.dynamic_resource_by_id(&uri, id);
        }
        if let Ok((name,)) = Path::<(String,)>::extract(&uri, "file:///documents/{name}.text") {
            return self.dynamic_resource_by_name(&uri, &name);
        }

        // 静态路由匹配
        match uri.as_str() {
            "docs://readme" => self.readme(uri),
            "file:///documents/report.pdf" => self.report_pdf(uri),
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            prompts: vec![Prompt::new(
                "code_review".to_string(),
                Some("Code review assistance".to_string()),
                Some(vec![PromptArgument {
                    name: "pr_number".to_string(),
                    description: Some("pr_number is required".to_string()),
                    required: Some(true),
                }]),
            )],
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "code_review" => self.code_review(&arguments),
            _ => Err(McpError::method_not_found::<GetPromptRequestMethod>()),
        }
    }
}
