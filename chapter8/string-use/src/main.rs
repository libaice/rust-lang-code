use std::fmt::format;

fn main() {
    // Create String
    let mut s = String::new();

    // core is &str
    // String 标准库
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    // String can get ownership
    // &str can be borrowed


    // create string
    let s = String::from("initial String");

    // update string
    let mut s = String::from("foo ");
    // not lose ownership
    s.push_str("bar");
    println!("s is {}", s);


    let mut s1 = String::from("foo ");
    let s2 = String::from(" bar ");
    s1.push_str(&s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);


    // push single
    let mut s3 = String::from("lo");
    s3.push('x');
    println!("s3 is {}", s3);

    // + string
    let s1 = String::from("Hello ");
    let s2 = String::from(" World ! ");


    // get s1 ownership and add s2
    // then return s3
    let s3 = s1 + &s2;
    println!(" s3 is {} ", s3);

    // &String coerce -> &str
    println!(" s2 is {} ", s2);
    // s1 have no ownership
    // println!(" s1 is {} ", s1);


    // format String


    let m1 = String::from("tic");
    let m2 = String::from("Tokem");
    let m3 = String::from("ecto");

    let mm = format!(" {}-{}-{} ", m1, m2, m3);
    println!("mm is {}", mm);
}
