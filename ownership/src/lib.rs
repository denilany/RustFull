pub fn first_subword(mut s: String) -> String {
    let mut index = 0;
    let mut chars = s.chars();

    if let Some(first) = chars.next() {
        if first.is_ascii_uppercase() {
            for c in chars {
                if c.is_ascii_uppercase() {
                    break;
                }
                index += 1;
            }

            return s[..index+1].to_string();
        } else if first.is_ascii_lowercase() {
            for c in chars {
                if c.is_ascii_uppercase() {
                    break;
                } else if c == '_' {
                    break;
                }
                index += 1;
            }

            return s[..index+1].to_string();
        }
    }
    "".to_string()
}