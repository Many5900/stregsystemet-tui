use serde::{Deserialize, Serialize};

pub use crate::models::sale::Sale;
use crate::utils::money::Money;

#[derive(Debug, Deserialize, Clone)]
pub struct MemberId {
    pub member_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MemberInfo {
    pub balance: Money,
    pub username: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SalesResponse {
    pub sales: Vec<Sale>,
}
