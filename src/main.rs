use std::collections::HashMap;

// 结构体定义--元组结构
struct Teacher(String, u8);

fn main() {
    println!("Hello, world!");

    // const
    const MAX_SIZE: u8 = 1;
    println!("MAX_SIZE: {MAX_SIZE}");
    //  声明一个 元组
    let a_tuple = (1, 1.23, '2', true, "hello");

    // 获取元组中的元素
    let a_number = a_tuple.0;
    let a_bool = a_tuple.3;
    let a_str = a_tuple.4;
    println!("{a_number} {a_bool} {a_str} ");
    // 经典结构定义
    #[derive(Debug)]
    struct Student {
        name: String,
        age: u8,
    }

    // 实例化经典结构时，需要提供结构的名称和字段的值，以逗号分隔。不需要关注字段顺序
    let student_1 = Student {
        name: String::from("zhangsan"),
        age: 18,
    };

    println!(
        "student_1's name is {}, age is {}",
        student_1.name, student_1.age
    );

    let teacher = Teacher(String::from("tom"), 18);
    println!(
        "teacher element one: {}, element two: {}",
        teacher.0, teacher.1
    );
    #[derive(Debug)]
    enum People {
        Student(Student),
    }
    let people = People::Student(student_1);
    println!("People is {:#?}", people);

    let res = add(0, 2);
    println!("res is {}", res);

    // 数组
    let mut nums = [1, 2, 3, 4, 5];
    println!("nums element is {}", nums[2]);
    nums[0] = 10;
    println!("new nums is {:?}", nums); // new nums is [10, 2, 3, 4, 5]

    // 向量
    let zeros = vec![0; 3];
    println!("zeros is {:?}", zeros);

    let mut fruit = Vec::new();
    fruit.push("apple");
    fruit.push("orange");
    fruit.push("Cherry");
    println!("fruit: {:?}", fruit);
    println!("Pop off: {:?}", fruit.pop()); // Pop off: Some("Cherry")
    println!("fruit: {:?}", fruit);

    // 通过索引访问
    println!("first fruit is {}", fruit[0]); // first fruit is apple
    fruit[0] = "pear";
    println!("fruit: {:?}", fruit); // fruit: ["pear", "orange"]
    println!("hello");

    // practice
    let colors = ["Blue", "Green", "Red", "Silver"];
    let mut engine: Transmission = Transmission::Manual;
    let mut car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[1]), engine, true, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    //  if/else （if 块可以接表达式）
    let book_type = "history";
    let book_name = if book_type == "history" {
        "The Sun Also Rises"
    } else if book_type == "science" {
        "The Martian"
    } else {
        "The Martian"
    };

    println!("book_name: {}", book_name);

    // HashMap
    let mut book_prices_map: HashMap<String, u8> = HashMap::new();
    book_prices_map.insert(String::from("Rust Cook Book"), 1);
    book_prices_map.insert(String::from("Go Cook Book"), 2);
    println!("book prices map: {:?}", book_prices_map);

    let book_name_key: &str = "Rust Cook Book";
    println!(
        "《{book_name_key}》 book's price is {:?}",
        book_prices_map.get(book_name_key)
    );
    book_prices_map.remove(book_name_key);

    println!(
        "《{book_name_key}》 book's price is {:?}",
        book_prices_map.get(book_name_key)
    );

    // loop 2. 中断循环，并返回值
    let mut counter = 1;
    let stop_loop = loop {
        counter += 1;
        if counter > 10 {
            break;
        }
    };

    println!("counter is {:?}", stop_loop);

    // for 1. 基础写法
    let num_arrs = [1, 2, 3, 4, 5];
    // for num in num_arrs.iter() {
    // 或者不加 iter() rust 1.57后改动
    for num in num_arrs {
        println!("num : {num}");
    }
    // for 2. 简单写法
    for i in 0..5 {
        // 从0开始，不包括5
        println!("i : {i}");
    }

    // 13. 错误处理
    // panic!("error handler");// error: process didn't exit successfully

    let num_arr = vec![1, 2, 3];
    // 数组越界会引发 panic
    println!("{}", num_arr[2]);
}

fn add(x: i64, y: i64) -> i64 {
    if x == 0 {
        return 0;
    }
    x + y
}
#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let mut quality: (Age, u32) = (Age::New, 0);
    if miles > 0 {
        quality = (Age::Used, miles)
    }
    // Return the completed tuple to the caller
    return quality;
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}
