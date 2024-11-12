/*
结构体定义中的泛型

struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    // 不会报错xy都属于同种类型 泛型T
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // 会报错
    let wont_work = Point { x: 5, y: 4.0 };

}

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // 都不会报错 x: T y: U, 不属于同种类型
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
*/

/*枚举定义中的泛型
enum Option<T>{
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

/*方法定义中的泛型

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
*/
struct Point<T, U> {
    x: T,
    y: U,
}
impl <T,U>Point<T,U>{
    fn mixup<V,W>(self,other:Point<V,W>) ->Point<T,W>{
        Point{
            x:self.x,
            y:other.y,
        }
    }
}
fn main(){
    let p1 = Point{x:5,y:10.4};
    let p2 = Point{x:"Hello",y:"C"};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y)
}

