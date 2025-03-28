// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Quit,                       // 不带数据的变体
    Echo(String),                // 带有字符串数据的变体
    Move { x: i32, y: i32 },     // 带有命名字段的结构体变体
    ChangeColor(i32, i32, i32),  // 带有元组形式数据的变体
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
