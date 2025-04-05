pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}