use crate::app::state::{AppState, InputMode};

pub trait ErrorModalActions {
    fn show_error_modal(&mut self, message: &str, title: Option<&str>);

    fn hide_error_modal(&mut self);
}

impl ErrorModalActions for AppState {
    fn show_error_modal(&mut self, message: &str, title: Option<&str>) {
        self.modals.error.visible = true;
        self.modals.error.message = Some(message.to_string());
        self.modals.error.title = title.map(|t| t.to_string());

        self.push_input_mode(InputMode::ErrorModal);
    }

    fn hide_error_modal(&mut self) {
        self.modals.error.visible = false;
        self.modals.error.message = None;
        self.modals.error.title = None;

        self.pop_input_mode();
    }
}
