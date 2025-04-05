pub fn sum(a: &[i32]) -> i32 {
    // a.iter().sum()

    let mut result = 0;

    for num in a.iter() {
        result += num;
    }

    result
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let array: [i32; 32] = [10; 32];
    array
}