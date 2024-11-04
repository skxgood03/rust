// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     //查找 输入合适带属性执行响应的返回
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     println!("{}", value_in_cents(Coin::Quarter))
// }



//绑定值的模式-----------------------------------------------
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState), //Quarter中包含UsState枚举类型
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);  //Alaska!
//             25
//         }
//     }
// }
//
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

//匹配 Option<T>-------------------------------------------

// fn main() {
//     fn plus_one(x:Option<i32>) ->Option<i32>{
//
//         match x{
//             None=>None,
//             Some(i)=>Some(i+1)
//         }
//     }
//     let five = Some(5);
//     let six = plus_one(five);
//     println!("{:?}",six); //Some(6)
//     let none = plus_one(None);
//     println!("{:?}",none); //None
// }

// 通配模式和 _模式

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 简单来说 other就是其他值  如9就会走到这
        other => move_player(other),
        //_ 当我们不想使用通配模式获取的值时，
        // 请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
        // 这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。


        // _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}