fn main() {
    // let guess: u32 = "43".parse().expect("error parse string -> Int");
    // println!("guess is {}", guess)

    // Integer  default
    // isize  usize

    // let x = 2.0;
    // let y: f32 = 3.0;
    // println!("x is {}", x);
    // println!("y is {}", y);


    // operation
    let differance = 99.5 - 3.111;
    let product = 4 * 22;
    let quotient = 6331.123 / 23.2897;
    let floored = 8 / 3;

    let reminder = 43 / 5;

    println!("differance  {}", differance);
    println!("product  {}", product);
    println!("quotient  {}", quotient);
    println!("floored  {}", floored);
    println!("reminder  {}", reminder);


    // bool

    // let t = true;
    // let f: bool = false;
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!(" c is {}", c);
    println!(" z is {}", z);
    println!(" heart cat  is {} ", heart_eyed_cat);


    // compound

    // Tuple
    let tup: (i32, f64, u8) = (600, 3.1231, 23);

    let (x, y, z) = tup;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);


    println!(" 0 {}, 1  {}, 2 {} ", tup.0, tup.1, tup.2);


    // Array
    let a = [12, 324, 576, 76, 1];

    println!(" a is {} ", a[1]);
    let a = [1231; 6];
    println!(" a is {} ", a[3]);
}
