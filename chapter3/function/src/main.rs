fn main() {
    // println!("Hello, world!");
    // another_function(20,'M');

    // statement and expression

    // let x = 5;
    // let y = {
    //     let x = 1;
    //     x + 1
    // };
    //
    // println!(" y is {} ", y);


    let x = five();
    println!("x is {}", x);
    let y = plus_one(x);
    println!("y is {}", y);
}


fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 {
    // no ; or statement
    x + 1
}


// fn another_function(value: i32, unit_label: char) {
//     println!("Another function , param is {} and label is {}", value, unit_label);
// }


