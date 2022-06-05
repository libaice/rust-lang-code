// fn main() {
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1);
//     println!(" s1 is {} and its length is {} ", s1, len);
// }
//
//
// // reference is borrow
// fn calculate_length(s: &str) -> usize {
//     s.len()
// }

fn main() {
    // let mut s1 = String::from("Hello");
    //
    // change(&mut s1);
    //
    // println!("s1 is {} ", s1);

    // restrict multi referance

    let mut s = String::from("Hello ");
    {
        let r1 = &mut s;
        println!("r1 r2 is {}", r1);

    }

    //different scope
    let r2 = &mut s;

    println!("ri r2 is {}", r2);
}

// fn change(some_string: &mut String) {
//     some_string.push_str(" , world");
// }