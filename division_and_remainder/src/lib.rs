pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let division = x / y;
    let remainder = x % y;

    (division, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(15, 2);
        assert_eq!(result, (7, 1));
    }
}
