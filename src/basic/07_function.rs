// 在 Rust 中使用函数

fn Hello() {
    println!("hello world")
}

// Rust是一个表达式语言，表达式可以作为返回值
fn add(x: i64, y: i64) -> i64 {
    // 表达式
    x + y
    // 等同于:
    // return x + y;
}

fn add2() {
    let z = {
        let x = 1;
        let y = 2;
        // 不加分号，返回3
        x + y

        // 加分号，无返回值,其实是返回空元组 ()
        // x + y;
    };

    println!("res: {}", z)
    // 如果使用 x + y; 编译会报错：
    //  println!("res: {}", z)
    //                      ^ `()` cannot be formatted with the default formatter
}

fn five() -> u8 {
    5
}
