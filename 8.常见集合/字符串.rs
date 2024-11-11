
#![allow(unused)]
fn main() {
    // 使用 to_string 方法从字符串字面量创建 String==================
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面量：==============================
    let s = "initial contents".to_string();

    // 使用 String::from 函数从字符串字面量创建 String===================

    let s = String::from("initial contents");

    // 使用 push_str 和 push 附加字符串=========================
    let mut s = String::from("foo");
    s.push_str("bar") ;//foobar


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); //bar

    // 使用 push 将一个字符加入 String 值中  push 仅支持单一字符=========================
    let mut s = String::from("lo");
    s.push('l');

    // 使用 + 运算符或 format! 宏拼接字符串============================
    /*s2 使用了 &，意味着我们使用第二个字符串的 引用 与第一个字符串相加。这是因为 add 函数的 s
     参数：只能将 &str 和 String 相加，不能将两个 String 值相加。*/
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用 Hello, world!



    // 索引字符串++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let s1 = String::from("hello");
    let h = s1[0]; //直接访问会报错
    let len = String::from("Hola").len(); //先获取长度再根据长度访问

}
