pub fn scytale_cipher(message: String, i: u32) -> String {
    let rows = i as usize;
    let chars: Vec<char> = message.chars().collect();
    
    // Calculate how many columns we need
    let cols = (chars.len() + rows - 1) / rows;

    // Pad the characters with spaces to fit perfectly in the grid
    let mut padded = chars.clone();
    padded.resize(rows * cols, ' ');

    // Read column by column
    let mut result = String::new();
    for col in 0..cols {
        for row in 0..rows {
            result.push(padded[row * cols + col]);
        }
    }

    result
}
