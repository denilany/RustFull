use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        FormError {
            form_values: (field_name.to_string(), field_value),
            date,
            err: err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn new(name: String, password: String) -> Self {
        Form { name, password }
    }

    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        }

        if !self.password.chars().any(|c| c.is_ascii_digit()) {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }

        if !self.password.chars().any(|c| c.is_ascii_alphabetic()) {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }

        if !self.password.chars().any(|c| !c.is_ascii_alphanumeric()) {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        }

        Ok(())
    }
}