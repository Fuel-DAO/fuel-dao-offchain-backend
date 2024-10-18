/*!
   Module `create_transaction` specifies an HTTP handler for creating a new [Transaction], and the
   associated data structures.
*/

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::transactions::models::transaction::*;
use crate::domain::transactions::ports::TransactionService;
use crate::inbound::http::AppState;
#[derive(Debug, Clone)]
pub struct ApiSuccess<T: Serialize + PartialEq>(StatusCode, Json<ApiResponseBody<T>>);

impl<T> PartialEq for ApiSuccess<T>
where
    T: Serialize + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 .0 == other.1 .0
    }
}

impl<T: Serialize + PartialEq> ApiSuccess<T> {
    fn new(status: StatusCode, data: T) -> Self {
        ApiSuccess(status, Json(ApiResponseBody::new(status, data)))
    }
}

impl<T: Serialize + PartialEq> IntoResponse for ApiSuccess<T> {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    InternalServerError(String),
    UnprocessableEntity(String),
    Conflict(String),
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<CreateTransactionError> for ApiError {
    fn from(e: CreateTransactionError) -> Self {
        match e {
            CreateTransactionError::Unknown(cause) => {
                tracing::error!("Request Failed{:?}\n{}", cause, cause.backtrace());
                Self::Conflict(cause.to_string())
            }
            CreateTransactionError::InvalidAge(age_error) => {
                Self::UnprocessableEntity(format!("Invalid age: {}", age_error))
            }
            CreateTransactionError::InvalidPAN(pan_error) => {
                Self::UnprocessableEntity(format!("Invalid PAN: {}", pan_error))
            }
            CreateTransactionError::InvalidAadhar(aadhar_error) => {
                Self::UnprocessableEntity(format!("Invalid Aadhar: {}", aadhar_error))
            }
            CreateTransactionError::UserNameEmpty(user_name_empty_error) => {
                Self::UnprocessableEntity(format!(
                    "Username cannot be empty: {}",
                    user_name_empty_error
                ))
            }
            CreateTransactionError::InvalidEmail(email_address_error) => {
                Self::UnprocessableEntity(format!("Invalid email address: {}", email_address_error))
            }
            CreateTransactionError::InvalidMobile(mobile_number_error) => {
                Self::UnprocessableEntity(format!("Invalid mobile number: {}", mobile_number_error))
            }
            CreateTransactionError::StartTimeError => {
                Self::UnprocessableEntity("Invalid start time for transaction".to_string())
            }
            CreateTransactionError::EndTimeError => {
                Self::UnprocessableEntity("Invalid end time for transaction".to_string())
            }
            CreateTransactionError::TransactionExists { transaction_id } => {
                Self::UnprocessableEntity(format!(
                    "Transaction with ID {} already exists",
                    transaction_id
                ))
            }
            CreateTransactionError::InsufficientFunds => {
                Self::UnprocessableEntity("Insufficient funds for this transaction".to_string())
            }
            CreateTransactionError::CanisterCommunicationError(err) => Self::InternalServerError(
                format!("Failed to communicate with the canister: {}", err),
            ),
            CreateTransactionError::CanisterRejectedError(err) => {
                Self::UnprocessableEntity(format!("Could not reserver car: {}", err))
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use ApiError::*;

        match self {
            InternalServerError(e) => {
                tracing::error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponseBody::new_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_string(),
                    )),
                )
                    .into_response()
            }
            UnprocessableEntity(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    message,
                )),
            )
                .into_response(),
            Conflict(message) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponseBody::new_error(StatusCode::CONFLICT, message)),
            )
                .into_response(),
        }
    }
}

/// Generic response structure shared by all API responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiResponseBody<T: Serialize + PartialEq> {
    status_code: u16,
    data: T,
}

impl<T: Serialize + PartialEq> ApiResponseBody<T> {
    pub fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}

impl ApiResponseBody<ApiErrorData> {
    pub fn new_error(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data: ApiErrorData { message },
        }
    }
}

/// The response data format for all error responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorData {
    pub message: String,
}

/// The body of a [Transaction] creation request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateTransactionHttpRequestBody {
    pub name: String,
    pub email_address: String,
    pub pan: String,
    pub age: u8,
    pub car_id: u64,
    pub aadhar: u64,
    pub country_code: u16,
    pub mobile_number: String,
    pub principal_jwk: String,
    pub start_time: u64,
    pub end_time: u64,
}

