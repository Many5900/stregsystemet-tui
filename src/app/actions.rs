use crate::api::client::ApiClient;
use crate::app::modals::error::ErrorModalActions;
use crate::app::state::AppState;
use crate::error::Result;
use crate::utils::formatters::format_error_message;
use crate::utils::money::Money;

pub struct ActionHandler {
    state: AppState,
    api_client: ApiClient,
}

impl ActionHandler {
    pub fn new(state: AppState, api_client: ApiClient) -> Self {
        Self { state, api_client }
    }

    pub fn get_state(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub async fn load_app_data(&mut self) -> Result<()> {
        match self.api_client.fetch_products().await {
            Ok(products) => {
                self.state.products.items = products;
                self.state.products.error = None;
            }
            Err(e) => {
                self.state.products.error = Some(e.to_string());
            }
        }

        match self.api_client.fetch_named_products().await {
            Ok(named_products) => {
                self.state.products.named_products = named_products;
                self.state.products.named_products_error = None;
            }
            Err(e) => {
                self.state.products.named_products_error = Some(e.to_string());
            }
        }

        if self.state.config.username.is_some() {
            match self.load_user_data().await {
                Ok(_) => {}
                Err(e) => {
                    self.state.user.error = Some(format!("Failed to load user data: {e}"));
                }
            }
        }

        Ok(())
    }

    pub async fn load_user_data(&mut self) -> Result<()> {
        if let Some(ref username) = self.state.config.username {
            self.state.user.error = None;
            self.state.user.member_info = None;
            self.state.user.member_id = None;
            self.state.user.latest_sales = Vec::new();

            match self.api_client.fetch_member_id(username).await? {
                Some(member_id) => {
                    self.state.user.member_id = Some(member_id);

                    let member_info_future = self.api_client.fetch_member_info(member_id);
                    let sales_future = self.api_client.fetch_latest_sales(member_id);

                    let (member_info_result, sales_result) =
                        tokio::join!(member_info_future, sales_future);

                    match member_info_result {
                        Ok(info) => {
                            self.state.user.member_info = Some(info);
                        }
                        Err(e) => {
                            self.state.user.error =
                                Some(format!("Failed to fetch member info: {e}"));
                        }
                    }

                    match sales_result {
                        Ok(sales) => {
                            self.state.user.latest_sales = sales;
                        }
                        Err(e) => {
                            if self.state.user.error.is_none() {
                                self.state.user.error = Some(format!("Failed to fetch sales: {e}"));
                            }
                        }
                    }
                }
                None => {
                    self.state.user.error = Some(format!("Username '{username}' does not exist"));
                }
            }
        }

        Ok(())
    }

    pub async fn process_purchase(&mut self) -> Result<()> {
        if let Err(message) = self.state.validate_user_for_purchase() {
            let formatted_msg = format_error_message(&message, 50, 10);

            self.state
                .show_error_modal(&formatted_msg, Some("Invalid User"));
            return Ok(());
        }

        if !self.has_sufficient_balance() {
            if let Some(total_cost) = self.get_total_cost() {
                self.state.modals.purchase.error = Some(format!(
                    "Insufficient balance. This purchase requires {total_cost}"
                ));
            } else {
                self.state.modals.purchase.error =
                    Some("Insufficient balance for this purchase".to_string());
            }
            return Ok(());
        }

        if let (Some(member_id), Some(buystring)) =
            (self.state.user.member_id, self.get_buy_string())
        {
            self.state.modals.purchase.error = None;
            self.state.modals.purchase.success = false;

            match self.api_client.make_purchase(member_id, &buystring).await {
                Ok(_) => {
                    self.state.modals.purchase.success = true;

                    let _ = self.load_user_data().await;
                }
                Err(e) => {
                    self.state.modals.purchase.error = Some(format!("Purchase failed: {e}"));
                }
            }
        } else {
            use crate::utils::formatters::format_error_message;
            let msg = format_error_message(
                "Unable to process purchase: missing required information",
                50,
                5,
            );
            self.state.show_error_modal(&msg, Some("Purchase Error"));
        }

        Ok(())
    }

    pub fn update_search_results(&mut self) {
        let query = self.state.modals.search.input.trim().to_lowercase();

        if query.is_empty() {
            self.state.modals.search.results.clear();
            return;
        }

        let mut results = Vec::new();

        if let Ok(id) = query.parse::<i32>() {
            let id_str = id.to_string();
            if let Some(product) = self.state.products.items.get(&id_str) {
                results.push(product.clone());
            }
        }

        if query.chars().all(|c| c.is_ascii_digit()) && results.is_empty() {
            for (id, product) in &self.state.products.items {
                if id.starts_with(&query) {
                    results.push(product.clone());
                }
            }
        }

        if results.len() < 10 {
            let mut named_matches = Vec::new();

            for (search_term, product_id) in &self.state.products.named_products {
                let search_term_lower = search_term.to_lowercase();

                if search_term_lower.contains(&query) {
                    let id_str = product_id.to_string();

                    if let Some(product) = self.state.products.items.get(&id_str) {
                        if !results
                            .iter()
                            .any(|result_product| result_product.id == id_str)
                        {
                            let relevance_score = if search_term_lower == query {
                                1000
                            } else if search_term_lower.starts_with(&query) {
                                800
                            } else {
                                500 - (search_term_lower.len() as i32 - query.len() as i32).abs()
                            };

                            named_matches.push((id_str.clone(), product.clone(), relevance_score));
                        }
                    }
                }
            }

            named_matches.sort_by(|a, b| b.2.cmp(&a.2));

            for (id, product, _) in named_matches {
                if !results.iter().any(|result_product| result_product.id == id) {
                    results.push(product);

                    if results.len() >= 10 {
                        break;
                    }
                }
            }
        }

        if results.len() < 10 {
            let mut name_matches = Vec::new();

            for (id, product) in &self.state.products.items {
                let product_name = product.name.to_lowercase();

                if product_name.contains(&query)
                    && !results
                        .iter()
                        .any(|result_product| result_product.id == *id)
                {
                    let relevance_score = if product_name.starts_with(&query) {
                        700
                    } else {
                        400 - (product_name.len() as i32 - query.len() as i32).abs()
                    };

                    name_matches.push((id.clone(), product.clone(), relevance_score));
                }
            }

            name_matches.sort_by(|a, b| b.2.cmp(&a.2));

            for (id, product, _) in name_matches {
                if !results.iter().any(|result_product| result_product.id == id) {
                    results.push(product);

                    if results.len() >= 10 {
                        break;
                    }
                }
            }
        }

        results.truncate(10);

        self.state.modals.search.results = results;

        if !self.state.modals.search.results.is_empty() {
            self.state.modals.search.selected_index = 0;
        }
    }

    fn has_sufficient_balance(&self) -> bool {
        if let (Some(member_info), Some(total_cost)) =
            (&self.state.user.member_info, self.get_total_cost())
        {
            return member_info.balance >= total_cost;
        }
        false
    }

    fn get_total_cost(&self) -> Option<Money> {
        if let Some(product_id) = &self.state.modals.purchase.selected_product_id {
            if let Some(product) = self.state.products.items.get(product_id) {
                return Some(product.price * self.state.modals.purchase.quantity);
            }
        }
        None
    }

    fn get_buy_string(&self) -> Option<String> {
        if let (Some(member_info), Some(product_id)) = (
            &self.state.user.member_info,
            &self.state.modals.purchase.selected_product_id,
        ) {
            return Some(format!(
                "{} {}:{}",
                member_info.username, product_id, self.state.modals.purchase.quantity
            ));
        }
        None
    }

    pub async fn process_parking_registration(
        &mut self,
        plate: &str,
        phone_number: &str,
    ) -> Result<()> {
        match self.api_client.register_parking(plate, phone_number).await {
            Ok(_) => {
                self.state.modals.parking.success = true;
                self.state.modals.parking.error = None;
            }
            Err(e) => {
                self.state.modals.parking.success = false;
                self.state.modals.parking.error = Some(format!("Failed to register parking: {e}"));
            }
        }
        Ok(())
    }
}
