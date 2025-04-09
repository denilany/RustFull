pub use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now();
        let date = now.format("%Y-%m-%d %H:%M:%S").to_string();

        FormError {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        // 1. Check name is not empty
        if self.name.trim().is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        // 2. Password must be at least 8 chars
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        // 3. Password must contain at least one letter, one number, and one symbol
        let mut has_letter = false;
        let mut has_number = false;
        let mut has_symbol = false;

        for c in self.password.chars() {
            if c.is_ascii_alphabetic() {
                has_letter = true;
            } else if c.is_ascii_digit() {
                has_number = true;
            } else if c.is_ascii() {
                has_symbol = true;
            }
        }

        if !(has_letter && has_number && has_symbol) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ));
        }

        Ok(())
    }
}