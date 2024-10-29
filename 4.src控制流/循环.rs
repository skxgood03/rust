//使用 loop 重复执行代码
// fn main() {
//     loop {
//         println!("again!");
//     }
// }


//嵌套loop
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {}", count);
// }

//从loop中返回
// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//
//     println!("The result is {}", result);
// }

// while 条件循环
// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{}!", number);
//
//         number -= 1;
//     }
//
//     println!("LIFTOFF!!!");
// }

//使用 for 遍历集合

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is: {}", element);
    }
}