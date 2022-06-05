struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;


    let mut user1 = User {
        email: String::from("baice@gmail.com"),
        username: String::from("baiceloveyoi"),
        active: true,
        sign_in_count: 332,
    };

    println!(" user1's email is {} ", user1.email);
    user1.email = String::from("gsdaef");
    println!(" user1's email is {} ", user1.email);

    println!("user1 is {}", user1.active);


    let user2 = User {
        email: String::from("User2"),
        ..user1
    };


    println!("user2 is {}", user2.active);
    println!("user2 is {}", user2.email);
}
