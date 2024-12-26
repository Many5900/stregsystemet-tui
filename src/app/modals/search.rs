use crate::app::state::{AppState, InputMode};
use crate::models::product::Product;

pub trait SearchModalActions {
    fn show_search_modal(&mut self);

    fn hide_search_modal(&mut self);

    fn next_search_result(&mut self);

    fn previous_search_result(&mut self);

    fn get_selected_search_result(&self) -> Option<&Product>;

    fn select_product_from_search(&mut self);
}

impl SearchModalActions for AppState {
    fn show_search_modal(&mut self) {
        self.modals.search.visible = true;
        self.push_input_mode(InputMode::SearchMode);
        self.modals.search.input.clear();
        self.modals.search.results.clear();
        self.modals.search.selected_index = 0;
    }

    fn hide_search_modal(&mut self) {
        self.modals.search.visible = false;
        self.pop_input_mode();
        self.modals.search.input.clear();
        self.modals.search.results.clear();
    }

    fn next_search_result(&mut self) {
        if !self.modals.search.results.is_empty() {
            self.modals.search.selected_index =
                (self.modals.search.selected_index + 1) % self.modals.search.results.len();
        }
    }

    fn previous_search_result(&mut self) {
        if !self.modals.search.results.is_empty() {
            self.modals.search.selected_index = if self.modals.search.selected_index == 0 {
                self.modals.search.results.len() - 1
            } else {
                self.modals.search.selected_index - 1
            };
        }
    }

    fn get_selected_search_result(&self) -> Option<&Product> {
        if !self.modals.search.results.is_empty() {
            Some(&self.modals.search.results[self.modals.search.selected_index])
        } else {
            None
        }
    }

    fn select_product_from_search(&mut self) {
        if let Some(selected_product) = self.get_selected_search_result() {
            let product_id = selected_product.id.clone();

            let products_vec = self.get_sorted_products();

            if let Some(index) = products_vec
                .iter()
                .position(|product| product.id == product_id)
            {
                self.products.list_state.select(Some(index));
                self.hide_search_modal();
            }
        }
    }
}
