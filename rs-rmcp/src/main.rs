use std::env;

use log::{error, info};
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::{ServiceExt, transport::stdio};

mod calculator;
use calculator::Calculator;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    // 获取所有命令行参数
    let args: Vec<String> = env::args().collect();

    info!("args: {args:?}");

    if args.len() > 1 && args[1] == "stdio" {
        stdio_server().await?;
        return Ok(());
    }

    start_server().await?;

    Ok(())
}

fn init_log() {
    tracing_subscriber::fmt()
        .compact()
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .log_internal_errors(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .init();
}

async fn stdio_server() -> anyhow::Result<()> {
    // Create an instance of our Calculator router
    let service = Calculator::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}

async fn start_server() -> anyhow::Result<()> {
    let http_service = StreamableHttpService::new(
        || Ok(Calculator::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let sse_config = SseServerConfig {
        bind: BIND_ADDRESS.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    // Create SSE server
    let (sse_server, sse_router) = SseServer::new(sse_config);
    // Start SSE server with Calculator service
    sse_server.with_service(Calculator::new);
    // // Register token validation middleware for SSE
    // let sse_cancel_token = sse_server.config.ct.clone();
    // // Handle Ctrl+C
    // let sse_cancel_token2 = sse_server.config.ct.clone();

    // Create HTTP router with request logging middleware
    let app = axum::Router::new()
        .nest_service("/mcp", http_service)
        .merge(sse_router)
        .with_state(());

    // SSE Handle signals for graceful shutdown
    // tokio::spawn(async move {
    //     match tokio::signal::ctrl_c().await {
    //         Ok(()) => {
    //             info!("Received Ctrl+C, shutting down");
    //             sse_cancel_token.cancel();
    //         }
    //         Err(e) => error!("Failed to listen for Ctrl+C: {}", e),
    //     }
    // });

    // Start HTTP server
    info!("MCP Server started on {}", BIND_ADDRESS);
    let listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await?;
    let server = axum::serve(listener, app).with_graceful_shutdown(async move {
        // sse_cancel_token2.cancelled().await;
        tokio::signal::ctrl_c().await.unwrap();
        info!("Server is shutting down");
    });

    // start HTTP and SSE servers
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    info!("Server has been shut down");
    Ok(())
}
