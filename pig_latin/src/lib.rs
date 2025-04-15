pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let word = text.to_lowercase();
    let chars: Vec<char> = word.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", word);
    }

    let mut index = 0;

    // Special case for consonant followed by 'qu'
    if chars.len() > 2 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        index = 3;
    } else {
        // Find first vowel position
        for (i, c) in chars.iter().enumerate() {
            if vowels.contains(c) {
                index = i;
                break;
            }
        }
    }

    let prefix: String = chars[..index].iter().collect();
    let rest: String = chars[index..].iter().collect();

    format!("{}{}ay", rest, prefix)
}