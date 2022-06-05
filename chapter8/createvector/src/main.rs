fn main() {
    // let v: Vec<i32> = Vec::new();

    // use vec! macro
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(8);
    v.push(7);

    {
        // delete vector
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(8);
        v.push(7);
    }
    // then no inner vector

    // read vec
    // index


}

