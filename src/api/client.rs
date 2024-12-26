use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

use crate::api::endpoints::{
    ACTIVE_PRODUCTS_ENDPOINT, MEMBER_ID_ENDPOINT, MEMBER_INFO_ENDPOINT, NAMED_PRODUCTS_ENDPOINT,
    PURCHASE_ENDPOINT, SALES_ENDPOINT,
};
use crate::config::settings::Config;
use crate::error::{AppError, Result};
use crate::models::member::{MemberId, MemberInfo, SalesResponse};
use crate::models::product::Product;
use crate::models::sale::Sale;
use crate::utils::formatters::sanitize_html;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    api_url: String,
    room_id: u32,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_url: Config::get_api_url(),
            room_id: config.room_id,
        }
    }

    pub async fn fetch_products(&self) -> Result<HashMap<String, Product>> {
        let url = format!(
            "{}{}",
            self.api_url,
            ACTIVE_PRODUCTS_ENDPOINT.replace("{room_id}", &self.room_id.to_string())
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to fetch products: HTTP status {}",
                response.status()
            )));
        }

        let mut products: HashMap<String, Product> = response.json().await?;

        for (product_id, product) in products.iter_mut() {
            product.name = sanitize_html(&product.name);
            product.id = product_id.clone();
        }

        Ok(products)
    }

    pub async fn fetch_named_products(&self) -> Result<HashMap<String, i32>> {
        let url = format!("{}{}", self.api_url, NAMED_PRODUCTS_ENDPOINT);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to fetch named products: HTTP status {}",
                response.status()
            )));
        }

        let named_products: HashMap<String, i32> = response.json().await?;
        Ok(named_products)
    }

    pub async fn fetch_member_id(&self, username: &str) -> Result<Option<i32>> {
        let url = format!(
            "{}{}",
            self.api_url,
            MEMBER_ID_ENDPOINT.replace("{username}", username)
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to fetch member ID: HTTP status {}",
                response.status()
            )));
        }

        let member_id_response: MemberId = response.json().await?;
        Ok(Some(member_id_response.member_id))
    }

    pub async fn fetch_member_info(&self, member_id: i32) -> Result<MemberInfo> {
        let url = format!(
            "{}{}",
            self.api_url,
            MEMBER_INFO_ENDPOINT.replace("{member_id}", &member_id.to_string())
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to fetch member info: HTTP status {}",
                response.status()
            )));
        }

        let member_info: MemberInfo = response.json().await?;
        Ok(member_info)
    }

    pub async fn fetch_latest_sales(&self, member_id: i32) -> Result<Vec<Sale>> {
        let url = format!(
            "{}{}",
            self.api_url,
            SALES_ENDPOINT.replace("{member_id}", &member_id.to_string())
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to fetch sales: HTTP status {}",
                response.status()
            )));
        }

        let sales_response: SalesResponse = response.json().await?;
        Ok(sales_response.sales)
    }

    pub async fn make_purchase(&self, member_id: i32, buystring: &str) -> Result<()> {
        let url = format!("{}{}", self.api_url, PURCHASE_ENDPOINT);

        let body = json!({
            "member_id": member_id,
            "buystring": buystring,
            "room": self.room_id
        });

        let response = self.client.post(&url).json(&body).send().await?;

        if !response.status().is_success() {
            return Err(AppError::Api(format!(
                "Failed to make purchase: HTTP status {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn register_parking(&self, plate: &str, phone_number: &str) -> Result<()> {
        let payload = json!({
            "email": "",
            "PhoneNumber": format!("45{}", phone_number),
            "VehicleRegistrationCountry": "DK",
            "Duration": 600,
            "VehicleRegistration": plate,
            "parkingAreas": [
                {
                    "ParkingAreaId": 1956,
                    "ParkingAreaKey": "ADK-4688"
                }
            ],
            "UId": "12cdf204-d969-469a-9bd5-c1f1fc59ee34",
            "Lang": "da"
        });

        let response = self
            .client
            .post("https://api.mobile-parking.eu/v10/permit/Tablet/confirm")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Api(format!(
                "Parking registration failed: {status} - {error_text}"
            )));
        }

        Ok(())
    }
}
