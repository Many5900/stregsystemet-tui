use crate::app::state::{AppState, InputMode};

pub trait HelpModalActions {
    fn show_help_modal(&mut self);
    fn hide_help_modal(&mut self);
    fn next_help_tab(&mut self);
    fn previous_help_tab(&mut self);
}

impl HelpModalActions for AppState {
    fn show_help_modal(&mut self) {
        self.modals.help.visible = true;
        self.push_input_mode(InputMode::HelpModal);
    }

    fn hide_help_modal(&mut self) {
        self.modals.help.visible = false;
        self.pop_input_mode();
    }

    fn next_help_tab(&mut self) {
        self.modals.help.current_tab = self.modals.help.current_tab.next();
    }

    fn previous_help_tab(&mut self) {
        self.modals.help.current_tab = self.modals.help.current_tab.previous();
    }
}
