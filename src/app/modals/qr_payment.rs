use crate::app::modals::error::ErrorModalActions;
use crate::app::state::{AppState, InputMode};
use crate::error::Result;
use crate::qr::payment_qr::PaymentQrData;

pub trait QrPaymentModalActions {
    fn show_qr_payment_modal(&mut self);
    fn hide_qr_payment_modal(&mut self);
    fn generate_qr_code(&mut self) -> Result<()>;
    fn back_to_amount_input(&mut self);
}

impl QrPaymentModalActions for AppState {
    fn show_qr_payment_modal(&mut self) {
        if self.user.member_info.is_none() {
            self.show_error_modal(
                "Invalid user account. Please log in with a valid username.",
                Some("Invalid User"),
            );
            return;
        }

        self.modals.qr_payment.visible = true;
        self.modals.qr_payment.amount_input.clear();
        self.modals.qr_payment.qr_data = None;
        self.modals.qr_payment.showing_qr = false;
        self.push_input_mode(InputMode::QrPaymentAmount);
    }

    fn hide_qr_payment_modal(&mut self) {
        self.modals.qr_payment.visible = false;
        self.modals.qr_payment.amount_input.clear();
        self.modals.qr_payment.qr_data = None;
        self.modals.qr_payment.showing_qr = false;

        if self.ui.input_mode == InputMode::QrPaymentDisplay {
            self.pop_input_mode();
            self.pop_input_mode();
        } else {
            self.pop_input_mode();
        }
    }

    fn generate_qr_code(&mut self) -> Result<()> {
        let amount_str = self.modals.qr_payment.amount_input.trim();

        let amount: f64 = amount_str
            .parse()
            .map_err(|_| crate::error::AppError::Input("Invalid amount format".to_string()))?;

        if amount < 50.0 {
            self.show_error_modal("Amount must be at least 50.00 DKK", Some("Invalid Amount"));
            return Ok(());
        }

        let username = self.config.username.as_ref().unwrap().clone();

        match PaymentQrData::new(username, amount) {
            Ok(qr_data) => {
                if !qr_data.has_valid_qr() {
                    self.show_error_modal(
                        "Failed to generate QR code",
                        Some("QR Generation Error"),
                    );
                    return Ok(());
                }

                self.modals.qr_payment.qr_data = Some(qr_data);
                self.modals.qr_payment.showing_qr = true;
                self.push_input_mode(InputMode::QrPaymentDisplay);
            }
            Err(e) => {
                self.show_error_modal(
                    &format!("Failed to create payment data: {e}"),
                    Some("Payment Error"),
                );
            }
        }

        Ok(())
    }

    fn back_to_amount_input(&mut self) {
        self.modals.qr_payment.showing_qr = false;
        self.modals.qr_payment.qr_data = None;
        self.pop_input_mode();
    }
}
