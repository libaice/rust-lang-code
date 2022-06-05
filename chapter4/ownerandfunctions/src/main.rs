fn main() {
    let s = String::from("Hello World");
    take_ownership(s);

    let x = 5;
    // i32 is copied
    makes_copy(x);

    // ok to still use x
    println!("{}", x);
    // println!("{}", s);

}


fn take_ownership(some_thing: String) {
    // s ownership in there
    println!("{}", some_thing);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}