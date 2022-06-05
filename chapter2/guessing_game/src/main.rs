use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(" Guessing the number ");
    println!(" Please input your guess ");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("the secret number is {}", secret_number);

    loop {
        println!("please input your guess  ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // if convert String -> Integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is {}", guess);

        // let guess: u32 = guess.trim().parse().expect("Please type a number");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
