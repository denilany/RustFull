use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut available = HashSet::new();

    s.to_lowercase()
        .chars()
        .for_each(|c| match c {
            'a'..='z' => { available.insert(c); },
            _ => {}
        });

    available.len() == 26
}
