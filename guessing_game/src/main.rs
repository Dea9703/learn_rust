use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("猜数字游戏1.0");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个数字: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("读取用户输入出错！");
    
        println!("你输入的数字是: {guess}");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
    }
}
