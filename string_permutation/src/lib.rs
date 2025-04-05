use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut count_map = HashMap::new();

    for ch in s1.chars() {
        *count_map.entry(ch).or_insert(0) += 1;
    }

    for ch in s2.chars() {
        match count_map.get_mut(&ch) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    count_map.remove(&ch);
                }
            }
            None => return false,
        }
    }

    count_map.is_empty()
}
