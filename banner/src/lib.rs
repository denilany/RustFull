use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, desc: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        // Insert both short and long versions
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(callback) = self.flags.get(input) {
            if argv.len() >= 2 {
                callback(argv[0], argv[1]).map_err(|e| e.to_string())
            } else {
                Err("Not enough arguments".to_string())
            }
        } else {
            Err("Flag not found".to_string())
        }
    }
}


pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a % b).to_string())
}
