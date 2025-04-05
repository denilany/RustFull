use std::collections::HashMap;

/// Calculates the mean (average) of a list of integers
pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len();
    sum as f64 / count as f64
}

/// Calculates the median (middle value) of a list of integers
pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[mid]
    }
}

/// Calculates the mode (most frequent value) of a list of integers
pub fn mode(list: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();

    for &num in list {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    *freq_map.iter().max_by_key(|&(_, count)| count).unwrap().0
}
