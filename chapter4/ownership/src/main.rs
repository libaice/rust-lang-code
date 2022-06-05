fn main() {
    // let mut s = String::from("Hello");
    // s.push_str("World");
    // println!("s value is {}", s);


    // move


    // copy stack
    let s1 = String::from("hell0");
    // owner s1 -> s2
    // let s2 = s1;
    let s2 = s1.clone();

    // Move
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);


    let m = 32;
    let n = m;

    println!("n is {}", n);
}

