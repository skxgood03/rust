use std::fs::File;
use std::io::ErrorKind;
// fn main() {
//     let f = File::open("hello.txt");
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("Problem opening the file: {:?}", error)
//         },
//     };
// }
// 使用不同的方式处理不同类型的错误（没有文件则创建）
// fn main() {
//     let f = File::open("hello.txt");
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error),
//         },
//     };
// }

// 改进上面的方法--------------删除match---------------------------------------
// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }


//失败时 panic 的简写：unwrap 和 expect 简写异常处理
// match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好地表明其意图


// fn main() {
//     // unwrap() 有问题可直接输出报错
//     let f = File::open("hello.txt").unwrap();
// }

// expect自定义报错信息
// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

//传播
// 当编写一个需要先调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理

/*读取用户名的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：*/
// fn main() {
//     use std::io;
//     use std::io::Read;
//     use std::fs::File;
//
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let f = File::open("hello.txt");
//
//         let mut f = match f {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };
//
//         let mut s = String::new();
//
//         match f.read_to_string(&mut s) {
//             Ok(_) => Ok(s),
//             Err(e) => Err(e),
//         }
//     }
// }

// 传播错误的简写：? 运算符


// fn main() {
//     use std::io;
//     use std::io::Read;
//     use std::fs::File;
//
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut f = File::open("hello1.txt")?;
//         let mut s = String::new();
//         f.read_to_string(&mut s)?;
//         Ok(s)
//     }
// }

// fn main() {
//     use std::io;
//     use std::io::Read;
//     use std::fs::File;
//
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut s = String::new();
//
//         File::open("hello.txt")?.read_to_string(&mut s)?;
//
//         Ok(s)
//     }
// }

// 更短的写法：

// fn main() {
//     use std::io;
//     use std::fs;
//
//     fn read_username_from_file() -> Result<String, io::Error> {
//         fs::read_to_string("hello.txt")
//     }
// }

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}


