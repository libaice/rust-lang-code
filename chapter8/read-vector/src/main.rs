fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("the third value is {}", third);

    // get return Option  None
    match v.get(11) {
        Some(third) => println!("the third value is {}", third),
        None => println!("There is no third element"),
    }


    // the content cannot have mut and immut

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    v.push(8);
    println!("the first element is {} ", first )
}
