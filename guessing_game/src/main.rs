use std::io;
use rand::Rng;
fn main() {
    println!("开始游戏!");
    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("随机生成：{secret_number}");
    println!("请输出");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("无法读取行");
    println!("输入{guess}");
}
