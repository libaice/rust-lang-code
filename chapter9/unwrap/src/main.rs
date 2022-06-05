use std::fs::File;
use std::io::{self, Read};

fn main() {
    // if Ok the => ok  if err then panic!
    // let f = File::open("hello.txt").unwrap();


    //expect
    // let f = File::open("hello.txt").expect("Fail to opem file !!! Hello.txt  !!!");

    // propagate error  return error to calling code

    // let result = read_username_from_file();
}


// progate error
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e)  // Err(3)
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }


