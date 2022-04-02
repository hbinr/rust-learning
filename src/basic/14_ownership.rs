// copy trait: 针对存与 stack 上的数据。任何简单的标量的组合类型(基本数据类型)都实现了 copy trait;
// 这些基本数据类型都保存于栈上，如:整数、浮点数、bool类型、字符、元组(全部基本数据类型组成)、不可变引用 &T，但是注意: 可变引用 &mut T 是不可以 Copy的

fn copy_demo() {
    // 实现了 copy trait
    let tuple_copy = (1, 2);
}

// clone trait: 针对存于 heap 上的数据。任何需要分配内存或者某种资源的类型都实现了 clone trait;
// 复杂数据类型都保存于堆上

fn clone_trait() {
    // 实现了 clone trait
    let name = String::from("tom");
}

// 所有权与函数
// 1. 将值传递给函数会发生移动或复制，类似把值赋值给变量
// 2. 函数在返回值的时候同样也会发生所有权的转移

// 1. 入参所有权demo
fn arg_demo() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x}
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// 2. 返回值所有权demo
fn callback_demo() {
    // some_str 的所有权移到了 s1
    let s1 = gives_ownership();

    let s2 = String::from("Hello, World");
    // s2 作为入参，所有权发送了移动，移到了 takes_and_gives_back 函数里，调用后又被移动给了s3
    let s3 = takes_and_gives_back(s2);
    // 执行 println!(" s2: {s2}"); 会报错：
    // takes_and_gives_back(s2); 这儿所有权进行了移动-- value moved here
    // println!(" s2: {s2}");
    //                ^^^^ value borrowed here after move

    // 所有权正确使用
    println!("s1: {s1}, s3: {s3}");
}

// 返回值的所有权移动
fn gives_ownership() -> String {
    // some_str 作为出参，所有权发送了移动，移到了调用该函数的地方
    let some_str = String::from("Hello");
    some_str
}

// 入参的所有权移动
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// 一个变量的所有权总是遵循同样的模式：
// 1. 把一个值赋值给其他变量的时候就会发生移动
// 2. 当一个包含heap数据的变量离开作用域时，它的值就会被 drop 函数清理，除非数据的所有权移动到另一个变量上了
