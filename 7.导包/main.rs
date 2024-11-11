//引用 包
// use std::collections::HashMap; //外部引用
//
// fn main() {
//     use std::fmt; //内部引用
//     use std::io;
//     let mut map = HashMap::new();
//     map.insert(1, 2);
//
//     fn function1() -> fmt::Result {
//         // --snip--
//         Ok(())
//     }
//
//     fn function2() -> io::Result<()> {
//         // --snip--
//         Ok(())
//     }
// }

// 使用 as 重命名=============================================

// #![allow(unused)]
// fn main() {
//     // 两个不同函数相同的包名可用as 区分
//     use std::fmt::Result;
//     use std::io::Result as IoResult;
//
//     fn function1() -> Result {
//         // --snip--
//         Ok(())
//     }
//
//     fn function2() -> IoResult<()> {
//         // --snip--
//         Ok(())
//     }
// }

// 化简引用=============================================、

    // use std::cmp::Ordering;
    // use std::io;
// 或
    //use std::{cmp::Ordering, io};

    // use std::io;
    // use std::io::Write;
// 或
    //use std::io::{self, Write};


//导入所有
use std::collections::*;

fn main() {

}