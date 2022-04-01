// 变量
fn variable() {
    // let 声明一个不可变的变量
    let a_number = 1;

    // 加mut关键字声明一个可变的变量
    let mut b_number = 2;

    // 变量隐藏, b_number
    let mut b_number = 2 + 1;
}

// 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
// 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变
// 常量可以在任意作用域内声明，包括全局作用域，在声明的作用域内，常量在程序运行的整个过程中都有效
fn const_demo() {
    // const
    const MAX_SIZE: u8 = 10;
}

const MIN_SIZE: u8 = 1;
