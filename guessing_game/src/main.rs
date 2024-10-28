use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("开始游戏!");
    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("随机生成：{secret_number}");
    loop {
        println!("猜猜看");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("无法读取行");
        //类型转换 str->u32 trim()去除前后空格或者换行，parse()根据:转换成：后面的类型expect错误处理
        let guess: u32 = guess.trim().parse().expect("请输入数字！");
        println!("输入{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("🤷‍♂️太小了🤷‍♂️"),
            Ordering::Greater => println!("🤦‍♂️太大了🤦‍"),
            Ordering::Equal => {

                break;
            }
        }
    }
    println!("恭喜你🎇🎇🎇🎇🎇🎇🎇🎇猜对了🤣🤣");
}
