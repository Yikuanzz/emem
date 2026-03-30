struct Rectangle {
    width: u32,
    height: u32,
}

// 给 Rectangle 实现 new 构造函数
impl Rectangle {
    // 关联函数：创建实例
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

// 1. 定义一个特征
// pub 代表公开的 trait，可以被其他模块使用
pub trait Summary {
    // 必须实现的方法
    fn summarize(&self) -> String;

    // 带有默认实现的方法
    fn preview(&self) {
        println!("Here's a preview of the item...");
    }
}

// 2. 为 Rectangle 实现 Summary 特征
impl Summary for Rectangle {
    fn summarize(&self) -> String {
        format!("A rectangle of {}x{}", self.width, self.height)
    }
    // preview 使用默认实现
}

// 3. 为其他类型实现特征
struct NewsArticle {
    headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News: {}", self.headline)
    }
}

// 这个函数可以接受任何实现了 Summary 特征的类型
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let rect = Rectangle::new(5, 10);
    let article = NewsArticle { headline: String::from("Rust is awesome!") };

    notify(&rect);    // 正确！Rectangle 实现了 Summary
    notify(&article); // 正确！NewsArticle 也实现了 Summary
}