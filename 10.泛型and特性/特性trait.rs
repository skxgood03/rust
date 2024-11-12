/*特性

trait Descriptive {
    fn describe(&self) -> String;
}
// Descriptive 规定了实现者必需有 describe(&self) -> String 方法。
// 用它实现一个结构体：
struct Person {
    name: String,
    age: u8
}
// impl <特性名> for <所实现的类型名>
impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
*/

/*

// 默认特性
// 特性可以定义方法作为默认方法，因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法：

trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}
struct Person {
    name: String,
    age: u8
}
impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe()); //Cali 24
    // 如果我们将 impl Descriptive for Person 块中的内容去掉，那么运行结果就是： [Object]
}
*/

/*特性做参数*/
// 很多情况下我们需要传递一个函数做参数，例如回调函数、设置按钮事件等。在 Java 中函数必须以接口实现的类实例来传递，在 Rust 中可以通过传递特性参数来实现：
// trait Descriptive {
//     fn describe(&self) -> String {
//         String::from("[Object]")
//     }
// }
// fn output(object: impl Descriptive) {
//     println!("{}", object.describe());
// }

trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object { 1 }
        else if &self == &object { 0 }
        else { -1 }
    }
}

fn main() {
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
}