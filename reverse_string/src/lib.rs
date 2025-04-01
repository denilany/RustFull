pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("Hello, world!");
        assert_eq!(result, "!dlrow ,olleH");
    }
}
