extern crate case;
use case::CaseExt;
use edit_distance::edit_distance;

// This function calculates the similarity between an original and an expected string using the edit distance
// and returns a percentage similarity if it meets certain conditions.
pub fn expected_variable(original: &str, expected: &str) -> Option<String> {
    if original.contains(" ") {
        return None;
    }
    
    if !original.contains('_') && !original.chars().any(|c| c.is_ascii_uppercase()) {
        None
    } else {
        let diff = edit_distance(&original.to_lowercase(), &expected.to_lowercase());
        
        if diff > original.len() {
            return None;
        }
        
        let bigger = std::cmp::max(original.len(), expected.len());
        
        let res = ((bigger - diff) * 100) as f64 / bigger as f64;
        
        let resu = res.ceil();
        
        if resu < 50.0 {
            return None;
        }
        
        return Some(resu.to_string() + &"%".to_string());
    }
}