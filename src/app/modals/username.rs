use crate::app::state::{AppState, InputMode};
use crate::config::store::save_config;
use crate::error::{AppError, Result};

pub trait UsernameModalActions {
    fn show_username_modal(&mut self);

    fn hide_username_modal(&mut self);

    fn update_username(&mut self) -> Result<()>;
}

impl UsernameModalActions for AppState {
    fn show_username_modal(&mut self) {
        self.modals.username.visible = true;
        self.push_input_mode(InputMode::EditingUsername);

        self.modals.username.input = self.config.username.clone().unwrap_or_default();
    }

    fn hide_username_modal(&mut self) {
        self.modals.username.visible = false;
        self.pop_input_mode();
        self.modals.username.input.clear();
    }

    fn update_username(&mut self) -> Result<()> {
        let new_username = self.modals.username.input.trim().to_string();

        if new_username.is_empty() {
            return Err(AppError::Input("Username cannot be empty".to_string()));
        }

        self.config.username = Some(new_username);
        save_config(&self.config)?;
        self.hide_username_modal();

        Ok(())
    }
}
