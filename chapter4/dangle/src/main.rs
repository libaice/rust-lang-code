fn main() {
    let reference = dangle();
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
