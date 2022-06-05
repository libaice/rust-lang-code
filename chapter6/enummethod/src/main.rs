enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32),
}


struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);


impl Message {
    fn call(&self) {}
}

fn main() {
    let m = Message::Write(String::from("Test"));
    m.call()
}
