pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)
        .filter(|num| num % 2 == 0)
        .map(|num| num * num)
        .take(50)
        .collect()
}