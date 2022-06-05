use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.js")?;
    Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file2() -> Result<String, io::Error> {
//     let mut s = String::new();
//     //  ? return type Result Option
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
