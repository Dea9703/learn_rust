#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl Rectangle {
    fn area4(&self)-> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle)-> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数
    fn square(size: u32) -> Self{
        Self { 
            width:size,
            height: size
        }
    }
}

fn main() {
    structure();
    tuple_structs();

    let width = 50;
    let height = 30;
    println!("area1 = {}", area1(width, height));
    // 使用元组重构
    let rect1 = (30, 50);
    println!("area2 = {}", area2(rect1));

    // 使用结构体重构：赋予更多意义
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("rect2 = {:#?}", rect2);
    println!("area3 = {}", area3(&rect2));

    // 在结构体上实现方法
    println!("area4 = {}", rect2.area4());

    let rect3 = Rectangle {
        width: 10,
        height: 10,
    };
    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    // 关联函数实现Rectangle
    let rect5 = Rectangle::square(25);
    println!("rect5: {:#?}", rect5)


}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area1(width: u32, height: u32)-> u32 {
    width * height
}



struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn structure() {
    let mut user1 = User {
        username: String::from("dea"),
        email: String::from("dea@example.com"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username;
    println!("name = {}", name);
    user1.username = String::from("dea1");
    println!("user1.username = {}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    let user2 = build_user(String::from("hello"), String::from("dea2@email.com"));
    println!("user2 = {}", user2.username);

    let user3 = User { 
        username: String::from("dea3"),
        email: String::from("dea3@email.com"),
        ..user2
    };
    println!("user3 = {}", user3.active);
}

fn build_user(username: String, email: String)-> User {
    User { 
        username,
        email,
        active: false,
        sign_in_count: 0,
     }
}