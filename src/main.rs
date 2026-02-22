use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rmcp::{
    RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    service::RequestContext,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 获取当前时间的参数
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct GetTimeParams {
    /// 时区名称，例如 "Asia/Shanghai" 或 "America/New_York"，默认为 "Asia/Shanghai"
    #[serde(default = "default_timezone")]
    pub timezone: String,
}

fn default_timezone() -> String {
    "Asia/Shanghai".to_string()
}

/// 等待功能的参数
#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WaitParams {
    /// 等待时间（单位：秒）
    pub seconds: u64,
}

#[derive(Clone)]
pub struct TimeService {
    tool_router: ToolRouter<TimeService>,
}

#[tool_router]
impl TimeService {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// 获取指定时区的当前时间
    #[tool(description = "获取指定时区的当前时间。如果未指定时区，默认返回上海时区的时间。")]
    fn get_time(
        &self,
        Parameters(params): Parameters<GetTimeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        // 解析时区
        let timezone: Tz = params.timezone.parse().map_err(|_| {
            ErrorData::invalid_params(
                format!("无效的时区：{}", params.timezone),
                Some(serde_json::json!({
                    "timezone": params.timezone
                })),
            )
        })?;

        // 获取当前时间并转换到指定时区
        let now: DateTime<Tz> = Utc::now().with_timezone(&timezone);
        let time_str = now.format("%Y-%m-%d %H:%M:%S %Z").to_string();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "当前时间（{}）: {}",
            params.timezone, time_str
        ))]))
    }

    /// 等待指定的秒数
    #[tool(description = "等待指定的秒数，然后返回等待的开始时间、结束时间和持续时间。")]
    async fn wait(
        &self,
        Parameters(params): Parameters<WaitParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let start_time: DateTime<Tz> = Utc::now().with_timezone(&Tz::Asia__Shanghai);
        
        // 等待指定的秒数
        tokio::time::sleep(tokio::time::Duration::from_secs(params.seconds)).await;
        
        let end_time: DateTime<Tz> = Utc::now().with_timezone(&Tz::Asia__Shanghai);

        let result = format!(
            "等待开始时间：{}\n等待结束时间：{}\n等待时长：{} 秒",
            start_time.format("%Y-%m-%d %H:%M:%S %Z"),
            end_time.format("%Y-%m-%d %H:%M:%S %Z"),
            params.seconds
        );

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for TimeService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "此 MCP 服务提供时间相关的工具。\n\
                 Tools:\n\
                 - get_time: 获取指定时区的当前时间，默认时区为上海 (Asia/Shanghai)\n\
                 - wait: 等待指定的秒数后返回等待信息"
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, ErrorData> {
        Ok(self.get_info())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 从环境变量读取绑定地址，默认使用 127.0.0.1:8000
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    let ct = CancellationToken::new();

    // 创建 StreamableHttpService
    let service = StreamableHttpService::new(
        || Ok(TimeService::new()),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig {
            cancellation_token: ct.child_token(),
            ..Default::default()
        },
    );

    // 创建 axum 路由
    let router = axum::Router::new().nest_service("/mcp", service);

    // 绑定 TCP 监听器
    let tcp_listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!("MCP 时间服务正在监听：{}", bind_address);

    // 启动服务
    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await;

    Ok(())
}
