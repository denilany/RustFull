pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    match trimmed {
        "" => "Just say something!",
        _ if is_yelling_question(trimmed) => "Quiet, I am thinking!",
        _ if is_yelling(trimmed) => "There is no need to yell, calm down!",
        _ if !is_yelling(trimmed) && is_question(trimmed) => "Sure.",
        _ => "Interesting",
    }
}

fn is_yelling(text: &str) -> bool {
    let letters: String = text.chars().filter(|ch| ch.is_alphabetic()).collect();
    !letters.is_empty() && letters.chars().all(|c| c.is_uppercase())
}

fn is_question(text: &str) -> bool {
    text.ends_with('?')
}

fn is_yelling_question(text: &str) -> bool {
    is_yelling(text) && is_question(text)
}