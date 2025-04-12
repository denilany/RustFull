pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (index, &value) in array.iter().enumerate() {
        match value {
            v if v == key => return Some(index),
            _ => continue,
        }
    }
    None
}