pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }

    let mut parts = Vec::new();

    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut number = n;

    let thousands = [
        (1_000_000, "million"),
        (1_000, "thousand"),
        (100, "hundred"),
    ];

    for &(value, name) in &thousands {
        if number >= value {
            let count = number / value;
            number %= value;
            let sub = spell(count);
            parts.push(format!("{} {}", sub, name));
        }
    }

    if number >= 20 {
        let t = number / 10;
        let r = number % 10;
        if r != 0 {
            parts.push(format!("{}-{}", tens[t as usize], below_20[r as usize]));
        } else {
            parts.push(tens[t as usize].to_string());
        }
    } else if number > 0 {
        parts.push(below_20[number as usize].to_string());
    }

    parts.join(" ")
}
