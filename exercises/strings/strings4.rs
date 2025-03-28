// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                                  // 字符串字面量是 &str
    string("red".to_string());                             // 转为 String 类型
    string(String::from("hi"));                            // 使用 String::from() 创建 String
    string("rust is fun!".to_owned());                     // 将 &str 转为 String
    string("nice weather".into());                         // 使用 .into() 转为 String
    string(format!("Interpolation {}", "Station"));        // format!() 返回 String
    string_slice(&String::from("abc")[0..1]);              // 使用切片得到 &str
    string_slice("  hello there ".trim());                  // .trim() 返回 &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace() 返回 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());       // .to_lowercase() 返回 String
}
