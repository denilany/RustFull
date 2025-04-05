pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut current_word = String::new();
    let mut in_word = false;

    for ch in input.chars() {
        if ch.is_alphabetic() {
            current_word.push(ch);
            in_word = true;
        } else {
            if in_word {
                result.push_str(&capitalize_first(&current_word));
                current_word.clear();
                in_word = false;
            }
            result.push(ch);
        }
    }

    if in_word {
        result.push_str(&capitalize_first(&current_word));
    }

    result
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
