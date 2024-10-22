use std::fmt::{Display, Formatter};
use derive_more::From;
use regex::Regex;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transaction {
    booking_id: u64, 
    car_id: u64,
    name: UserName,
    email: EmailAddress,
    age: Age, 
    country_code: u16,
    mobile_number: MobileNumber,
    pan: PAN,
    aadhar: Aadhar,
    start_time: u64, 
    end_time: u64
}


impl Transaction {
    // Constructor: Creates a new Transaction
    pub fn new(
        booking_id: u64,
        car_id: u64,
        name: UserName,
        email: EmailAddress,
        age: Age,
        country_code: u16,
        mobile_number: MobileNumber,
        pan: PAN,
        aadhar: Aadhar,
        start_time: u64,
        end_time: u64
    ) -> Self {
        Self {
            booking_id,
            car_id,
            name,
            email,
            age,
            country_code,
            mobile_number,
            pan,
            aadhar,
            start_time,
            end_time
        }
    }

    // Getter for booking_id
    pub fn booking_id(&self) -> u64 {
        self.booking_id
    }

    // Getter for car_id
    pub fn car_id(&self) -> u64 {
        self.car_id
    }

    // Getter for name
    pub fn name(&self) -> &UserName {
        &self.name
    }

    // Getter for email
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    // Getter for age
    pub fn age(&self) -> &Age {
        &self.age
    }

    // Getter for country_code
    pub fn country_code(&self) -> u16 {
        self.country_code
    }
    // Getter for country_code
    pub fn start_time(&self) -> u64 {
        self.start_time
    }
    // Getter for country_code
    pub fn end_time(&self) -> u64 {
        self.end_time
    }

    // Getter for mobile_number
    pub fn mobile_number(&self) -> &MobileNumber {
        &self.mobile_number
    }

    // Getter for PAN
    pub fn pan(&self) -> &PAN {
        &self.pan
    }

