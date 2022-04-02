// 引用和借用
// & 符号即是引用，它们允许你使用值，但是不获取所有权，
fn references_demo() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // 不允许比较整数与引用，因为它们是不同的类型。必须使用解引用运算符解出引用所指向的值。
    // assert_eq!(5, y);
    // ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
}
