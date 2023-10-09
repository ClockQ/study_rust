use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1..101);
    println!("随机数字是 {}", num);
    
    loop {
        println!("请输入一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入错误");
        // println!("猜测数字是 {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };
        match guess.cmp(&num) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        };
    }
}