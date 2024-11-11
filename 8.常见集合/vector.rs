
#![allow(unused)]
// vector 在其离开作用域时会被释放
// fn main() {
//
//     let mut v = Vec::new();
//
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("{:?}", v); //[5, 6, 7, 8]
//
//     let third: &i32 = &v[2];
//     println!("The third element is {}", third); //The third element is 7
//     //也可使用get(索引)获取
//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     } //The third element is 7\
//     //超出索引范围会报错
//     // let does_not_exist = &v[100];
//     // let does_not_exist = v.get(100);
//
// } // <- 这里 v 离开作用域并被丢弃


// 使用枚举来储存多种类型+++++++++++++++++++++++++++++++++++++++++++++++++++++

#![allow(unused)]
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
