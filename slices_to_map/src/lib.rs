use std::collections::HashMap;

pub fn slices_to_map<'a, T: std::hash::Hash + std::cmp::Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    let len = std::cmp::min(keys.len(), values.len());
    
    for i in 0..len {
        map.insert(&keys[i], &values[i]);
    }
    
    map
}