use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

    println!("{riddle}");

    let correct_answer = "The letter e";

    let mut count = 0;

    loop {
        let mut answer = String::new();

        count += 1;

        io::stdin().read_line(&mut answer).expect("Failed to read line");

        let is_correct = correct_answer.to_lowercase() == answer.trim().to_lowercase();

        if !is_correct {
            println!("{riddle}");
            continue;
        } else {
            println!("Number of trials: {}", count);
            break;
        }
    }
}
