use offchain::config::Config;
use offchain::domain::transactions::service::Service;
use offchain::inbound::http::{HttpServer, HttpServerConfig};
use offchain::outbound::email_client::EmailClient;
use offchain::outbound::prometheus::Prometheus;
use offchain::outbound::ic_agent::IcAgentTransactionRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();

    let prometheus = Prometheus::new();
    let email_client = EmailClient::new(config.email_config);
    let ic_agent = IcAgentTransactionRepository::new();
    let offchain_service = Service::new(ic_agent, prometheus, email_client);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let http_server = HttpServer::new(offchain_service, server_config).await?;
    http_server.run().await
}