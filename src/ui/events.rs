use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::api::client::ApiClient;
use crate::app::actions::ActionHandler;
use crate::app::modals::{
    error::ErrorModalActions, parking::ParkingModalActions, purchase::PurchaseModalActions,
    search::SearchModalActions, terminal_size::TerminalSizeModalActions,
    username::UsernameModalActions,
};
use crate::app::state::{AppState, InputMode};
use crate::config::store::save_config;
use crate::error::Result;

pub struct EventHandler<'a> {
    terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
    state: AppState,
    action_handler: ActionHandler,
    shutdown_flag: Arc<Mutex<bool>>,
}

impl<'a> EventHandler<'a> {
    pub fn new(
        terminal: &'a mut Terminal<CrosstermBackend<io::Stdout>>,
        state: AppState,
        shutdown_flag: Arc<Mutex<bool>>,
    ) -> Self {
        let api_client = ApiClient::new(&state.config);
        let state_clone = state.clone();
        let action_handler = ActionHandler::new(state_clone, api_client);

        Self {
            terminal,
            state,
            action_handler,
            shutdown_flag,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        if self.state.config.username.is_none() {
            self.state.ui.input_mode = InputMode::Editing;
        }

        match self.action_handler.load_app_data().await {
            Ok(_) => {
                let action_state = self.action_handler.get_state();
                self.state.products = action_state.products.clone();
                self.state.user = action_state.user.clone();
            }
            Err(e) => {
                let action_state = self.action_handler.get_state();
                self.state.products = action_state.products.clone();

                self.state
                    .handle_invalid_username(&format!("Failed to load user data: {e}"));

                use crate::utils::formatters::format_error_message;
                let error_msg = format_error_message(
                    &format!("There was a problem with your username. Error: {e}."),
                    50,
                    10,
                );
                self.state
                    .show_error_modal(&error_msg, Some("User Data Error"));
            }
        };

        let (tx, mut rx) = mpsc::channel(100);

        let shutdown_flag = self.shutdown_flag.clone();

        let event_tx = tx.clone();
        let event_shutdown_flag = shutdown_flag.clone();
        tokio::spawn(async move {
            loop {
                if *event_shutdown_flag.lock().unwrap() {
                    break;
                }

                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Ok(event) = event::read() {
                        if event_tx.send(UIEvent::Input(event)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let clock_tx = tx.clone();
        let clock_shutdown_flag = shutdown_flag.clone();
        tokio::spawn(async move {
            loop {
                if *clock_shutdown_flag.lock().unwrap() {
                    break;
                }

                tokio::time::sleep(Duration::from_secs(1)).await;

                if clock_tx.send(UIEvent::ClockTick).await.is_err() {
                    break;
                }
            }
        });

        loop {
            self.terminal
                .draw(|f| crate::ui::render::ui(f, &self.state))?;

            let size = self.terminal.size()?;
            self.state.check_terminal_size(size.width, size.height);

            if let Some(UIEvent::Input(Event::Key(key))) = rx.recv().await {
                self.handle_key_event(key).await?;

                if self.state.should_quit {
                    let mut shutdown = self.shutdown_flag.lock().unwrap();
                    *shutdown = true;
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        match self.state.ui.input_mode {
            InputMode::Normal => self.handle_normal_mode(key).await?,
            InputMode::Editing => self.handle_editing_mode(key).await?,
            InputMode::EditingUsername => self.handle_username_editing(key).await?,
            InputMode::BuyConfirmation => self.handle_buy_confirmation(key).await?,
            InputMode::SearchMode => self.handle_search_mode(key).await?,
            InputMode::ErrorModal => self.handle_error_modal(key).await?,
            InputMode::ParkingModal => self.handle_parking_modal(key).await?,
            InputMode::ParkingConfirmation => self.handle_parking_confirmation(key).await?,
            InputMode::TerminalSizeModal => {
                if key.code == KeyCode::Char('q') {
                    self.state.should_quit = true;
                }
            }
        }

        Ok(())
    }

    async fn handle_normal_mode(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.state.should_quit = true;
            }
            KeyCode::Esc => {
                self.state.ui.number_prefix.clear();
                self.state.ui.pending_g = false;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let count = self.parse_and_clear_number_prefix();
                self.move_products_down(count);
                self.state.ui.pending_g = false;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let count = self.parse_and_clear_number_prefix();
                self.move_products_up(count);
                self.state.ui.pending_g = false;
            }
            KeyCode::Char('g') => {
                if self.state.ui.pending_g {
                    self.go_to_top();
                    self.state.ui.pending_g = false;
                } else {
                    self.state.ui.pending_g = true;
                }
            }
            KeyCode::Char('G') => {
                self.go_to_bottom();
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.state.ui.number_prefix.push(c);

                if self.state.get_movement_target_indices().is_empty() {
                    self.state.ui.number_prefix.clear();
                }

                self.state.ui.pending_g = false;
            }
            KeyCode::Char('u') => {
                if self.state.config.username.is_some() {
                    self.state.show_username_modal();
                }
            }
            KeyCode::Char('/') | KeyCode::Char('s') => {
                self.state.show_search_modal();
            }
            KeyCode::Char('p') => {
                self.state.show_parking_modal();
            }
            KeyCode::Enter => {
                if self.state.config.username.is_some() && !self.state.products.items.is_empty() {
                    self.state.show_purchase_modal()?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_editing_mode(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if !self.state.ui.input.trim().is_empty() {
                    self.state.config.username = Some(self.state.ui.input.trim().to_string());
                    save_config(&self.state.config)?;
                    self.state.ui.input_mode = InputMode::Normal;

                    self.state.user.member_id = None;
                    self.state.user.member_info = None;
                    self.state.user.latest_sales = Vec::new();
                    self.state.user.error = None;

                    self.action_handler.get_state().config.username =
                        self.state.config.username.clone();

                    match self.action_handler.load_user_data().await {
                        Ok(_) => {
                            let action_state = self.action_handler.get_state();
                            self.state.user = action_state.user.clone();
                        }
                        Err(e) => {
                            self.state
                                .handle_invalid_username(&format!("Error loading user data: {e}"));

                            use crate::utils::formatters::format_error_message;
                            let error_msg = format_error_message(
                                &format!("There was a problem with your username. Error: {e}"),
                                50,
                                10,
                            );
                            self.state
                                .show_error_modal(&error_msg, Some("Username Error"));
                        }
                    }
                }
            }
            KeyCode::Char(c) => {
                self.state.ui.input.push(c);
            }
            KeyCode::Backspace => {
                self.state.ui.input.pop();
            }
            KeyCode::Esc => {
                if self.state.config.username.is_some() {
                    self.state.ui.input_mode = InputMode::Normal;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_username_editing(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if let Err(e) = self.state.update_username() {
                    self.state.show_error_modal(
                        &format!("Error updating username: {e}"),
                        Some("Username Update Error"),
                    );
                } else {
                    let new_username = self.state.config.username.clone();
                    self.action_handler.get_state().config.username = new_username.clone();

                    self.state.user.member_id = None;
                    self.state.user.member_info = None;
                    self.state.user.latest_sales = Vec::new();
                    self.state.user.error = None;

                    let action_user_state = &mut self.action_handler.get_state().user;
                    action_user_state.member_id = None;
                    action_user_state.member_info = None;
                    action_user_state.latest_sales = Vec::new();
                    action_user_state.error = None;

                    match self.action_handler.load_user_data().await {
                        Ok(_) => {}
                        Err(e) => {
                            self.state.handle_invalid_username(&format!("Error: {e}"));

                            use crate::utils::formatters::format_error_message;
                            let error_msg = format_error_message(
                                &format!("Check that the username exists and try again!\n{e}"),
                                50,
                                10,
                            );
                            self.state
                                .show_error_modal(&error_msg, Some("Username Error"));
                        }
                    }

                    let action_state = self.action_handler.get_state();

                    if let Some(error_msg) = &action_state.user.error {
                        if error_msg.contains("not found") {
                            self.state.user.error = Some(format!(
                                "User '{}' not found",
                                new_username.unwrap_or(String::from("unknown"))
                            ));
                        } else {
                            self.state.user = action_state.user.clone();
                        }
                    } else {
                        self.state.user = action_state.user.clone();
                    }
                }
            }
            KeyCode::Char(c) => {
                self.state.modals.username.input.push(c);
            }
            KeyCode::Backspace => {
                self.state.modals.username.input.pop();
            }
            KeyCode::Esc => {
                self.state.hide_username_modal();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_buy_confirmation(&mut self, key: KeyEvent) -> Result<()> {
        if self.state.modals.purchase.success || self.state.modals.purchase.error.is_some() {
            self.state.hide_purchase_modal();
            return Ok(());
        }

        match key.code {
            KeyCode::Char('y') => {
                self.action_handler.get_state().modals.purchase =
                    self.state.modals.purchase.clone();

                match self.action_handler.process_purchase().await {
                    Ok(_) => {
                        let action_state = self.action_handler.get_state();
                        self.state.modals.purchase = action_state.modals.purchase.clone();
                        self.state.user = action_state.user.clone();
                    }
                    Err(e) => {
                        self.state.modals.purchase.error = Some(format!("System error: {e}"));
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.state.hide_purchase_modal();
            }
            KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Right => {
                self.state.increase_quantity();
            }
            KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Left => {
                self.state.decrease_quantity();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_error_modal(&mut self, _key: KeyEvent) -> Result<()> {
        self.state.hide_error_modal();
        Ok(())
    }

    async fn handle_search_mode(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                self.state.select_product_from_search();
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.next_search_result();
            }
            KeyCode::Down => {
                self.state.next_search_result();
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.previous_search_result();
            }
            KeyCode::Up => {
                self.state.previous_search_result();
            }
            KeyCode::Char(c) => {
                self.state.modals.search.input.push(c);

                self.action_handler.get_state().modals.search.input =
                    self.state.modals.search.input.clone();
                self.action_handler.update_search_results();

                let action_state = self.action_handler.get_state();
                self.state.modals.search.results = action_state.modals.search.results.clone();
                self.state.modals.search.selected_index = action_state.modals.search.selected_index;
            }
            KeyCode::Backspace => {
                self.state.modals.search.input.pop();

                self.action_handler.get_state().modals.search.input =
                    self.state.modals.search.input.clone();
                self.action_handler.update_search_results();

                let action_state = self.action_handler.get_state();
                self.state.modals.search.results = action_state.modals.search.results.clone();
                self.state.modals.search.selected_index = action_state.modals.search.selected_index;
            }
            KeyCode::Esc => {
                self.state.hide_search_modal();
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_and_clear_number_prefix(&mut self) -> usize {
        let count = if self.state.ui.number_prefix.is_empty() {
            1
        } else {
            self.state.ui.number_prefix.parse::<usize>().unwrap_or(1)
        };
        self.state.ui.number_prefix.clear();
        count
    }

    fn move_products_down(&mut self, count: usize) {
        let products = self.state.get_sorted_products();
        let len = products.len();
        if len == 0 {
            return;
        }

        let current = self.state.products.list_state.selected().unwrap_or(0);

        let target_down = current + count;
        if target_down < len {
            self.state.products.list_state.select(Some(target_down));
        } else {
            let new_index = (current + 1).min(len - 1);
            self.state.products.list_state.select(Some(new_index));
        }
    }

    fn move_products_up(&mut self, count: usize) {
        let products = self.state.get_sorted_products();
        let len = products.len();
        if len == 0 {
            return;
        }

        let current = self.state.products.list_state.selected().unwrap_or(0);

        if current >= count {
            self.state.products.list_state.select(Some(current - count));
        } else {
            let new_index = current.saturating_sub(1);
            self.state.products.list_state.select(Some(new_index));
        }
    }

    fn go_to_top(&mut self) {
        self.state.products.list_state.select(Some(0));
        self.state.ui.number_prefix.clear();
    }

    fn go_to_bottom(&mut self) {
        let products = self.state.get_sorted_products();
        let len = products.len();
        if len > 0 {
            self.state.products.list_state.select(Some(len - 1));
        }
        self.state.ui.number_prefix.clear();
    }

    async fn handle_parking_modal(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if let Err(e) = self.state.confirm_parking() {
                    self.state.show_error_modal(
                        &format!("Error confirming parking: {e}"),
                        Some("Parking Error"),
                    );
                }
            }
            KeyCode::Tab => {
                self.state.next_parking_field();
            }
            KeyCode::BackTab => {
                self.state.prev_parking_field();
            }
            KeyCode::Char(c) => {
                if self.state.modals.parking.current_field == 0 {
                    if c.is_ascii_digit() && self.state.modals.parking.phone_input.len() < 8 {
                        self.state.modals.parking.phone_input.push(c);
                    }
                } else if c.is_alphanumeric() {
                    self.state
                        .modals
                        .parking
                        .license_plate_input
                        .push(c.to_ascii_uppercase());
                }
            }
            KeyCode::Backspace => {
                if self.state.modals.parking.current_field == 0 {
                    self.state.modals.parking.phone_input.pop();
                } else {
                    self.state.modals.parking.license_plate_input.pop();
                }
            }
            KeyCode::Esc => {
                self.state.hide_parking_modal();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_parking_confirmation(&mut self, key: KeyEvent) -> Result<()> {
        if self.state.modals.parking.success || self.state.modals.parking.error.is_some() {
            self.state.hide_parking_modal();
            return Ok(());
        }

        match key.code {
            KeyCode::Char('y') => {
                let phone = self.state.modals.parking.phone_input.clone();
                let license_plate = self.state.modals.parking.license_plate_input.clone();

                match self
                    .action_handler
                    .process_parking_registration(&license_plate, &phone)
                    .await
                {
                    Ok(_) => {
                        let action_state = self.action_handler.get_state();
                        self.state.modals.parking.success = action_state.modals.parking.success;
                        self.state.modals.parking.error = action_state.modals.parking.error.clone();
                    }
                    Err(e) => {
                        self.state.show_error_modal(
                            &format!("System error: {e}"),
                            Some("Parking System Error"),
                        );
                    }
                }
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                self.state.hide_parking_modal();
            }
            _ => {}
        }
        Ok(())
    }
}

enum UIEvent {
    Input(Event),
    ClockTick,
}
