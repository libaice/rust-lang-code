// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {}", count);
// }


fn main() {
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     println!(" x is {} ", counter);
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("the result is  {}", result)


    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{ }! ", number);
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF ");

    // while iter collection

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 1;
    //
    // while index < 5 {
    //     println!(" x value is  {}", a[index]);
    //     index += 1;
    // }


    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!(" x value is  {}", element);
    }
    println!();

    for number  in (1..4).rev()  {
        println!(" x number is  {}", number);
    }
    println!("LIFTOFF ");

}
