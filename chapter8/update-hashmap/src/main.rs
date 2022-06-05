use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    // default overwrite
    scores.insert(String::from("Blue"), 30);
    println!("{:?}", scores);

    // only insert if key has no value


    scores.entry(String::from("Blue")).or_insert(40);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores);

    // update value base old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