#[derive(Debug, Clone, Error)]
pub enum ParseCreateTransactionHttpRequestError {
    #[error(transparent)]
    Name(#[from] UserNameEmptyError),
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressError),
    #[error(transparent)]
    Pan(#[from] PANError),
    #[error(transparent)]
    Age(#[from] AgeError),
    #[error(transparent)]
    StartTime(#[from] StartTimeError),
    #[error(transparent)]
    EndTime(#[from] EndTimeError),
    #[error(transparent)]
    Aadhar(#[from] AadharError),
    #[error(transparent)]
    MobileNumber(#[from] MobileNumberError),
}

impl CreateTransactionHttpRequestBody {
    /// Converts the HTTP request body into a domain request.
    pub fn try_into_domain(
        self,
    ) -> Result<CreateTransactionRequest, ParseCreateTransactionHttpRequestError> {
        let name = UserName::new(&self.name)?;
        let email = EmailAddress::new(&self.email_address)?;
        let pan = PAN::new(&self.pan)?;
        let age = Age::new(self.age)?;
        let aadhar = Aadhar::new(&self.aadhar.to_string())?;
        let mobile_number =
            MobileNumber::new(self.mobile_number.parse().map_err(|_| MobileNumberError {
                invalid_mobile_number: self.mobile_number.clone(),
            })?)?;

        Ok(CreateTransactionRequest::new(
            name,
            email,
            age,
            pan,
            aadhar,
            mobile_number,
            self.country_code,
            self.car_id,
            StartTime::new(self.start_time)?,
            EndTime::new(self.end_time, self.start_time)?,
            self.principal_jwk,
        ))
    }
}

impl From<ParseCreateTransactionHttpRequestError> for ApiError {
    fn from(e: ParseCreateTransactionHttpRequestError) -> Self {
        match e {
            ParseCreateTransactionHttpRequestError::Name(user_name_empty_error) => {
                Self::UnprocessableEntity(format!(
                    "Username cannot be empty: {}",
                    user_name_empty_error
                ))
            }
            ParseCreateTransactionHttpRequestError::EmailAddress(email_error) => {
                Self::UnprocessableEntity(format!(
                    "Invalid email address: {}",
                    email_error.invalid_email
                ))
            }
            ParseCreateTransactionHttpRequestError::Pan(pan_error) => {
                Self::UnprocessableEntity(format!("Invalid PAN: {}", pan_error.invalid_pan))
            }
            ParseCreateTransactionHttpRequestError::Age(age_error) => {
                Self::UnprocessableEntity(format!("Invalid age: {}", age_error.invalid_age))
            }
            ParseCreateTransactionHttpRequestError::Aadhar(aadhar_error) => {
                Self::UnprocessableEntity(format!(
                    "Invalid Aadhar: {}",
                    aadhar_error.invalid_aadhar
                ))
            }
            ParseCreateTransactionHttpRequestError::MobileNumber(mobile_error) => {
                Self::UnprocessableEntity(format!(
                    "Invalid mobile number: {}",
                    mobile_error.invalid_mobile_number
                ))
            }
            ParseCreateTransactionHttpRequestError::StartTime(start_time_error) => {
                Self::UnprocessableEntity(format!(
                    "Invalid start time: {}",
                    start_time_error.start_time
                ))
            }
            ParseCreateTransactionHttpRequestError::EndTime(end_time_error) => {
                Self::UnprocessableEntity(format!("Invalid end time: {}", end_time_error.end_time))
            } // Add more cases if you have additional validation errors to handle
        }
    }
}

/// The response body data field for successful [Transaction] creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateTransactionResponseData {
    id: u64,
}

impl From<&Transaction> for CreateTransactionResponseData {
    fn from(transaction: &Transaction) -> Self {
        Self {
            id: transaction.booking_id(),
        }
    }
}

/// Create a new [Transaction].
///
/// # Responses
///
/// - 201 Created: the [Transaction] was successfully created.
/// - 422 Unprocessable entity: A [Transaction] with invalid data was provided.
pub async fn create_transaction<TS: TransactionService>(
    State(state): State<AppState<TS>>,
    Json(body): Json<CreateTransactionHttpRequestBody>,
) -> Result<ApiSuccess<CreateTransactionResponseData>, ApiError> {
    // Validate input and convert to domain request
    let domain_req = body.try_into_domain()?;

    state
        .transaction_service
        .create_transaction(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref transaction| ApiSuccess::new(StatusCode::CREATED, transaction.into()))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::anyhow;
    // use uuid::Uuid;

    use crate::domain::transactions::models::transaction::{CreateTransactionRequest, Transaction};
    use crate::domain::transactions::ports::TransactionService;

    use super::*;

    #[derive(Clone)]
    struct MockTransactionService {
        create_transaction_result:
            Arc<std::sync::Mutex<Result<Transaction, CreateTransactionError>>>,
    }

    impl TransactionService for MockTransactionService {
        async fn create_transaction(
            &self,
            _: &CreateTransactionRequest,
        ) -> Result<Transaction, CreateTransactionError> {
            let mut guard = self.create_transaction_result.lock();
            let mut result = Err(CreateTransactionError::Unknown(anyhow!("substitute error")));
            std::mem::swap(guard.as_deref_mut().unwrap(), &mut result);
            result
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_transaction_success() {
        let booking_id = 1;
        let car_id = 101;
        let transaction_id = 1;

        let service = MockTransactionService {
            create_transaction_result: Arc::new(std::sync::Mutex::new(Ok(Transaction::new(
                booking_id,
                car_id,
                UserName::new("Test User").unwrap(),
                EmailAddress::new("test@example.com").unwrap(),
                Age::new(25).unwrap(),
                91,
                MobileNumber::new(9876543210).unwrap(),
                PAN::new("ABCDE1234F").unwrap(),
                Aadhar::new("123456789012").unwrap(),
                1734556800, 1734564000
            )))),
        };

        let state = axum::extract::State(AppState {
            transaction_service: Arc::new(service),
        });

        let body = axum::extract::Json(CreateTransactionHttpRequestBody {
            name: "Test User".to_string(),
            email_address: "test@example.com".to_string(),
            pan: "ABCDE1234F".to_string(),
            age: 25,
            car_id,
            aadhar: 123456789012,
            country_code: 91,
            mobile_number: "9876543210".to_string(),
            principal_jwk: String::new(),
            start_time: 1734556800, // Example epoch time for 2024-12-18 00:00:00 UTC
            end_time: 1734564000,
        });

        let expected = ApiSuccess::new(
            StatusCode::CREATED,
            CreateTransactionResponseData {
                id: transaction_id,
            },
        );

        let actual = create_transaction(state, body).await;

        assert!(
            actual.is_ok(),
            "expected create_transaction to succeed, but got {:?}",
            actual
        );

        let actual = actual.unwrap();
        assert_eq!(
            actual, expected,
            "expected ApiSuccess {:?}, but got {:?}",
            expected, actual
        );
    }
}
