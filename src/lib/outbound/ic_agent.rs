use anyhow::{anyhow, Context};
use ic_agent::identity::DelegatedIdentity;
use ic_agent::Identity;

use crate::canister::backend::{RazorpayPayment, RentalTransaction};
use crate::canister::canister::Canisters;
use crate::domain::transactions::models::transaction::{Aadhar, Age, CreateTransactionError, CreateTransactionRequest, EmailAddress, MobileNumber, Transaction, UserName, PAN};
use crate::domain::transactions::ports::TransactionRepository;
use crate::identity::delegated_identity::DelegatedIdentityWire;
use crate::identity::identity::extract_identity;



#[derive(Debug, Clone)]
pub struct IcAgentTransactionRepository{
    admin_private_key: String,
}

impl IcAgentTransactionRepository {
    pub fn new(admin_private_key: String) -> Self {
        Self {
            admin_private_key
        }
    }

    fn create_identity_for_admin(&self) -> impl Identity {
        let private_key = self.admin_private_key.clone();
    
        let identity = ic_agent::identity::Secp256k1Identity::from_pem(
            stringreader::StringReader::new(private_key.as_str()),
        )
        .unwrap();
    
        identity
    }

    fn get_admin_principal(&self) -> Result<String, CreateTransactionError> {
        let identity = self.create_identity_for_admin();
        let identity = DelegatedIdentity::try_from(DelegatedIdentityWire::delegate(&identity)).map_err( |f| CreateTransactionError::Unknown(anyhow!(format!("Failed to get principal: {f:?}"))))?;
        identity.sender().map_err(|f| CreateTransactionError::Unknown(anyhow!(format!("Failed to get principal: {f:?}")))).map(|f| f.to_text())
    }

    async fn call_check_if_car_available(&self, req: &CreateTransactionRequest) -> Result<RentalTransaction, CreateTransactionError> {

        let secret = req.secret();

        let identity = extract_identity(Some(secret)).context("failed to set delegated identity")?;

        let identity = DelegatedIdentity::try_from(identity).context("Failed to generate delegated identity from secret")?;

        let caller =identity.sender().map_err(|f| anyhow::format_err!("{f}") )?;

        let canister = Canisters::authenticated(identity);

        let check = canister.backend().await.validate_details_and_availaibility(req.car_id(), req.start_time(), req.end_time(), req.customer(caller) ).await.map_err(|f| CreateTransactionError::Unknown(anyhow!(f.to_string())))?;

        match check {
            crate::canister::backend::Result_::Ok(r) => Ok(r),
            crate::canister::backend::Result_::Err(e) => Err(anyhow!(e).into()),
        }

    }

    /// Call the transaction function in the canister.
    async fn call_create_transaction(
        &self,
        booking_id: u64,
        payment: &RazorpayPayment,
    ) -> Result<RentalTransaction, anyhow::Error> {

        let identity = self.create_identity_for_admin();

        let canister = Canisters::set_arc_id(identity.into());

        let backend  = canister.backend();

        let tx = backend.await.reserve_car(booking_id, payment.clone() ).await?;

        match tx {
            crate::canister::backend::Result_::Ok(rental_transaction) => Ok(rental_transaction),
            crate::canister::backend::Result_::Err(e) => Err(anyhow!(e)),
        }
    }
}

impl TransactionRepository for IcAgentTransactionRepository {
    async fn create_transaction(&self, booking_id: u64, payment: &RazorpayPayment) -> Result<Transaction, CreateTransactionError> {
        let response = self.call_create_transaction(booking_id, payment).await.map_err(|e| {
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

    async fn check_if_car_available(&self, req: &CreateTransactionRequest) -> Result<RentalTransaction, CreateTransactionError> {
         self.call_check_if_car_available(req).await
    }

    async fn get_principal(&self) -> Result<String, CreateTransactionError> {
        self.get_admin_principal()
    }
}
