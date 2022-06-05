// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}


enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let integer = Point { x: 5, y: 12 };
    let float = Point { x: 5.2, y: 12.1 };
    // x and y has same type
    let float = Point { x: 5, y: 12.1 };


    // let number_list = vec![24, 13, 436, 867, 23];
    // let result = largest(&number_list);
    // println!("the largest number is {}", result);
    //
    // let char_list = vec!['M', 'X', "y", 'Q', 'N'];
    // let result = largest(&char_list);
    // println!("the largest cahr is {}", result);
}



