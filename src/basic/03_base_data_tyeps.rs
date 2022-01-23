// 基本数据类型
fn base_data_types() {
    // 数字
    // 声明数字类型, 无符号整数(64位)
    let a_num: u64 = 1;

    // 声明数字类型, 有符号整数(64位)
    let b_num: i32 = 1;
    // rust 默认推断为 i32 类型
    let c_num = 1;

    // 声明浮点数类型, 32位浮点数
    let a_float_num: f32 = 1.0;
    //  默认浮点类型为 `f64`。
    let b_float_num = 1.0;

    // bool
    let a_flag: bool = true;
    // Rust 默认推断为 bool 类型
    let b_flag = false;

    // 文本

    // 1. 字符
    let a_char: char = 'a';
    let b_char = 'b';

    // 2. 字符串 -- 比较特殊, 有多中表示方式
    // &str 可以理解为指向不可变字符串数据的指针
    let a_string: &str = "hello";
    let b_string = "world";

    // 字符串拼接
    println!("{a_string} {b_string}");
}
