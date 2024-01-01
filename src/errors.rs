use console::Style;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    pub fn missing_config(&self) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(&self.message));
    }

    pub fn invalid_config_type(&self) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(&self.message));
    }
}
