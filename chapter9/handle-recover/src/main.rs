use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let res = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("error open file {:?}", error);
        }
    };

}
