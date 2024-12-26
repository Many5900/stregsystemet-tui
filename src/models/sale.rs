use crate::utils::money::Money;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Sale {
    pub timestamp: String,
    pub product: String,
    pub price: Money,
}

impl Sale {
    pub fn formatted_timestamp(&self) -> String {
        chrono::DateTime::parse_from_rfc3339(&self.timestamp)
            .map(|dt| dt.format("%d/%m/%Y %H:%M").to_string())
            .unwrap_or_else(|_| "Invalid date".to_string())
    }
}
