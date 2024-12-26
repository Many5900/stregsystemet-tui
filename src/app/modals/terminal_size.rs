use crate::app::state::{AppState, InputMode};

pub const MIN_TERMINAL_WIDTH: u16 = 120;
pub const MIN_TERMINAL_HEIGHT: u16 = 40;

pub trait TerminalSizeModalActions {
    fn show_terminal_size_modal(&mut self);

    fn hide_terminal_size_modal(&mut self);

    fn check_terminal_size(&mut self, width: u16, height: u16);
}

impl TerminalSizeModalActions for AppState {
    fn show_terminal_size_modal(&mut self) {
        self.modals.terminal_size.visible = true;

        self.push_input_mode(InputMode::TerminalSizeModal);
    }

    fn hide_terminal_size_modal(&mut self) {
        self.modals.terminal_size.visible = false;

        self.pop_input_mode();
    }

    fn check_terminal_size(&mut self, width: u16, height: u16) {
        let is_too_small = width < MIN_TERMINAL_WIDTH || height < MIN_TERMINAL_HEIGHT;
        let modal_currently_visible = self.modals.terminal_size.visible;

        if is_too_small && !modal_currently_visible {
            self.show_terminal_size_modal();
        } else if !is_too_small && modal_currently_visible {
            self.hide_terminal_size_modal();
        }
    }
}
