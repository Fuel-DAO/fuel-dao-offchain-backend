use anyhow::{anyhow, Context};
use ic_agent::identity::DelegatedIdentity;

use crate::canister::backend::RentalTransaction;
use crate::canister::canister::Canisters;
use crate::domain::transactions::models::transaction::{Aadhar, Age, CreateTransactionError, CreateTransactionRequest, EmailAddress, MobileNumber, Transaction, UserName, PAN};
use crate::domain::transactions::ports::TransactionRepository;
use crate::identity::identity::extract_identity;



#[derive(Debug, Clone)]
pub struct IcAgentTransactionRepository;

impl IcAgentTransactionRepository {
    pub fn new() -> Self {
        Self 
    }

    /// Call the transaction function in the canister.
    async fn call_create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<RentalTransaction, anyhow::Error> {

        let secret = req.secret();

        let identity = extract_identity(Some(secret)).context("failed to set delegated identity")?;

        let canister = Canisters::authenticated(DelegatedIdentity::try_from(identity).context("Failed to generate delegated identity from secret")?);

        let backend  = canister.backend();

        let tx = backend.reserve_car(req.car_id(), req.start_time(), req.end_time(), req.customer() ).await?;

        match tx {
            crate::canister::backend::Result_::Ok(rental_transaction) => Ok(rental_transaction),
            crate::canister::backend::Result_::Err(e) => Err(anyhow!(e)),
        }
    }
}

impl TransactionRepository for IcAgentTransactionRepository {
    async fn create_transaction(&self, req: &CreateTransactionRequest) -> Result<Transaction, CreateTransactionError> {
        let response = self.call_create_transaction(req).await.map_err(|e| {
            // Handle specific errors based on your application's needs.
            CreateTransactionError::Unknown(anyhow!(e))
        })?;

        let customer = response.customer.ok_or( CreateTransactionError::Unknown(anyhow!("Failed to parse Customer")))?;

        // Handle response and return Transaction object
        Ok(Transaction::new(
        response.booking_id,
        response.car_id,
         UserName::new(&customer.name)?,  
        EmailAddress::new(&customer.email)?,
        Age::new(customer.age)?,
        customer.country_code.parse::<u16>().map_err(|_| anyhow!("Could not parse country code"))?,
        MobileNumber::new(customer.mobile_number.parse::<u64>().map_err(|_| anyhow!("Could not parse country code"))?)?,
        PAN::new(&customer.pan)?,
        Aadhar::new(&customer.aadhar)?,
        response.start_timestamp,
        response.end_timestamp
        ))
    }
}
