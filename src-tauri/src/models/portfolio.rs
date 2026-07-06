use serde::{Deserialize, Serialize};
use crate::utils::now;
use ulid::Ulid;
use thiserror::Error;
use sqlx::FromRow;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Portfolio {
    pub id: String,
    pub name: String,
    pub base_currency: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PortfolioError {
    #[error("Portfolio Name cannot be empty.")]
    InvalidName,
    #[error("Currency must be a 3-letter ISO code.")]
    InvalidCurrency,
}

impl Portfolio {
    fn touch(&mut self) {
        self.updated_at = now();
    }

    fn validate_name(name: &str) -> Result<(), PortfolioError> {
        if name.is_empty() {
            return Err(PortfolioError::InvalidName);
        };

        Ok(())
    }

    fn validate_currency(currency: &str) -> Result<(), PortfolioError> {
        if currency.len() != 3 || !currency.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(PortfolioError::InvalidCurrency);
        }

        Ok(())
    }

    pub fn new (name: &str, base_currency: &str) -> Result<Self, PortfolioError> {

        let name = name.trim();
        let base_currency = base_currency.trim().to_uppercase();

        Self::validate_name(name)?;
        Self::validate_currency(&base_currency)?;

        let time_stamp = now();

        Ok(Self { id: Ulid::new().to_string(),
            name: name.to_owned(),
            base_currency: base_currency,
            created_at: time_stamp.clone(),
            updated_at: time_stamp })
    }

    pub fn rename(&mut self,name: &str) -> Result<(), PortfolioError> {
        let name = name.trim();
        Self::validate_name(name)?;

        self.name = name.to_owned();
        self.touch();

        Ok(())
    }

    pub fn change_base_currency(&mut self, base_currency: &str) -> Result<(), PortfolioError> {
        let base_currency = base_currency.trim().to_uppercase();

        Self::validate_currency(&base_currency)?;

        self.base_currency = base_currency;
        self.touch();

        Ok(())
    }

}