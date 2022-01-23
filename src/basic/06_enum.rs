// 为复合数据使用枚举变量

// 定义枚举
enum WebEvent {
    // 单元结构体类型
    WELoad,
    // 元组结构体类型
    WEKeys(String, char),
    // 经典结构体类型
    WEClick { x: i64, y: i64 },
}

// 定义一个元组结构体
#[derive(Debug)] // 通过 #[derive(Debug)] 语法可以在代码执行期间查看某些在标准输出中无法查看的值
struct KeyPress(String, char);

// 定义一个经典结构体
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

// 定义一个新的枚举
// 枚举中的每个变体都使用已定义好的结构
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

// 实例化枚举
fn enum_learn() {
    // 简单变体：WELoad(bool)
    let we_load = WebEvent::WELoad(true);

    // 结构变体：WEClick(MouseClick)
    let click = MouseClick { x: 1, y: 2 };
    let we_click = WebEvent::WEClick(click);

    // 元组变体：WEKeys(KeyPress)
    let key_press = KeyPress(String::from("Ctrl + "), 'A');
    let we_keys = WebEvent::WEKeys(key_press);

    // 通过 {:#?} 格式化输出枚举变体
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
