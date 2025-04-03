pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(&str, u32)> = phrase
        .split_whitespace()
        .filter_map(|word| {
            let pos = word.chars().find(|c| c.is_digit(10))?.to_digit(10)?;
            Some((word, pos))
        })
        .collect();
    
    words.sort_by_key(|&(_, pos)| pos);
    
    words.iter()
        .map(|&(word, _)| word.chars().filter(|c| !c.is_digit(10)).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
