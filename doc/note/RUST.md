# Rust 学习笔记

## Rust 基础 TIP

### Rust 项目结构

1、`main.rs` 或者 `lib.rs` 是项目根（`crate::`）；
2、一个 `.rs` 文件 = 一个模块；
3、一个文件夹 = 一个模块组；
4、如果想让文件能被外部访问，需要添加 `pub` 在 `mod` 中声明；
5、导入用 `use 路径::名称;`，根路径为 `crate::`。

- `crate::`从项目根开始
- `self::`从当前模块开始
- `super::`从父模块开始

```shell
your_project/
├── Cargo.toml
└── src/
    ├── main.rs               # 项目根 crate::
    ├── utils.rs              # 模块 crate::utils
    ├── config.rs             # 模块 crate::config
    └── data/                 # 文件夹模块
        ├── mod.rs            # 文件夹入口（必须）
        ├── list.rs           # crate::data::list
        └── user.rs           # crate::data::user
```

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

## STAGE_4：错误处理（Error Handling）

### 不可空值与 `Option<T>`

Rust 没有 `null`，如果一个值可能不存在，就必须用 `Option<T>` 枚举，它有两个表示：`Some(T)` 和 `None`。

```rust
fn main() {
    // Some(5) 表示有一个值，是 5
    let some_number = Some(5);
    
    // None 表示没有值
    let absent_number: Option<i32> = None;

    // 你不能直接把 Option<i32> 当作 i32 使用
    // let x: i32 = some_number; // 编译错误！

    // 必须处理“有值”和“无值”的情况
    match some_number {
        Some(val) => println!("Got a value: {}", val),
        None => println!("Got nothing"),
    }
}
```

### 可恢复错误与 `Result<T, E>`

当操作可能会失败的时候（比如打开文件、解析字符串），Rust 返回 `Result<T, E>` 其中 `Ok(T)` 表示操作成功返回结果值，`Err(E)` 表示操作失败包含错误信息。

```rust
use std::fs::File;

fn main() {
    // 尝试打开一个不存在的文件
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => {
            println!("File opened successfully!");
            file
        },
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        },
    };
}
```

### 快速失败 `?` 运算符

因为用 `match` 比较繁琐，Rust 提供了 `?` 运算符作为语法糖，如果结果是 `Ok` 就提取值，如果是 `Err` 就返回错误给调用者。

```rust
use std::fs::File;
use std::io::{self, Read};

// 这个函数返回一个 Result，错误类型是 io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    // 如果 File::open 失败，? 会直接返回 Err，函数结束
    // 如果成功，? 提取出 File 对象赋值给 f
    let mut f = File::open("username.txt")?;
    
    let mut s = String::new();
    
    // 同样，如果读取失败，? 会直接返回错误
    // 读取文件内容到字符串
    // 失败 → return Err
    // 成功 → 继续
    f.read_to_string(&mut s)?;
    
    // 成功：把字符串包在 Ok 里返回
    Ok(s)
}

// 注意：main 函数也可以使用 Result，但这通常用于测试或简单程序
fn main() -> Result<(), io::Error> {
    let username = read_username_from_file()?;
    println!("Username: {}", username);
    Ok(())
}
```

## STAGE_5：泛型、特质边界与生命周期（Generics, Trait Bouds & Lifetimes）

### 泛型定义

Rust 的泛型会在编译的时候将代码“特化”，编译器会自动生成过分对应具体类型代码，让其在运行时没有性能损耗。

```rust
// 定义泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // impl 中可以定义泛型方法
    fn x(&self) -> &T{
        &self.x
    }
}

fn main(){
    let integer_point = Point{x:5, y:10};

    let float_point = Point{x: 1.0, y: 4.0};

    println!("Integer point x: {}", integer_point.x())
}
```

### 特质边界

泛型很强大，但有时我们需要限制泛型的能力。比如，你想写一个函数找出列表中的最大值，那么列表中的类型必须是可以比较大小的。这时就需要特质边界。

