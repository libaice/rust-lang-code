fn main() {
    // String = Wrapped Vec<u8>

    let len = String::from("Hola").len();
    println!("len is {}", len);


    // let s = String::from("Hello");
    // let h = s[3];

    // bytes
    for c in "World".bytes() {
        println!("c is {}", c);
    }
    // chars iter string
    for c in "World".chars() {
        println!("c is {}", c);
    }
}
