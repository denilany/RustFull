pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let dec: f64 = c as f64;
    (c, dec.exp(), dec.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .map(|num| num.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_values = b
        .iter()
        .map(|&num| (num as f64).abs().ln())
        .collect();

    (b, log_values)
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
