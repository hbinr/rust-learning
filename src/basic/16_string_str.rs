// 字符串和字符串切片

// String 与 &str 的转换

// &str -> String
fn str_to_string() {
    // 1. String::from
    let hello = String::from("hello");

    // 2. to_string()
    let world = "world".to_string();
}

// String -> &str
fn string_to_str() {
    let hello = String::from("hello");

    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}", s);
}

// 字符串操作
// push
fn push_demo() {
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);
}

// insert
fn insert_demo() {
    let mut s = String::from("Hello ");

    s.insert(1, 's');
    println!("{s}");

    s.insert_str(10, "ssss");
    println!("{s}");

    let mut hl = String::from("小明");
    hl.insert(3, 's');
    println!("hl: {hl}");

    hl.insert_str(3, "学");
    println!("hl: {hl}");
}

// delete
fn delete_demo() {
    // 1. pop 删除并返回字符串的最后一个字符

    // 2. remove
    // 3.
}
