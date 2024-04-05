use console::Style;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> String {
        let error_style = Style::new().for_stderr().red();
        format!("{}", error_style.apply_to(message))
    }

    pub fn _log(message: &str) -> () {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(message));
    }

    pub fn _missing_config(&self) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(&self.message));
    }

    pub fn _invalid_config_type(&self) {
        let error_style = Style::new().for_stderr().red();
        println!("{}", error_style.apply_to(&self.message));
    }
}
