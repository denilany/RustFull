use std::fs::File;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(error) => panic!("{}", error)
    }
}