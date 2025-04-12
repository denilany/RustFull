pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as usize - 'A' as usize;
    let width = 2 * n + 1;
    let mut diamond = Vec::new();

    for i in 0..=n {
        diamond.push(build_line(i, width));
    }

    for i in (0..n).rev() {
        diamond.push(build_line(i, width));
    }

    diamond
}

fn build_line(i: usize, width: usize) -> String {
    let ch = (b'A' + i as u8) as char;
    if i == 0 {
        format!("{:^width$}", ch, width = width)
    } else {
        let inner_spaces = 2 * i - 1;
        let line = format!("{}{}{}", ch, " ".repeat(inner_spaces), ch);
        format!("{:^width$}", line, width = width)
    }
}
