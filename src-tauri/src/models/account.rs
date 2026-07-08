use crate::utils::now;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use ulid::Ulid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccountType {
    Brokerage,
    Chequing,
    Cash,
    Savings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub portfolio_id: String,
    pub name: String,
    pub institution: Option<String>,
    pub account_type: AccountType,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    InvalidName,
    InvalidCurrency,
}

impl Account {
    fn touch(&mut self) {
        self.updated_at = now();
    }

    fn validate_name(name: &str) -> Result<(), AccountError> {
        if name.trim().is_empty() {
            return Err(AccountError::InvalidName);
        };

        Ok(())
    }

    fn validate_currency(currency: &str) -> Result<(), AccountError> {
        if currency.len() != 3 || !currency.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(AccountError::InvalidCurrency);
        }

        Ok(())
    }

    pub fn new(
        portfolio_id: &str,
        name: &str,
        institution: Option<&str>,
        account_type: AccountType,
        currency: &str,
    ) -> Result<Self, AccountError> {
        let name = name.trim();
        let currency = currency.trim().to_uppercase();
        let institution = institution
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);

        Self::validate_name(name)?;
        Self::validate_currency(&currency)?;

        let time_stamp = now();

        Ok(Self {
            id: Ulid::new().to_string(),
            portfolio_id: portfolio_id.to_owned(),
            name: name.to_owned(),
            institution,
            account_type,
            currency,
            is_active: true,
            created_at: time_stamp,
            updated_at: time_stamp,
        })
    }

    pub fn rename(&mut self, name: &str) -> Result<(), AccountError> {
        let name = name.trim();
        Self::validate_name(name)?;

        self.name = name.to_owned();
        self.touch();

        Ok(())
    }

    pub fn change_currency(&mut self, currency: &str) -> Result<(), AccountError> {
        let currency = currency.trim().to_uppercase();

        Self::validate_currency(&currency)?;

        self.currency = currency;
        self.touch();

        Ok(())
    }

    pub fn change_institution(&mut self, institution: Option<&str>) {
        let institution = institution
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned);

        self.institution = institution;
        self.touch();
    }

    pub fn change_account_type(&mut self, account_type: AccountType) {
        self.account_type = account_type;
        self.touch();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.touch();
    }

    pub fn activate(&mut self) {
        self.is_active = true;
        self.touch();
    }
}
