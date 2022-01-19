// 在 Rust 中使用函数

fn Hello() {
    println!("hello world")
}

fn add(x: i64, y: i64) -> i64 {
    x + y
    // 等同于:
    // return x + y;
}

fn add2(x: i64, y: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    x + y
}
