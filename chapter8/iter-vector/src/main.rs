fn main() {
    // for iter vector

    // let v = vec![100, 23, 78];
    // for i in &v {
    //     println!("i is {}", i)
    // }

    let mut v = vec![100, 23, 78];
    for i in &mut v {
        *i += 60;
    }

    for i in v {
        println!("i is {}", i)
    }


    // Enum in vector  Enum + Match
    // trait
    let row = vec![
        SpreadSheetCall::Int(3),
        SpreadSheetCall::Float(3.999),
        SpreadSheetCall::Text(String::from("World")),
    ];
}


enum SpreadSheetCall {
    Int(i32),
    Float(f64),
    Text(String),
}


