// 矢量数据类型
fn vectors() {
    // 定义矢量
    let thress_nums = vec![1, 2, 3];

    let zeros = vec![0; 3]; //  [0, 0, 0]

    // {:?} 格式化等其他向量
    println!("zeros is {:?}", zeros);

    // 通过 Vec::new() 创建向量
    let mut fruit = Vec::new();
    fruit.push("apple");
    fruit.push("orange");
    fruit.push("Cherry");

    println!("fruit: {:?}", fruit);
    println!("Pop off: {:?}", fruit.pop());
    println!("fruit: {:?}", fruit);

    // 通过索引访问
    println!("first fruit is {}", fruit[0]);

    // 向量值是可变的，因此可通过使用索引访问元素值来就地更改值
    fruit[0] = "pear";
    println!("fruit: {:?}", fruit); // fruit: ["pear", "orange"]
}
