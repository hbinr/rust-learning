# 在 Rust 中使用函数

函数是在 Rust 中执行代码的主要方法。 我们已有提及此语言中最重要的一种函数，即 `main` 函数。 在本单元中，我们将介绍有关如何定义函数的更多详细信息。

## 定义函数
Rust 中的函数定义以 `fn` 关键字开头。 在函数名称后面，将函数的输入参数指定为括在括号内以逗号分隔的数据类型列表。 编译器通过大括号判断函数体的起始和结束位置。
```rust
fn main() {
    println!("Hello, world!");
    `goodbye`();
}

fn goodbye() {
    println!("Goodbye.");
}
```
我们使用函数名称以及括号中的输入参数来调用函数。 如果函数没有任何输入参数，则将括号留空。 在示例中，`main` 和 `goodbye` 函数都没有输入参数。

你可能已经注意到，我们在定义 `main` 函数后，定义了 `goodbye` 函数。 我们本可以在定义 `main` 之前定义 `goodbye` 函数。 Rust 不在意文件中函数的定义位置，只要在文件中的某处定义了函数即可。

## 返回值
当函数返回某个值时，请在函数**参数列表后面**和**函数体的左大括号前面**添加语法 `-> <type>`。 箭头语法 `->` 表示函数向调用方返回某个值。 编译器通过 `<type>` 部分判断返回值的数据类型。

在 Rust 中，常见的做法是通过使函数中的最后一行代码等于要返回的值，从而在函数末尾返回一个值。
```sh
fn add(x: i64, y: i64) -> i64 {
    x + y
    // 等同于:
    // return x + y;
}
```
**注意：**

- 如果有返回值，函数中的最后一行代码，**不要加 `;`**，编译器会以为函数体的逻辑还未写完

```sh
error[E0308]: mismatched types
  --> src\main.rs:40:27
   |
40 | fn add(x: i64, y: i64) -> i64 {
   |    ---                    ^^^ expected `i64`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
41 |     x + y;
   |          - help: consider removing this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rust-learning` due to previous error
```

可以在函数中的任意位置使用 `return` 关键字来停止执行并将值发送回调用方。 通常，`return` 关键字与条件测试结合使用。

在下面的示例中，如果 `x` 的值为 0，则显式使用 `return` 关键字提前从函数返回：
```rust
fn add(x: i64, y: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    x + y
}
```

显式使用 `return` 关键字时，语句以分号结束。 **如果在不使用 `return` 关键字的情况下发送回返回值，请勿以分号结束语句**。 你可能已注意到，我们没有将结束分号用于 `x + y` 返回值语句。

## 查看签名

函数声明的第一部分称为函数签名。

示例中 `add` 函数的签名具有以下特征：
- ·：Rust 中的函数声明关键字。
- `add`：函数名称。
- `(x: i64, y: i64)`：函数的实参或形参列表。 一个指向字符串数据的指针应作为输入值。
- `-> i64`：箭头指向此函数将始终返回的值类型。

`add` 函数接受两字数字作为输入并输出一个数字。