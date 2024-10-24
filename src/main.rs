#[derive(Debug)]
struct Person{
    name:String,
    age:u8,
}
// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name=String::from("peter");
    let age=18;
    let peter = Person{name,age};
    println!("{:?}",peter)
}