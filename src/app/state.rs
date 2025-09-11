use std::collections::HashMap;

use ratatui::widgets::ListState;

use crate::config::settings::Config;
use crate::models::member::{MemberInfo, Sale};
use crate::models::product::Product;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub ui: UiState,
    pub products: ProductsState,
    pub user: UserState,
    pub modals: ModalState,
    pub should_quit: bool,
}

#[derive(Clone)]
pub struct UiState {
    pub input: String,
    pub input_mode: InputMode,
    pub previous_input_mode: Option<InputMode>,
    pub number_prefix: String,
    pub pending_g: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputMode {
    Normal,
    Editing,
    EditingUsername,
    BuyConfirmation,
    SearchMode,
    ErrorModal,
    ParkingModal,
    ParkingConfirmation,
    TerminalSizeModal,
    QrPaymentAmount,
    QrPaymentDisplay,
}

#[derive(Clone)]
pub struct ProductsState {
    pub items: HashMap<String, Product>,
    pub list_state: ListState,
    pub error: Option<String>,
    pub named_products: HashMap<String, i32>,
    pub named_products_error: Option<String>,
}

#[derive(Clone)]
pub struct UserState {
    pub member_id: Option<i32>,
    pub member_info: Option<MemberInfo>,
    pub latest_sales: Vec<Sale>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct ModalState {
    pub username: UsernameModalState,
    pub purchase: PurchaseModalState,
    pub search: SearchModalState,
    pub error: ErrorModalState,
    pub parking: ParkingModalState,
    pub terminal_size: TerminalSizeModalState,
    pub qr_payment: QrPaymentModalState,
}

#[derive(Clone)]
pub struct ErrorModalState {
    pub visible: bool,
    pub message: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone)]
pub struct UsernameModalState {
    pub visible: bool,
    pub input: String,
}

#[derive(Clone)]
pub struct PurchaseModalState {
    pub visible: bool,
    pub selected_product_id: Option<String>,
    pub error: Option<String>,
    pub success: bool,
    pub quantity: u32,
}

#[derive(Clone)]
pub struct SearchModalState {
    pub visible: bool,
    pub input: String,
    pub results: Vec<Product>,
    pub selected_index: usize,
}

#[derive(Clone)]
pub struct ParkingModalState {
    pub visible: bool,
    pub phone_input: String,
    pub license_plate_input: String,
    pub current_field: usize,
    pub error: Option<String>,
    pub success: bool,
    pub confirming: bool,
    pub vehicle_brand: Option<String>,
    pub vehicle_model: Option<String>,
    pub vehicle_variant: Option<String>,
}

#[derive(Clone)]
pub struct TerminalSizeModalState {
    pub visible: bool,
}

#[derive(Clone)]
pub struct QrPaymentModalState {
    pub visible: bool,
    pub amount_input: String,
    pub qr_data: Option<crate::qr::payment_qr::PaymentQrData>,
    pub showing_qr: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let mut product_list_state = ListState::default();
        product_list_state.select(Some(0));

        Self {
            config: config.clone(),

            ui: UiState {
                input: String::new(),
                input_mode: InputMode::Normal,
                previous_input_mode: None,
                number_prefix: String::new(),
                pending_g: false,
            },

            products: ProductsState {
                items: HashMap::new(),
                list_state: product_list_state,
                error: None,
                named_products: HashMap::new(),
                named_products_error: None,
            },

            user: UserState {
                member_id: None,
                member_info: None,
                latest_sales: Vec::new(),
                error: None,
            },

            modals: ModalState {
                username: UsernameModalState {
                    visible: false,
                    input: String::new(),
                },
                purchase: PurchaseModalState {
                    visible: false,
                    selected_product_id: None,
                    error: None,
                    success: false,
                    quantity: 1,
                },
                search: SearchModalState {
                    visible: false,
                    input: String::new(),
                    results: Vec::new(),
                    selected_index: 0,
                },
                error: ErrorModalState {
                    visible: false,
                    message: None,
                    title: None,
                },
                parking: ParkingModalState {
                    visible: false,
                    phone_input: String::new(),
                    license_plate_input: String::new(),
                    current_field: 0,
                    error: None,
                    success: false,
                    confirming: false,
                    vehicle_brand: None,
                    vehicle_model: None,
                    vehicle_variant: None,
                },
                terminal_size: TerminalSizeModalState { visible: false },
                qr_payment: QrPaymentModalState {
                    visible: false,
                    amount_input: String::new(),
                    qr_data: None,
                    showing_qr: false,
                },
            },

            should_quit: false,
        }
    }

    pub fn handle_invalid_username(&mut self, error: &str) {
        self.user.error = Some(format!("Username error: {error}"));

        self.user.member_id = None;
        self.user.member_info = None;
        self.user.latest_sales = Vec::new();
    }

    pub fn get_sorted_products(&self) -> Vec<&Product> {
        let mut products_vec: Vec<&Product> = self.products.items.values().collect();

        products_vec.sort_by(|a, b| match (a.id.parse::<i32>(), b.id.parse::<i32>()) {
            (Ok(id_a), Ok(id_b)) => id_a.cmp(&id_b),
            _ => a.id.cmp(&b.id),
        });

        products_vec
    }

    pub fn get_movement_target_indices(&self) -> Vec<usize> {
        if self.ui.number_prefix.is_empty() {
            return Vec::new();
        }

        let current_index = self.products.list_state.selected().unwrap_or(0);
        let products = self.get_sorted_products();
        let total_lines = products.len();
        let mut targets = Vec::new();

        if let Ok(relative_distance) = self.ui.number_prefix.parse::<usize>() {
            if relative_distance > 0 {
                if current_index >= relative_distance {
                    targets.push(current_index - relative_distance);
                }

                let target_below = current_index + relative_distance;
                if target_below < total_lines {
                    targets.push(target_below);
                }
            }
        }

        targets
    }

    pub fn push_input_mode(&mut self, new_mode: InputMode) {
        self.ui.previous_input_mode = Some(self.ui.input_mode);
        self.ui.input_mode = new_mode;
    }

    pub fn pop_input_mode(&mut self) {
        if let Some(mode) = self.ui.previous_input_mode.take() {
            self.ui.input_mode = mode;
        } else {
            self.ui.input_mode = InputMode::Normal;
        }
    }

    pub fn validate_user_for_purchase(&self) -> Result<(), String> {
        if self.user.error.is_some()
            || self.user.member_id.is_none()
            || self.user.member_info.is_none()
        {
            let message = if let Some(ref error) = self.user.error {
                format!("Error: {error}")
            } else {
                "Please sign in with a valid username. The current username doesn't exist or couldn't be verified.".to_string()
            };
            return Err(message);
        }
        Ok(())
    }
}
