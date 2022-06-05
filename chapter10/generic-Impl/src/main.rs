struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2).sqrt())
    }
}


// T 在impl 后面 表示 在类型T 上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 6, y: 10 };
    println!("p.x is {}", p.x)



}
