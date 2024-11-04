// 传统match--------------------------------------
// fn main() {
//     let some_u8_value = Some(0u8);
//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => (),
//     }
// }


// let mut count = 0;
// match coin {
// Coin::Quarter(state) => println!("State quarter from {:?}!", state),
// _ => count += 1,
// }

//简写  if let------------------------------------------

fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}


// let mut count = 0;
// if let Coin::Quarter(state) = coin {
// println!("State quarter from {:?}!", state);
// } else {
// count += 1;
// }