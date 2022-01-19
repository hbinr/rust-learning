# 在 Rust 中创建和使用变量


## 变量
-`let`

如果调用 println! 宏并尝试在绑定 a_number 变量之前显示该变量的值，则编译器会返回错误

## 不可变与可变
- `let`
- `mut`
在 Rust 中，变量绑定默认不可变。 如果变量不可变，在将值绑定到名称后，将无法更改此值。

例如，如果我们尝试更改前面示例中 a_number 变量的值，则会收到来自编译器的错误消息。
```rust
// Change the value of an immutable variable
a_number = 15;
```

若要更改值，必须先使用 mut 关键字将变量绑定设为可变。
```rust
// The `mut` keyword lets the variable be changed
let mut a_number = 10; 
println!("The number is {}.", a_number);

// Change the value of an immutable variable
a_number = 15;
println!("Now the number is {}.", a_number);
```

参考：
- https://docs.microsoft.com/zh-cn/learn/modules/rust-create-program/2-variables