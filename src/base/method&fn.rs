#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 结构体的方法是在 impl 块中定义的，并且以 &self、&mut self 或 self 作为第一个参数
impl Rectangle {
    // 检查宽度是否非零
    fn has_width(&self) -> bool {
        self.width > 0
    }
    // 计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 创建正方形矩形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // 判断是否能容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.has_width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // 调用 square 结构体关联函数创建一个新的正方形矩形
    let square_rect = Rectangle::square(20);
    println!("Square rectangle: {:?}", square_rect);

    // 判断 rect1 是否能容纳 square_rect
    if rect1.can_hold(&square_rect) {
        println!("rect1 can hold square_rect");
    } else {
        println!("rect1 cannot hold square_rect");
    }
}
