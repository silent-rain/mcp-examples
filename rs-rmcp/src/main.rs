use clap::{Parser, ValueEnum};
use log::{error, info};
use rmcp::{
    ServiceExt,
    transport::{
        sse_server::{SseServer, SseServerConfig},
        stdio,
        streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager},
    },
};
use tracing_subscriber::fmt::format::FmtSpan;

mod error;
mod extract;

mod calculator;
use calculator::Calculator;

#[derive(Debug, Clone, ValueEnum)]
enum Transport {
    Stdio,
    Http,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Transport type to use (stdio or http)
    #[arg(short, long, value_enum, default_value = "http")]
    transport: Transport,

    /// Server address for HTTP transport (format: host:port)
    #[arg(short, long, default_value = "127.0.0.1:8000")]
    address: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    // Parse command line arguments using clap
    let args = Args::parse();
    info!(
        "args transport: {:?}, address: {}",
        args.transport, args.address
    );

    match args.transport {
        Transport::Stdio => {
            stdio_server().await?;
            return Ok(());
        }
        Transport::Http => {
            start_http_server(&args.address).await?;
        }
    }

    Ok(())
}

/// Initializes a logger.
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
        .with_span_events(FmtSpan::CLOSE)
        .init();
}

/// Starts TCP server to communicate with standard input/output
async fn stdio_server() -> anyhow::Result<()> {
    // Create an instance of our Calculator router
    let service = Calculator::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("stdio serving error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}

/// Starts SSE and HTTP servers
async fn start_http_server(address: &str) -> anyhow::Result<()> {
    let http_service = StreamableHttpService::new(
        || Ok(Calculator::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let sse_config = SseServerConfig {
        bind: address.parse()?,
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
    info!("MCP Server started on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await?;
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
