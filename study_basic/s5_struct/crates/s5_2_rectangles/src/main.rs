fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("初版实现，面积为 {}", area(width1, height1));

    let rect = (30, 50);
    println!("元组重构，面积为 {}", area_tuple(rect));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("结构体重构，结构体为 {:#?}\n面积为 {}", rect, area_struct(&rect));

    println!("方法重构，面积为 {}", dbg!(rect.area())); // 调用方法

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect 包含 rect2 吗? {}", rect.can_hold(&rect2));
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };
    println!("rect 包含 rect3 吗? {}", rect.can_hold(&rect3));

    let square = Rectangle::square(30);
    println!("创建一个正方形，结构体为 {:#?}\n面积为 {}", square, square.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 结构体中都是简单类型，所以直接传对象也可以
// 但考虑到结构体如果复杂一点，包含有复杂类型，那此处传对象会导致移交
fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // 定义到此结构的上下文中
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle { // 可以分开定义
    fn can_hold(&self, other: &Self) -> bool { // Self 类型是 Rectangle 别名
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}