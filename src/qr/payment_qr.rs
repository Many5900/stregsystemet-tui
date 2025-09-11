use qrcode::QrCode;
use std::error::Error;

const F_KLUB_NUMBER: &str = "90601";

#[derive(Clone)]
pub struct PaymentQrData {
    pub username: String,
    pub amount: f64,
    pub qr_code: Option<QrCode>,
}

impl PaymentQrData {
    pub fn new(username: String, amount: f64) -> Result<Self, Box<dyn Error>> {
        let mobilepay_url = format!(
            "mobilepay://send?phone={F_KLUB_NUMBER}&comment={username}&amount={amount:.2}"
        );

        let qr_code = QrCode::new(&mobilepay_url).ok();

        Ok(Self {
            username,
            amount,
            qr_code,
        })
    }

    pub fn has_valid_qr(&self) -> bool {
        self.qr_code.is_some()
    }
}
