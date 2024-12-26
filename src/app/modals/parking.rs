use crate::app::state::{AppState, InputMode};
use crate::config::store::save_config;
use crate::error::{AppError, Result};

pub trait ParkingModalActions {
    fn show_parking_modal(&mut self);

    fn hide_parking_modal(&mut self);

    fn next_parking_field(&mut self);

    fn prev_parking_field(&mut self);

    fn confirm_parking(&mut self) -> Result<()>;
}

impl ParkingModalActions for AppState {
    fn show_parking_modal(&mut self) {
        self.modals.parking.visible = true;
        self.push_input_mode(InputMode::ParkingModal);

        self.modals.parking.phone_input = self.config.phone_number.clone().unwrap_or_default();
        self.modals.parking.license_plate_input =
            self.config.license_plate.clone().unwrap_or_default();
        self.modals.parking.current_field = 0;
        self.modals.parking.confirming = false;
        self.modals.parking.error = None;
        self.modals.parking.success = false;
    }

    fn hide_parking_modal(&mut self) {
        self.modals.parking.visible = false;

        if self.modals.parking.confirming {
            self.pop_input_mode();
        }
        self.pop_input_mode();

        self.modals.parking.phone_input.clear();
        self.modals.parking.license_plate_input.clear();
        self.modals.parking.current_field = 0;
        self.modals.parking.confirming = false;
        self.modals.parking.error = None;
        self.modals.parking.success = false;
    }

    fn next_parking_field(&mut self) {
        self.modals.parking.current_field = (self.modals.parking.current_field + 1) % 2;
    }

    fn prev_parking_field(&mut self) {
        self.modals.parking.current_field = if self.modals.parking.current_field == 0 {
            1
        } else {
            0
        };
    }

    fn confirm_parking(&mut self) -> Result<()> {
        let phone = self.modals.parking.phone_input.trim();
        let license_plate = self
            .modals
            .parking
            .license_plate_input
            .trim()
            .to_uppercase();

        if phone.is_empty() {
            return Err(AppError::Input("Phone number cannot be empty".to_string()));
        }

        if license_plate.is_empty() {
            return Err(AppError::Input("License plate cannot be empty".to_string()));
        }

        if !phone.chars().all(|c| c.is_ascii_digit()) || phone.len() != 8 {
            return Err(AppError::Input("Phone number must be 8 digits".to_string()));
        }

        if license_plate.len() != 7 {
            return Err(AppError::Input(
                "License plate must be exactly 7 characters".to_string(),
            ));
        }

        let chars: Vec<char> = license_plate.chars().collect();

        if !chars[0].is_ascii_alphabetic() || !chars[1].is_ascii_alphabetic() {
            return Err(AppError::Input(
                "License plate must start with 2 letters".to_string(),
            ));
        }

        if !chars[2..].iter().all(|c| c.is_ascii_digit()) {
            return Err(AppError::Input(
                "License plate must end with 5 digits".to_string(),
            ));
        }

        self.config.phone_number = Some(phone.to_string());
        self.config.license_plate = Some(license_plate.clone());
        save_config(&self.config)?;

        self.modals.parking.phone_input = phone.to_string();
        self.modals.parking.license_plate_input = license_plate;

        self.modals.parking.confirming = true;
        self.push_input_mode(InputMode::ParkingConfirmation);

        Ok(())
    }
}
