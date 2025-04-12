pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => {
                let a = 'a' as u8;
                let offset = (26 + (c as i8 - a as i8 + key) % 26) % 26;
                (a + offset as u8) as char
            }
            'A'..='Z' => {
                let a = 'A' as u8;
                let offset = (26 + (c as i8 - a as i8 + key) % 26) % 26;
                (a + offset as u8) as char
            }
            _ => c,
        })
        .collect()
}