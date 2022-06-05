fn main() {
    let config_max = Some(6u8);
    // match config_max {
    //     Some(max) => println!("the max data to be {}", max),
    //     _ => ()
    // }

    if let Some(3) = config_max {
        println!("three");
    } else {
        println!("Others");
    }
}
