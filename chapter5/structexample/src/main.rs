// fn main() {
//     let all = area(12, 90);
//     println!("all area is {}", all);
// }
//
// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }
//


// tuple
// fn main() {
//     let rect = (12, 98);
//     let ar = area(rect);
//     println!("all area is {}", ar);
//
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// structs

// struct Rectangle {
//     width: u32,
//     length: u32,
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         length: 67,
//     };
//
//     let area = area(&rect1);
//     println!("area is {}", area);
// }
//
// fn area(rect: &Rectangle) -> u32 {
//     rect.length * rect.width
// }


//  Debug Trits

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    // let rect = Rectangle {
    //     width: 30,
    //     length: 87,
    // };
    //
    // println!(" rect is {:#?} ", rect)


    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        length: 60,
    };
    dbg!(&rect1);
}






















