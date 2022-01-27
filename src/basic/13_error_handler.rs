fn error_handler_panic() {
    println!("error handler");

    // 数组越界panic示例
    let num_arr = vec![1, 2, 3];
    println!("{}", num_arr[4]);
}

fn error_handler_option() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first); // Some("banana")

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third); // Some("coconut")

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent); // None
}
