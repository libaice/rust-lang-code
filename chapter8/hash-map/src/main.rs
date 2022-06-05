use std::collections::HashMap;

fn main() {
    // create hash Map Store data on the heap
    let mut scores = HashMap::new();
    scores.insert(String::from("baice"), 20);
    scores.insert(String::from("world"), 30);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();


    // HashMap get OwnerShip hashMap be the owner

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // if param is & , then value will not lose ownership
    map.insert(&field_name, &field_value);

    println!("field_name is {}", field_name);
    println!("field_value is {}", field_value);


    // access Hash Map


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 30);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);


    match score {
        Some(s) => println!("s is {}", s),
        None => println!(" team not exist "),
    }


    // iter hashMap
    for (key, value) in &scores {
        println!("{}:{} ", key, value);
    }

    // update HashMap


}
