use std::io;
fn main() {
    println!("请输入化石摄氏度或者摄氏度，他们将转换彼此");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        println!("输入的值是: {}", input);
        // 去除末尾的换行符和空格
        let input = input.trim();
        let my_str: &str = &input;
        let len = my_str.len();
        // 获取最后一个字符，作为单位
        let unit = &my_str[len-1..];
        // 获取前面的浮点数部分
        let value_str = &my_str[..len-1];
        // 解析浮点数
        let value: f64 = value_str.parse().expect("Not a valid float number");
        println!("Value: {}, Unit: {}", value, unit);
        if unit == "C"||unit == "c" {
            println!("{:.2}°C = {:.2}°F", value, celsius_to_fahrenheit(value));
        }
        else if unit == "F"||unit == "f" {
            println!("{:.2}°F = {:.2}°C", value, fahrenheit_to_celsius(value));
        }


    }
}
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}
