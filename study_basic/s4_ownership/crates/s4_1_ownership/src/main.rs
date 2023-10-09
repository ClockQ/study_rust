fn main() {
    string();
    var_int_move();
    string_move();
    string_clone();
    ownership();
    return_ownership();
}

fn _scope() {
    // s 在这里无效, 它尚未声明 
    let s = "hello";    // 从此处起，s 开始有效
    println!("{}", s)
}   // 此作用域已结束，s 不再有效

fn string() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`
}

fn var_int_move() {
    let x = 5;              // 将 5 绑定到 x
    let y = x;              // 接着生成一个值 x 的拷贝并绑定到 y
    println!("x={}, y={}", x, y) // 现在有了两个变量，x 和 y，都等于 5
}

fn string_move() {
    let s1 = String::from("hello"); // 
    let s2 = s1;
    println!("s1={}, s2={}", "s1报错", s2) // 现在有了两个变量，x 和 y，都等于 5
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership() {
    let s = String::from("hello");       // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里，所以到这里不再有效
  
    let x = 5;                      // x 进入作用域
    makes_copy(x);         // x 应该移动函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
}
  
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("some_string = {}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("some_integer = {}", some_integer);
}

fn return_ownership() {
    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3
    println!("s1={}, s2={}, s3={}", s1, "s2报错", s3)
}
  
fn gives_ownership() -> String {           // gives_ownership 将返回值移动给调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {   // a_string 进入作用域
    a_string + " returned"                               // 返回 a_string 并移出给调用的函数
}
