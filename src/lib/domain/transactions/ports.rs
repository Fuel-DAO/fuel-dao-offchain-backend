/*
   Module `ports` specifies the API by which external modules interact with the transaction domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use std::future::Future;

use crate::domain::transactions::models::transaction::{
    CreateTransactionError,
    {Transaction, CreateTransactionRequest}
};


/// `TransactionService` is the public API for the transaction domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait TransactionService: Clone + Send + Sync + 'static {
    /// Asynchronously create a new [Transaction].
    ///
    /// # Errors
    ///
    /// - [CreateTransactionError::Duplicate] if a [Transaction] with the same [TransactionName] already exists.
    fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> impl Future<Output = Result<Transaction, CreateTransactionError>> + Send;
}

/// `TransactionRepository` represents a store of transaction data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait TransactionRepository: Send + Sync + Clone + 'static {
    /// Asynchronously persist a new [Transaction].
    ///
    /// # Errors
    ///
    /// - MUST return [CreateTransactionError::Duplicate] if a [Transaction] with the same [TransactionName]
    ///   already exists.
    fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> impl Future<Output = Result<Transaction, CreateTransactionError>> + Send;
}

/// `TransactionMetrics` describes an aggregator of transaction-related metrics, such as a time-series
/// database.
pub trait TransactionMetrics: Send + Sync + Clone + 'static {
    /// Record a successful transaction creation.
    fn record_transaction_creation_success(&self) -> impl Future<Output = ()> + Send;

    /// Record a transaction creation failure.
    fn record_transaction_creation_failure(&self) -> impl Future<Output = ()> + Send;
}

/// `TransactionNotifier` triggers notifications related to transactions.
///
/// Whether the notification medium (email, SMS, etc.) is known by the business logic is a
/// judgement call based on your use case.
///
/// Some domains will always require email, for example, so hiding this detail would be
/// pointless.
///
/// For others, code coordinating notifications will be complex enough to warrant its own domain.
/// In this case, a `TransactionNotifier` adapter will call that domain's `Service`.
pub trait TransactionNotifier: Send + Sync + Clone + 'static {
    fn transaction_created(&self, transaction: &Transaction) -> impl Future<Output = ()> + Send;
}
