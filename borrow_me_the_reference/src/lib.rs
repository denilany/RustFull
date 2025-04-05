pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut plus_count = 0;

    while i < chars.len() {
        match chars[i] {
            '-' => {
                if !result.is_empty() {
                    result.pop(); // Handle backspace
                }
            }
            '+' => {
                plus_count += 1; // Count + occurrences
            }
            c => {
                if plus_count > 0 {
                    plus_count -= 1; // Remove characters equal to count of +
                } else {
                    result.push(c);
                }
            }
        }
        i += 1;
    }

    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        if let Some((lhs, op, rhs)) = parse_expression(expr) {
            let result = match op {
                '+' => lhs + rhs,
                '-' => lhs - rhs,
                _ => continue,
            };
            *expr = result.to_string();
        }
    }
}

fn parse_expression(expr: &str) -> Option<(i32, char, i32)> {
    if let Some(pos) = expr.find(|c: char| c == '+' || c == '-') {
        let (left, right) = expr.split_at(pos);
        let op = right.chars().next()?;
        let right = &right[1..];
        
        let lhs = left.trim().parse::<i32>().ok()?;
        let rhs = right.trim().parse::<i32>().ok()?;
        
        return Some((lhs, op, rhs));
    }
    None
}
