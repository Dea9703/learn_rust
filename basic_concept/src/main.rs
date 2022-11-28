fn main() {
    data_type();
    another_function();
    parameter_function(12);
    let return_value = return_function(5, 6);
    println!("return_value = {return_value}");
    control_flow()
}

// 控制流
fn control_flow() {
    // if 表达式
    // 条件表达式必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。
    let number = 3;
    if number < 5 {
        println!("first condition was true");
    } else if number < 10 {
        println!("second condition was true");
    } else {
        println!("other condition was false");
    }

    // 在 let 语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");


    // 循环
    // Rust 有三种循环：loop、while 和 for。

    // loop
    // break 停止循环
    let mut counter = 0;
    let result = loop {
        println!("again!");
        counter += 1;
        if counter == 10 {
            break counter * 2; // 从循环返回值
        }
    };
    println!("result = {result}");

    // while
    // 当条件为真，执行循环。当条件不再为真，调用 break 停止循环。
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    // 遍历一个迭代器 iterator
    for element in a {
        println!("the value is: {element}");
    }

    // 遍历一个范围 range
    for element in 1..4 {
        println!("the value is: {element}"); // 1 2 3
    }


}

/* 函数 start */

// main 函数，它是很多程序的入口点。
// fn 关键字，它用来声明新函数。
fn another_function() {
    println!("Another function.");
}

// 参数
fn parameter_function(x: i32) {
    println!("The value of x is: {x}");
}

// 具有返回值的函数
// 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头 -> 后声明它的类型。
// 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
// 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
fn return_function(x: i32, y: i32)-> i32 {
    x + y
}

/* 函数 end */ 

// 数据类型
fn data_type() {
    // 不可变变量
    let x = 5;
    println!("x的值是: {x}");
    // 可变(mut)变量
    let mut y = 5;
    println!("y的值是: {y}");
    y = 10;
    println!("y的值是: {y}");

    // 常量
    // 类似于不可变变量，不允许对常量使用 mut
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // 隐藏(Shadowing)
    // 可以用相同变量名称来隐藏前一个变量，以及重复使用 let 关键字来多次隐藏
    let z = 10;
    println!("z的值是: {z}");
    let z = "六";
    println!("z的值是: {z}");

    /* 标量类型 start */
    // 标量（scalar）类型代表一个单独的值。
    // Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型 

    // 整型
    let num = 98_222; // Decimal (十进制)
    let num = 0xff; // Hex (十六进制)
    let num = 0o77; // Octal (八进制)
    let num = 0b1111_0000; // Binary (二进制)
    let num = b'A'; // Byte (单字节字符)(仅限于u8)

    // 整型溢出
    // let num: u8 = 256; // literal out of range for `u8`

    // 浮点型 默认类型是 f64
    // 所有的浮点型都是有符号的
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数值运算
    // 加
    let sum = 5 + 10;

    // 减
    let difference = 95.5 - 4.3;

    // 乘
    let product = 4 * 30;

    // 除
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // 取余
    let remainder = 43 % 5;

    // 布尔型
    let t = true;
    let f: bool = false;

    // 字符类型
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", c, z, heart_eyed_cat);

    /* 标量类型 end */

    /* 复合类型 start */
    // 复合类型（Compound types）可以将多个值组合成一个类型。
    // Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // 元组长度固定：一旦声明，其长度不会增大或缩小。
    // 不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，
    // 表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

    // 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组
    let tup = (500, 6.4, false, "hello");
    // 解构
    let (w, x, y, z) = tup;
    println!("w = {}, x = {}, y = {}, z = {}", w, x, y, z);
    // 使用点号（.）后跟值的索引来直接访问它们
    // 元组的第一个索引值是 0
    let five_hundred = tup.0;
    println!("five_hundred = {}", five_hundred);

    // 数组类型
    // 与元组不同，数组中的每个元素的类型必须相同。Rust中的数组长度是固定的。
    let a = [1, 2, 3, 4, 5];
    // 访问数组元素
    let first = a[0];
    let second = a[1];
    // let sixth = a[5]; // 索引越界，运行时错误

    let byte = [0; 8]; // 创建一个具有8个元素的数组，其中所有元素都为0

     /* 复合类型 end */
}

