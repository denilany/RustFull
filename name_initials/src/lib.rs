pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials_vec: Vec<String> = Vec::new();

    for person in names {
        let mut initials = String::new();

        for name in person.split_whitespace() {
            let initial = name.chars().next().unwrap_or_default().to_ascii_uppercase();
            initials.push(initial);
            initials.push('.');
            initials.push(' ');
        }

        if !initials.is_empty() {
            initials.pop();
        }

        initials_vec.push(initials);
    }

    initials_vec
}
