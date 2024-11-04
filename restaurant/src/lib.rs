//
#![allow(unused)]
// fn main() {
//     mod front_of_house {
//         //pub表示外部可以访问
//         pub mod hosting {
//             // hosting模块的内容也要加pub 才能被外部访问
//             pub   fn add_to_waitlist() {
//                 println!("Added to waitlist")
//             }
//
//             fn seat_at_table() {}
//         }
//
//         mod serving {
//             fn take_order() {}
//
//             fn serve_order() {}
//
//             fn take_payment() {}
//         }
//     }
//     pub fn eat_at_restaurant() {
//         // 父访问子 子必须要加pub
//         // Absolute path
//         crate::front_of_house::hosting::add_to_waitlist();
//
//         // Relative path
//         front_of_house::hosting::add_to_waitlist();
//     }
// }

// 创建公有的结构体和枚举
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 更改我们想要的面包
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 不能修改以下 seasonal fruit 因为它被标记为私有的
    // meal.seasonal_fruit = String::from("blueberries");
}

// 枚举   枚举在模块中定义pub那么子类全都是公有

mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant2() {
    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;
}


// use 导入模块=============================================================
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// use crate::front_of_house::hosting;
// 第二种

use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant3() {
    // hosting::add_to_waitlist()
    add_to_waitlist()
}
