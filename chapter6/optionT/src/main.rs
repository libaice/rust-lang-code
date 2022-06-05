// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     // match must get all
//     // match x {
//     //      None => None,
//     //     Some(i) => Some(i + 1)
//     // }
//
//
//
// }


// fn main() {
//     let v = 0u8;
//     match v {
//         1 => println!("ONE"),
//         3 => println!("Three"),
//         4 => println!("Four"),
//         5 => println!("Five"),
//          tuple type
//         _ => ()
//     }
// }


fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}