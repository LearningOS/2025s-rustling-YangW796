// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,                // 不带数据的变体
    Echo,         // 带有字符串数据的变体
    Move,  // 带有命名字段的结构体变体
    ChangeColor,  // 带有元组形式的 RGB 颜色变体
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
