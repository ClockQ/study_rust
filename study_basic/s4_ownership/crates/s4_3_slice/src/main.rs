fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    println!("源字符串 {} 的第一个单词结束下标为 {}, 第一个单词为 {}", s, word, s.split_at(word).0); 
    s.clear(); // 这清空了字符串，使其等于 ""

    let s = String::from("hello world");
    let result = first_word_for_slice(&s);
    println!("源字符串 {} 的第一个单词为 {}", s, result);
}

fn first_word(s: &String) -> usize {  // 函数有一个参数只读引用 &String ，因为不需要所有权，所以这没有问题
    let bytes = s.as_bytes();  // 转化为字节数组

    for (i, &item) in bytes.iter().enumerate() { // 创建一个迭代器
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_for_slice(s: &str) -> &str { // “字符串 slice” 的类型声明写作 &str
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    };
    return &s[..]
}

