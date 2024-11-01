// 枚举

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    // 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
//-------------------------------------进阶
//枚举可以存结构体
// struct Ipv4Addr{
//
// }
// struct Ipv6Addr{
//
// }
// enum IpAddr2 {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }


// 一个 Message 枚举，其每个成员都存储了不同数量和类型的值

/*
这个枚举有四个含有不同类型的成员：

Quit 没有关联任何数据。
Move 包含一个匿名结构体。
Write 包含单独一个 String。
ChangeColor 包含三个 i32。
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

