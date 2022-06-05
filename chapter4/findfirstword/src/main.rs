fn main() {
    let mut s = String::from("hello world");

    let x = first_word(&mut s);
    println!("index is {}", x);
    s.clear();
    println!("index is {}", x);

    println!("s still is {}", s);

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
