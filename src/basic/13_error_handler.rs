fn error_handler() {
    println!("error handler");

    // 数组越界panic示例
    let num_arr = vec![1, 2, 3];
    println!("{}", num_arr[4]);
}
