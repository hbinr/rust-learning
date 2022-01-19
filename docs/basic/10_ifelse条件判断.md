# if/else 条件判断

和大部分语言的 `if/else` 一样，不同之处有
- 条件判断处可以不用加 `()`，这个和 `Go` 类似
- Rust 中的 `if` 块也可充当表达式。 条件分支中的所有执行块都必须为要编译的代码返回相同的类型。
```rust
let formal = true;
let greeting = if formal { // if used here as an expression
    "Good day to you."     // return a String
} else {
    "Hey!"                 // return a String
};
println!("{}", greeting)   // prints "Good day to you."
```

在此示例中，我们根据 `if formal` 表达式的结果为 `greeting` 变量赋值。 当表达式 `if formal` 为 `true` 时，`greeting` 值设置为字符串“Good day to you.”。 当表达式为 `false` 时，`greeting` 值设置为字符串“Hey!”。 由于我们将 formal 变量初始化为 `true`，所以我们知道表达式 `if formal` 的结果也是 `true`。