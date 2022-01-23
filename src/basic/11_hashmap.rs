use std::collections::HashMap;

fn hashmap() {
    let mut book_prices_map: HashMap<String, u8> = HashMap::new();

    book_prices_map.insert(String::from("Rust Cook Book"), 1);
    book_prices_map.insert(String::from("Go Cook Book"), 2);

    // book_prices_map:  {"Go Cook Book": 2, "Rust Cook Book": 1}

    let book_name_key: &str = "Rust Cook Book";
    let res = book_prices_map.get(book_name_key);
    println!("《{book_name_key}》 book's price is {:?}", res)

    // 《Rust Cook Book》 book's price is Some(1)
    // 注意：返回结果中使用 Some() 包裹
}
