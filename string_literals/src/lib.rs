pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    // v.is_ascii()?

    for c in v.chars() {
        if c as u8 > 127 {
            return false;
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    match v.find(pat) {
        Some(index) => index,
        None => usize::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
