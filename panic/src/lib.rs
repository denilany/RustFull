use std::fs::File;
use std::io::ErrorKind;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!(),
            _other_error => {
                panic!();
            }
        },
    }
}