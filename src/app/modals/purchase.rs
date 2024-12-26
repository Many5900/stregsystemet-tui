use crate::app::modals::error::ErrorModalActions;
use crate::app::state::{AppState, InputMode};
use crate::error::Result;

pub trait PurchaseModalActions {
    fn show_purchase_modal(&mut self) -> Result<()>;

    fn hide_purchase_modal(&mut self);

    fn increase_quantity(&mut self);

    fn decrease_quantity(&mut self);
}

impl PurchaseModalActions for AppState {
    fn show_purchase_modal(&mut self) -> Result<()> {
        if let Err(message) = self.validate_user_for_purchase() {
            use crate::utils::formatters::format_error_message;
            let formatted_msg = format_error_message(&message, 50, 10);

            self.show_error_modal(&formatted_msg, Some("Invalid User"));
            return Ok(());
        }

        if let Some(i) = self.products.list_state.selected() {
            let products = self.get_sorted_products();

            if i < products.len() {
                let product = products[i];
                self.modals.purchase.selected_product_id = Some(product.id.clone());
                self.modals.purchase.visible = true;
                self.push_input_mode(InputMode::BuyConfirmation);
                self.modals.purchase.error = None;
                self.modals.purchase.success = false;
            }
        }
        Ok(())
    }

    fn hide_purchase_modal(&mut self) {
        self.modals.purchase.visible = false;
        self.pop_input_mode();
        self.modals.purchase.selected_product_id = None;
        self.modals.purchase.quantity = 1;
    }

    fn increase_quantity(&mut self) {
        if self.modals.purchase.quantity < 99 {
            self.modals.purchase.quantity += 1;
        }
    }

    fn decrease_quantity(&mut self) {
        if self.modals.purchase.quantity > 1 {
            self.modals.purchase.quantity -= 1;
        }
    }
}
