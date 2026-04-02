enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let color = Color::Rgb(255, 0, 0);
    let point = Point { x: 10, y: 20 };

    match color {
        // 直接解构枚举，并提取值
        Color::Rgb(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::Hsv(h, s, v) => println!("Hue: {}, Sat: {}, Val: {}", h, s, v),
    }

    // 匹配结构体
    match point {
        // 守卫 (Guard): 额外的 if 条件
        Point { x, y } if x == 0 => println!("Y is {}, X is on the axis", y),
        Point { x: 0, y } => println!("X is 0, Y is {}", y), // 指定匹配特定值
        Point { x, y } => println!("X: {}, Y: {}", x, y),
    }
}
