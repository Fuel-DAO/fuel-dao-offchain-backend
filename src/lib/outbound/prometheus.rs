use crate::domain::transactions::ports::TransactionMetrics;

/// An unimplemented example of an adapter to [TransactionMetrics].
#[derive(Debug, Clone)]
pub struct Prometheus;

impl Prometheus {
    pub fn new() -> Self {
        Self
    }
}

impl TransactionMetrics for Prometheus {
    async fn record_transaction_creation_success(&self) {
        // Implement logic to record a successful transaction creation
        println!("Transaction creation successful.");
        // Here, you would typically send a metric to Prometheus
    }

    async fn record_transaction_creation_failure(&self) {
        // Implement logic to record a failed transaction creation
        println!("Transaction creation failed.");
        // Here, you would typically send a metric to Prometheus
    }
}
