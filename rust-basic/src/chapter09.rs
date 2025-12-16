// use std::fs::File;
// use std::io::{self, Read};

pub fn main() {
    println!("This is chapter 09: Error Handling in Rust.");

    // read_username_from_file().unwrap();

    // last_char_of_first_line("hello world");

    match Guess::new(101) {
        Ok(guess_number) => println!("Guess number is: {}", guess_number.value()),
        Err(e) => println!("Failed to create Guess: {}", e),
    };
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     // let username_file_result = File::open("hello.txt");

//     // let mut username_file = match username_file_result {
//     //     Ok(file) => file,
//     //     Err(e) => return Err(e),
//     // };

//     // let mut username_file = File::open("hello.txt")?;

//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username)
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     // match text.lines().next() {
//     //     None => None,
//     //     Some(first_line) => match first_line.chars().last() {
//     //         None => None,
//     //         Some(last_char) => Some(last_char),
//     //     },
//     // }
//     text.lines().next()?.chars().last()
// }

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!(
                "Guess value must be between 1 and 100, got {}.",
                value
            ));
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
