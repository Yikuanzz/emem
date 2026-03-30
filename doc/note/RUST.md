# Rust 学习笔记

## STAGE_1：所有权（Ownership）与 借用（Borrowing）

### 所有权的转移

在 Rust 中，每个值都会有自己的所有者，当所有者离开 **作用域** 的时候，值就会被丢弃（drop）。

```rust
fn main() {
    // 创建一个 String 类型的数据（堆上分配）
    let s1 = String::from("hello");
    
    // 将 s1 的所有权移动给 s2
    let s2 = s1; 
    
    // 下面这行代码会报错！因为 s1 的所有权已经移给了 s2，s1 失效了。
    // println!("{}", s1); // 编译错误: value borrowed here after move
    
    // 但 s2 是可以使用的
    println!("{}", s2); // 输出: hello
}
```

### 临时借用所有权

借用数据，而不改变它的所有权归属，需要用 `&` 来创建引用。

```rust
fn main() {
    let s1 = String::from("hello");

    // &s1 创建了 s1 的引用（借用），所有权没有转移
    let len = calculate_length(&s1);

    // s1 依然有效，因为所有权还在 main 函数手里
    println!("The length of '{}' is {}.", s1, len);
}

// 函数接收一个字符串的引用 (&String)，而不是拥有它
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 可变借用

默认引用是只读、不可变的，如果想要可变就要用 `&mut`，与此同时为了防止数据竞争会有这样的限制：同一个作用域下，要么有 **多个不可变引用**，要么只有一个 **可变引用**，二者不可同时存在。

```rust
// 这段代码无法编译
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用 (允许，多个读没问题)
    
    // let r3 = &mut s; // 编译错误！不能在已有不可变引用时创建可变引用
    
    println!("{}, {}, and {}", r1, r2, "cannot get mutable reference here");
}
```

## STAGE_2：结构体（Structs）、枚举（Enums）和 模式匹配

### 结构体

与其他编程语言一样，这里主要熟悉一下在 Rust 中的语法结构。

```rust
// 1. 定义结构体
// 加上 #[derive(Debug)] 即可支持打印
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,  
    active: bool,
}

fn main(){
    // 2. 创建结构体实例
    let user1 = User{
        username: String::from("someusername123"),
        email: String::from("someone@example.com"), 
        active: true,
        sign_in_count: 1,
    };

    // 访问字段
    println!("User email is: {}", user1.email);

    // 3. 创建可变实例
    let mut user2 = User{
        username: String::from("someusername123"),
        email: String::from("someone@example.com"), 
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("newemail@example.com");

    // 4. 合法打印结构体（两种方案）
    // 方案1：手动打印字段（最简单）
    println!("User2 邮箱: {}", user2.email);
    println!("User2 用户名: {}", user2.username);
    
    // 方案2：派生 Debug trait，直接打印整个结构体
    println!("User2 完整信息: {:?}", user2);
    // 美化打印（换行+缩进）
    println!("User2 美化版: {:#?}", user2);
}
```

### 枚举

Rust 中的枚举，允许每个值为不同类型。

```rust
// 定义一个枚举，它的变体可以包含不同类型的数据
enum Message {
    Quit,                       // 1. 不包含任何数据
    Move { x: i32, y: i32 },    // 2. 包含结构体字段
    Write(String),              // 3. 包含一个 String
    ChangeColor(i32, i32, i32), // 4. 包含三个 i32
}

fn main() {
    // 创建不同变体的枚举实例
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    // 测试调用
    process_message(m3); 
}

// 处理消息的函数（match 必须覆盖所有变体）
fn process_message(message: Message) {
    match message {
        Message::Quit => {
            println!("Quit 消息");
        }
        Message::Move { x, y } => {
            println!("Move to ({}, {})", x, y);
        }
        Message::Write(s) => {
            println!("Write 消息：{}", s);
        }
        Message::ChangeColor(r, g, b) => {
            println!("ChangeColor 颜色：({}, {}, {})", r, g, b);
        }
    }
}
```

### 模式匹配

`match` 表达式可以根据不同的枚举类型来去进行不同处理。

```rust
fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("This coin is worth {} cents.", value_in_cents(coin));
}

// 定义一个更具体的枚举
#[derive(Debug)] 
enum UsState {
    Alaska,
    NewYork,
    // ... 其他州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币还携带了州的信息
}

// 使用 match 表达式来处理 Coin 枚举
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 这里我们解构了 Quarter 变体，提取出 state 字段
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

## STAGE_3：方法（Methods）与 特征（Traits）

### 方法

方法与普通函数的关键区别在于，方法的第一个参数总是 `self`（或 `&self`、`&mut self`），这代表了调用该方法的那个实例本身。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 这是一个关联函数（常用作构造函数），它不接收 self
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // 这是一个方法，通过 &self 借用实例来读取数据
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 这是一个可变方法，通过 &mut self 借用实例来修改数据
    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    // 使用关联函数创建实例
    let mut rect = Rectangle::new(10, 20);

    // 使用方法
    println!("The area is: {}", rect.area()); // 输出: The area is: 200

    rect.set_width(30);
    println!("The new area is: {}", rect.area()); // 输出: The new area is: 600
}
```

### 特征

特征就是类似于其他语言的中的 `interface`，它定义了一组方法签名，任何类型都可以选择实现这个特征来获得这组行为。

```rust
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

fn main() {
    // 现在可以正常调用了
    let rect = Rectangle::new(5, 10);
    let article = NewsArticle { headline: String::from("Rust is awesome!") };

    println!("{}", rect.summarize());
    println!("{}", article.summarize());
    
    rect.preview();
}
```

### 特征作为约束（Trait Bounds）

特征最强大的用途之一是作为泛型的约束。你可以编写一个函数，它不接受某个具体类型，而是接受“任何实现了特定特征的类型”。

```rust
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
```
