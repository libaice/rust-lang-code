fn largest_i32(list: &[i32]) -> i32 {
    let mut lagerest = list[0];
    for &item in list {
        if item > lagerest {
            lagerest = item;
        }
    }
    lagerest
}


fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}


fn main() {
    let number_list = vec![34, 60, 12, 300, 67];
    let result = largest_i32(&number_list);
    println!("the lagerest number is {}", result);


    let char_list = vec!['y', 'r', 'T', 'Q'];
    let max_char = largest_char(&char_list);
    println!("the largest char is {}", max_char);
}