fn main() {
    // let s1 = give_ownership();
    // let s2 = String::from("Hello");
    //
    // let s3 = take_and_gives_back(s2);
    //
    // println!("s1 is {}", s1);
    // // s2 has no ownershi
    // // println!("s2 is {}", s2);
    // println!("s3 is {}", s3);

    let s1 = String::from("make love");
    let (s2, length) = calculate_length(s1);
    println!(" The length of {} is {} ", s2, length);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // return many with tuple
    (s, length)
}


// fn give_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }
//
// fn take_and_gives_back(a_string: String) -> String {
//     a_string
// }