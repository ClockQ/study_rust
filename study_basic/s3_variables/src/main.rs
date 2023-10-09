fn main() {
    // 标量类型
    let i = 32;
    let f = 3.14;  
    let b = true;
    let c = 'C';
    println!("i={}, f={}, b={}, c={}", i, f, b, c);

    // 数字运算
    // addition
    let sum = 5 + 10;
    // subtraction
    let sub = 95.5 - 4.3;
    // multiplication
    let mul = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;
    println!("sum={}, sub={}, mul={}, quo={}, fl={}, rem={}", sum, sub, mul, quotient, floored, remainder);

    // 元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup={:?}, tup1={}, tup2={}, tup3={}", tup, tup.0, tup.1, tup.2);
    println!("x={}, y={}, z={}", x, y, z);

    // 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr={:?}, arr[2]={}", arr, arr[2]);
    let arr = [3; 5]; // 等于 let a = [3, 3, 3, 3, 3];
    println!("arr={:?}, arr[2]={}", arr, arr[2]);

    // 调用函数
    println!("result={}", plus(5, 6));

    // 语句和表达式
    let x = 5;
    let x = {
        let _y = 7;
        println!("y={}", y);
        x + 1 // 注意，这里不能加 ; 否则表达式会转语句，也就会导致 x 没有值 ()
    };
    println!("x={}", x);

    // 开关语句
    let number = if true { 5 } else { 6 };
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 循环返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result={}", result);

    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    
    // for 循环倒计时
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// 函数定义
fn plus(x: i32, y: i32) -> i32 {
    println!("Another function.");
    x + y
}