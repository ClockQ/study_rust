fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");     // 我们必须将 s1 改为 mut
    change(&mut s1);                    // 然后必须在调用 change 函数的地方创建一个可变引用 &mut s1
    println!("使用可变引用修改后的字符串 {}.", s1);

    // 引用作用域
    let mut s1 = String::from("hello");
    let r1 = &s1; // 没问题
    let r2 = &s1; // 没问题
    // let r3 = &mut s1; // 大问题，cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("r1={}, r2={}", r1, r2);   // 最后一次使用 r1、r2 ，所以这之后 r1、r2 退出作用域
    let r3 = &mut s1; // 没问题
    println!("r3={}", r3);
    let r4 = &s1; // 没问题
    println!("r4={}", r4);
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

fn change(some_string: &mut String) {       // 并更新函数签名以接受一个可变引用 some_string: &mut String
    some_string.push_str(", world");
}