/*!
   Module `service` provides the canonical implementation of the [TransactionService] port. All
   transaction-domain logic is defined here.
*/

use crate::domain::transactions::models::transaction::{
    CreateTransactionError,
    {Transaction, CreateTransactionRequest}
};
use crate::domain::transactions::ports::{TransactionNotifier, TransactionMetrics, TransactionRepository, TransactionService};

/// Canonical implementation of the [TransactionService] port, through which the transaction domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, M, N>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
{
    repo: R,
    metrics: M,
    transaction_notifier: N,
}

impl<R, M, N> Service<R, M, N>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
{
    pub fn new(repo: R, metrics: M, transaction_notifier: N) -> Self {
        Self {
            repo,
            metrics,
            transaction_notifier,
        }
    }
}

impl<R, M, N> TransactionService for Service<R, M, N>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
{
    /// Create the [Transaction] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateTransactionError] returned by the [TransactionRepository].
    async fn create_transaction(&self, req: &CreateTransactionRequest) -> Result<Transaction, CreateTransactionError> {
        let result = self.repo.create_transaction(req).await;
        if result.is_err() {
            self.metrics.record_transaction_creation_failure().await;
        } else {
            self.metrics.record_transaction_creation_success().await;
            self.transaction_notifier
                .transaction_created(result.as_ref().unwrap())
                .await;
        }

        result
    }
}
