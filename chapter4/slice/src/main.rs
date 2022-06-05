fn main() {
    // let s = String::from("hello world");
    // let hello = &s[1..5];
    // let world = &s[6..11];
    // println!(" hello is {}", hello);
    // println!(" world is {}", world);

    let mut s = String::from("hello world");

    let first = first_world(&s);
    // s.clear();
    println!(" first world is {} ", first);


    let s = "Hello world";
    // &str as parameter


}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}