    // Getter for Aadhar
    pub fn aadhar(&self) -> &Aadhar {
        &self.aadhar
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserName(pub String);

#[derive(Clone, Debug, Error)]
#[error("User name cannot be empty")]
pub struct UserNameEmptyError;

impl UserName {
    pub fn new(raw: &str) -> Result<Self, UserNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(UserNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(pub String);

#[derive(Clone, Debug, Error)]
#[error("{invalid_email} is not a valid email address")]
pub struct EmailAddressError {
    pub invalid_email: String,
}

impl EmailAddress {
    pub fn new(raw: &str) -> Result<Self, EmailAddressError> {
        let trimmed = raw.trim();
        Self::validate_email_address(trimmed)?;
        Ok(Self(trimmed.to_string()))
    }

    fn validate_email_address(email: &str) -> Result<(), EmailAddressError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if email_regex.is_match(email) {
            Ok(())
        } else {
            Err(EmailAddressError {
                invalid_email: email.to_string(),
            })
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Age(u8);

#[derive(Clone, Debug, Error)]
#[error("{invalid_age} is not a valid age for a driver")]
pub struct AgeError {
    pub invalid_age: u8,
}

impl Age {
    pub fn new(age: u8) -> Result<Self, AgeError> {
        if age < 18 {
            Err(AgeError { invalid_age: age })
        } else {
            Ok(Self(age))
        }
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MobileNumber(u64);

#[derive(Clone, Debug, Error)]
#[error("{invalid_mobile_number} is not a valid mobile number")]
pub struct MobileNumberError {
    pub invalid_mobile_number: String,
}

impl MobileNumber {
    pub fn new(mobile_number: u64) -> Result<Self, MobileNumberError> {
        if mobile_number.to_string().len() != 10 {
            Err(MobileNumberError { invalid_mobile_number: mobile_number.to_string() })
        } else {
            Ok(Self(mobile_number))
        }
    }
}

impl Display for MobileNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

// PAN field and validation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PAN(String);

#[derive(Clone, Debug, Error)]
#[error("{invalid_pan} is not a valid PAN")]
pub struct PANError {
    pub invalid_pan: String,
}

impl PAN {
    pub fn new(pan: &str) -> Result<Self, PANError> {
        let trimmed = pan.trim();
        if PAN::validate_pan(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err(PANError {
                invalid_pan: trimmed.to_string(),
            })
        }
    }

    fn validate_pan(pan: &str) -> bool {
        let pan_regex = regex::Regex::new(r"^[A-Z]{5}[0-9]{4}[A-Z]{1}$").unwrap(); // PAN format
        pan_regex.is_match(pan)
    }
}

impl Display for PAN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// Aadhar field and validation
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Aadhar(String);

#[derive(Clone, Debug, Error)]
#[error("{invalid_aadhar} is not a valid Aadhar")]
pub struct AadharError {
    pub invalid_aadhar: String,
}

impl Aadhar {
    pub fn new(aadhar: &str) -> Result<Self, AadharError> {
        let trimmed = aadhar.trim();
        if Aadhar::validate_aadhar(trimmed) {
            Ok(Self(trimmed.to_string()))
        } else {
            Err(AadharError {
                invalid_aadhar: trimmed.to_string(),
            })
        }
    }

    fn validate_aadhar(aadhar: &str) -> bool {
        let aadhar_regex = regex::Regex::new(r"^\d{12}$").unwrap(); // Aadhar must be a 12-digit number
        aadhar_regex.is_match(aadhar)
    }
}

impl Display for Aadhar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From,)]
pub struct CreateTransactionRequest {
    name: UserName,
    email: EmailAddress,
    age: Age,
    pan: PAN,
    aadhar: Aadhar,
    mobile_number: MobileNumber,
    country_code: u16,
    // principal: Principal,   // Principal for ICP
    car_id: u64,            // Car ID for the transaction
    start_time: StartTime,   // Start time (validated)
    end_time: EndTime,
    principal_jwk: String 
}

impl CreateTransactionRequest {
    // Constructor
    pub fn new(
        name: UserName,
        email: EmailAddress,
        age: Age,
        pan: PAN,
        aadhar: Aadhar,
        mobile_number: MobileNumber,
        country_code: u16,
        // principal: Principal,
        car_id: u64,
        start_time: StartTime,
        end_time: EndTime,
        principal_jwk: String,
    ) -> Self {
        Self {
            name,
            email,
            age,
            pan,
            aadhar,
            mobile_number,
            country_code,
            // principal,
            car_id,
            start_time,
            end_time,
            principal_jwk,
        }
    }

    // Getter for name
    pub fn name(&self) -> &UserName {
        &self.name
    }

    // Getter for email
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    // Getter for age
    pub fn age(&self) -> &Age {
        &self.age
    }

    // Getter for PAN
    pub fn pan(&self) -> &PAN {
        &self.pan
    }

    // Getter for Aadhar
    pub fn aadhar(&self) -> &Aadhar {
        &self.aadhar
    }

    // Getter for mobile_number
    pub fn mobile_number(&self) -> &MobileNumber {
        &self.mobile_number
    }

    // Getter for country_code
    pub fn country_code(&self) -> u16 {
        self.country_code
    }

    // Getter for Principal
    // pub fn principal(&self) -> &Principal {
    //     &self.principal
    // }

    // Getter for car_id
    pub fn car_id(&self) -> u64 {
        self.car_id
    }

    // Getter for start_time
    pub fn start_time(&self) -> u64 {
        self.start_time.0
    }

    // Getter for end_time
    pub fn end_time(&self) -> u64 {
        self.end_time.0
    }

    pub fn secret(&self) -> k256::SecretKey {
        println!("{:?}", &self.principal_jwk);
        k256::SecretKey::from_jwk_str(&self.principal_jwk).unwrap()
    }

    pub fn customer(&self) -> Customer {
        Customer { age: self.age.clone().0, pan: self.pan.clone().0, mobile_number: self.mobile_number.0.to_string(), name: self.name.0.clone(), email: self.email.0.clone(), country_code: self.country_code.to_string(), aadhar: self.aadhar.0.to_string() }
    }

}

use std::time::{SystemTime, UNIX_EPOCH};

use crate::canister::backend::Customer;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartTime(u64);

#[derive(Clone, Debug, Error)]
#[error("Invalid start time: {start_time}, it must be greater than the current time {now}")]
pub struct StartTimeError {
    pub start_time: u64,
    pub now: u64,
}

impl StartTime {
    pub fn new(start_time: u64) -> Result<Self, StartTimeError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        if start_time <= now {
            Err(StartTimeError { start_time, now })
        } else {
            Ok(Self(start_time))
        }
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndTime(u64);

#[derive(Clone, Debug, Error)]
#[error("Invalid end time: {end_time}, it must be greater than start time {start_time}")]
pub struct EndTimeError {
    pub start_time: u64,
    pub end_time: u64,
}

impl EndTime {
    pub fn new(end_time: u64, start_time: u64) -> Result<Self, EndTimeError> {
        if end_time <= start_time {
            Err(EndTimeError { start_time, end_time })
        } else {
            Ok(Self(end_time))
        }
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}



/// Errors that may occur while creating a transaction.
#[derive(Debug, Error)]
pub enum CreateTransactionError {
    #[error(transparent)]
    InvalidAge(#[from] AgeError), // For invalid age


    #[error(transparent)]
    InvalidPAN(#[from] PANError), // For invalid PAN

    #[error(transparent)]
    InvalidAadhar(#[from] AadharError), // For invalid Aadhar

    #[error(transparent)]
    UserNameEmpty(#[from] UserNameEmptyError), // For empty username

    #[error(transparent)]
    InvalidEmail(#[from] EmailAddressError), // For invalid email address

    #[error(transparent)]
    InvalidMobile(#[from] MobileNumberError), // For invalid mobile

    #[error("Start time must be greater than the current time")]
    StartTimeError, // When the start time is not valid

    #[error("End time must be greater than start time")]
    EndTimeError, // When the end time is not valid

    #[error("Transaction already exists with ID: {transaction_id}")]
    TransactionExists { transaction_id: String }, // When a transaction with the same ID already exists.

    #[error("Insufficient funds for the transaction")]
    InsufficientFunds, // When the user's account has insufficient balance for the transaction.

    #[error("Failed to communicate with the canister: {0}")]
    CanisterCommunicationError(String), // Errors related to network or canister communication.

    #[error("Canister Response: {0}")]
    CanisterRejectedError(String), // For any other unknown errors.


    #[error(transparent)]
    Unknown(#[from] anyhow::Error), // For any other unknown errors.
}
