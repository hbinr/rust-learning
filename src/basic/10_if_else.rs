//  if 使用
fn compare(a: i64, b: i64) -> i64 {
    if a > b {
        return a;
    }
    b
}

fn if_else_learn() {
    //  if 块可以接表达式
    let book_type = "history";
    let book_name = if book_type == "history" {
        "The Sun Also Rises"
    } else if book_type == "science" {
        "The Martian"
    } else {
        "The Martian"
    };

    println!("book_name: P{}", book_name);
}
