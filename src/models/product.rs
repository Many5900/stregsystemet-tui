use crate::utils::money::Money;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Product {
    #[serde(skip)]
    pub id: String,

    pub name: String,
    pub price: Money,
}
