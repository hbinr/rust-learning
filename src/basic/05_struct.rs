// 结构

// 经典结构定义
struct User {
    name: String,
    age: u8,
}

// 元组结构 字段只有类型，没有名称。 要访问元组结构中的字段，请使用索引元组时所用的语法
struct Student(String, u8);

// 单元结构 仅仅是在关键字 struct 后加结构名称，最常用作标记
struct Unit;

// 实例化结构

// 实例化经典结构时，需要提供结构的名称和字段的值，以逗号分隔。不需要关注字段顺序
let user_1 = User {
    name: String::from("zhangsan"),
    age: 18,
};

// 实例化元组结构时，字段值需要和定义时的顺序保持绝对一致
let student_1 = Student(String::from("lisi"), 18);

println!("user_1's name is {}, age is {}", user_1.name, user_1.age);

println!("student_1's name is {}, age is {}", student_1.name, student_1 .age);
