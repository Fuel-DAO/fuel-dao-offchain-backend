/*!
   Module `service` provides the canonical implementation of the [TransactionService] port. All
   transaction-domain logic is defined here.
*/

use anyhow::anyhow;

use crate::{canister::backend::RazorpayPayment, domain::transactions::models::transaction::{
    CreateTransactionError, CreateTransactionRequest, Transaction
}};
use crate::domain::transactions::ports::{TransactionNotifier, TransactionMetrics, TransactionRepository, TransactionService};

use super::ports::PaymentService;

/// Canonical implementation of the [TransactionService] port, through which the transaction domain API is
/// consumed.
#[derive(Debug, Clone)]
pub struct Service<R, M, N, P>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
    P: PaymentService,
{
    repo: R,
    metrics: M,
    transaction_notifier: N,
    payment_service: P,
}

impl<R, M, N, P> Service<R, M, N, P>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
    P: PaymentService,
{
    pub fn new(repo: R, metrics: M, transaction_notifier: N, payment_service: P) -> Self {
        Self {
            repo,
            metrics,
            transaction_notifier,
            payment_service,
        }
    }
}

impl<R, M, N, P> TransactionService for Service<R, M, N, P>
where
    R: TransactionRepository,
    M: TransactionMetrics,
    N: TransactionNotifier,
    P: PaymentService,
{
    /// Create the [Transaction] specified in `req` and trigger notifications.
    ///
    /// # Errors
    ///
    /// - Propagates any [CreateTransactionError] returned by the [TransactionRepository].
    async fn create_transaction(&self, booking_id: u64, payment: &RazorpayPayment) -> Result<Transaction, CreateTransactionError> {
        let result = self.repo.create_transaction(booking_id, payment).await;
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

    async fn create_payment_link(&self, req: &CreateTransactionRequest) -> Result<String, CreateTransactionError> {
        let result = self.repo.check_if_car_available(req).await;
        match result {
            Ok(tx) => {
                // total amount + 2.36 % razorpay fee and taxes
                self.payment_service.create_payment_link(tx.total_amount * 1.0236, tx.booking_id).await.map_err(|f| CreateTransactionError::Unknown(anyhow!("Failed to generate the payment link {f}")))
            },
            Err(e) => Err(e),
        }
    }

    async fn get_principal(&self) -> Result<String, CreateTransactionError> {
        self.repo.get_principal().await
    }
}