```rust
use std::fmt::Display;

// 场景：我们要打印一个东西，并返回它。
// 约束 1: T 必须实现 Display (为了能打印)
// 约束 2: T 必须实现 Clone (为了能复制返回，假设我们不移动所有权)

// 写法 A：impl Trait (适合简单场景)
fn print_and_return(item: &impl Display) {
    println!("Displaying: {}", item);
}

// 写法 B：Trait Bound (适合复杂场景)
fn notify<T: Display + Clone>(item: &T) {
    println!("Announcement! {}", item);
}

// 写法 C：where 子句 (当约束太复杂时，让签名更清晰)
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}
```

### 生命周期

Rust 的生命周期是告诉编译器：引用之间谁和谁活一样久，帮编译器检查安全，避免野指针。注意：返回的引用，生命周期不能超过任何一个输入引用的生命周期。(保证：返回值活着的时候，它指向的数据一定还活着)

```rust
// 错误示范：编译器不知道返回的引用 'x 还是 'y 的有效期
// fn longest(x: &str, y: &str) -> &str { ... } // 编译报错！

// 正确示范：显式告诉编译器，输入和输出共享同一个生命周期 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");
        // string1 和 string2 的生命周期都必须长于 'a
        result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    } // string2 在这里被 drop 了，但 result 指向 string1，所以 result 依然有效
}
```

### 生命周期省略规则 (Lifetime Elision)

不需要在所有地方都写 `'a`。Rust 有三条规则，如果编译器能推断出来，就可以省略标注。

- 1、每个引用参数都有一个独立的生命周期。
- 2、如果只有一个输入生命周期，它被赋给所有输出生命周期。
- 3、如果有 `&self` 或 `&mut self`，输出生命周期被赋给 `self` 的生命周期。

```rust
// 'a 是 ImportantExcerpt 类型的一部分
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 这里不需要写生命周期，因为规则 3 生效：
    // 返回值的生命周期等于 &self 的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}
```

## STAGE_6：智能指针与并发（Smart Pointers & Concurrency）

### 智能指针

智能指针不仅仅是存储地址的变量，它们还拥有元数据（如引用计数）或能力（如自动释放）。Rust 标准库提供了三种核心的智能指针，分别对应不同的场景。

1、`Box<T>`：简单的堆分配，独占所有权，一般用于递归类型的数据结构上。栈上只存指针（固定大小），真正的数据存在堆上（动态大小）。

- 在 64 位系统上，这个指针永远占用 8 字节；
- 不管指向多大的数据，指针本身大小不变。

```rust
// 如果没有 Box，编译器无法确定 Cons 的大小，因为它无限递归
enum List {
    Cons(i32, Box<List>), // Cons 包含一个 i32 和 指向下一个 List 的指针
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 1 -> 2 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}
```

2、`Rc<T>`：引用计数（单线程），所有权是共享的，一般场景比如图结构中多个节点指向同一个节点。

```rust
use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    
    // b 和 c 都共享 a 的所有权
    let b = Cons(3, Rc::clone(&a)); 
    let c = Cons(4, Rc::clone(&a));

    // 打印引用计数，此时 a 被引用了 3 次 (a, b, c)
    println!("Rc strong count: {}", Rc::strong_count(&a)); 
}
```

3、`RefCell<T>`：有时想在一个拥有不可变引用的地方修改数据，它会将借用的检查从编译时转移到运行时。通常会用 `Rc<RefCell<T>>`，来实现多所有者且可修改。

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // 多个所有者共享，并且可以修改数据
    let shared_data = Rc::new(RefCell::new(10));
    
    let a = Rc::clone(&shared_data);
    let b = Rc::clone(&shared_data);
    
    // 任意一个所有者都能修改内部值
    *a.borrow_mut() += 5;
    *b.borrow_mut() += 5;
    
    println!("{}", shared_data.borrow()); // 20
}
```

### 无畏并发

TODO：....
