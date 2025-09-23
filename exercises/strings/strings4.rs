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
    string_slice("blue");  // 字符串字面量是 &str 类型
    string("red".to_string());  // to_string() 返回 String 类型
    string(String::from("hi"));  // String::from() 返回 String 类型
    string("rust is fun!".to_owned());  // to_owned() 在 &str 上返回 String 类型
    string("nice weather".into());  // into() 在这个上下文中转换为 String 类型
    string(format!("Interpolation {}", "Station"));  // format! 宏返回 String 类型
    string_slice(&String::from("abc")[0..1]);  // 字符串切片操作返回 &str 类型
    string_slice("  hello there ".trim());  // trim() 方法返回 &str 类型
    string("Happy Monday!".to_string().replace("Mon", "Tues"));  // replace() 方法返回 String 类型
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());  // to_lowercase() 返回 String 类型
}