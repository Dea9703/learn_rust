enum IpAddrKind {
    V4,
    V6
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
enum IpAddress {
    V4(String),
    V6(String),
}

enum ipAddress2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum Option<T> {
//     // 是一个泛型类型参数, <T> 意味着 Option 枚举的 Some 成员可以包含任意类型的数据
//     None,
//     Some(T),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let localhost = IpAddress::V4(String::from("127.0.0.1"));

    let localhost2 = ipAddress2::V4(127, 0, 0, 1);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x = 4;
    let y = Some(4);
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn route(ip_kind: IpAddrKind) {

}
