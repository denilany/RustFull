pub fn stars(n: u32) -> String {
    let repeat_count = (2u32).checked_pow(n).expect("Power overflow");
    "*".repeat(repeat_count as usize)
}