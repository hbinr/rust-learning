# 探索数字、文本和 true/false 值的数据类型

Rust 是一种静态类型化的语言。 编译器必须知道代码中所有变量的确切数据类型，以便程序编译和运行。 编译器通常可以根据绑定值推断变量的数据类型。 并非总是需要在代码中显式说明类型。 如果有多种类型，则必须通过使用类型注释，让编译器得知特定类型。

在以下示例中，我们告知编译器将 number 变量创建为 32 位整数。 在变量名称后面指定数据类型 u32。 请注意变量名称后面冒号 : 的用法。

```rust
let number: u32 = 100;
println!("The number is {}.", number);
```
如果用双引号将变量值括起来，编译器会将值解释为文本而不是数字。 值的推断数据类型不匹配为变量指定的 u32 数据类型，因此编译器会发出错误：
```rust
let number: u32 = "14";
```
编译器错误：
```sh
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let number: u32 = "14";
  |                 ---   ^^^^ expected `u32`, found `&str`
  |                 |
  |                 expected due to this

error: aborting due to previous error
```

## 数字：整数和浮点值
我们可以通过位数和带符号的属性识别 Rust 中的整数。 带符号整数可以是正数或负数。 无符号整数只能是正数。

常用数字类型：
![](../../img/runt-数字类型.png)

`isize` 和 `usize` 类型取决于运行程序的计算机的类型。 应在 64 位体系结构上使用 64 位类型，在 32 位体系结构上使用 32 位类型。 如果未指定整数的类型，并且系统无法推断类型，则默认情况下，系统会分配 `i32` 类型（带符号的 32 位整数）。

Rust 有两种用于十进制值的浮点数据类型：`f32`（32 位）和 `f64`（64 位）。 默认浮点类型为 `f64`。 在新式 CPU 上，`f64` 类型的速度与 `f32` 类型大致相同，但精度更高。

Rust 中的所有基元数字类型都支持数学运算，如加法、减法、乘法和除法。

```rust
// Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
```

**备注：**
调用 `println` 函数时，将数据类型后缀添加到每个文本数字，以告知 Rust 有关数据类型的信息。 语法 `1u32` 会告知编译器值为数字 1 并将该值解释为无符号 32 位整数。   如果我们不提供类型注释，Rust 会尝试从上下文推断类型。 当上下文不明确时，它默认分配 `i32` 类型（带符号的 32 位整数）。

## 布尔值：true 或 false
Rust 中的布尔类型用于存储真实性。 `bool` 类型具有两个可能的值：`true` 或 `false`。 布尔值广泛用于条件表达式。 如果 `bool` 语句或值为 `true`，则执行此操作；否则（语句或值为 `false`），执行其他操作。 布尔值通常由比较检查返回。

在以下示例中，我们使用大于 (>) 运算符来测试两个值。 该运算符返回显示测试结果的布尔值。
```rust
let is_bigger = 1 > 4;
println!("Is 1 > 4? {}", is_bigger);  //  Is 1 > 4? -- false

```
## 文本：字符和字符串
Rust 支持具有**两种基本字符串类型**和**一种字符类型的文本值**。 

字符是单个项，而字符串是一系列字符。 所有文本类型都是有效的 UTF-8 表示形式。

`char` 类型是最基元的文本类型。 通过将项括在**单引号**中来指定值：
```rust
let uppercase_s = 'S';
let lowercase_f = 'f';
```
**备注:**

某些语言将其 `char` 类型视为 8 位无符号整数，这等效于 Rust 中的 `u8` 类型。 Rust 中的 `char` 类型包含 unicode 码位，但不会使用 utf-8 编码。 Rust 中的 `char` 是一个 21 位整数，系统会将其宽度填充为 32 位。 `char` 直接包含纯码位值。

## 字符串
`str` 类型也称为“字符串切片”，它是字符串数据的一种视图。 大多数情况下，我们使用在类型前面添加 & 符号 `str` 的引用样式语法来引用这些类型。 关于引用，我们将在后续模块中介绍。 现在，大家可以将 `&str` 视为指向不可变字符串数据的指针。 字符串字面量的类型都是 `&str`。

尽管在 Rust 入门示例中使用字符串字面量很方便，但它们并不适合可能要使用文本的每种情况。 在编译时并非每个字符串都是已知的， 例如，当用户在运行时与程序交互并通过终端发送文本时。

对于这些场景，Rust 具有另一个名为 `String` 的字符串类型。 此类型在堆上分配。 使用 `String` 类型时，无需在编译代码之前知晓字符串的长度（字符数）。

> 备注:
> 
> 如果你熟悉垃圾回收语言，可能会想知道 Rust 为何有两种字符串类型。1 字符串是一种十分复杂的数据类型。 大多数语言使用其垃圾回收器来掩盖这种复杂性。 
> 
> Rust 作为一种系统语言，公开了字符串的一些内在复杂性。 随着复杂性的增加，我们对程序中内存使用方式的控制变得非常精细。
> 实际上，Rust 具有两种以上字符串类型。 在此模块中，我们只介绍了 `String` 和 `&str` 类型。 你可以在 Rust 文档中详细了解提供的字符串类型。

在我们学习 Rust 的**所有权和租借系统**之前，我们不会全面了解 `String` 和 `&str` 之间的区别。 在此之前，可将 `String` 类型数据看作可随程序运行而更改的文本数据。 `&str` 引用是文本数据的不可变视图，不会随着程序运行而改变。


## 文本示例

以下示例展示如何在 Rust 中使用 `char` 和 `&str` 数据类型。
- 使用 `: char` 注释语法声明两个字符变量。 这些值使用单引号指定。
- 声明第三个字符变量并将其绑定到单个图像。 对于此变量，请通过编译器推断数据类型。
- 声明两个字符串变量并将其绑定到它们各自的值。 字符串用双引号括起来。
- 使用 `: &str` 注释语法声明其中一个字符串变量以指定数据类型。 另一个变量的数据类型未指定。 编译器将根据上下文推断此变量的数据类型。

```rust
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';
   
// Complier interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Complier interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
```