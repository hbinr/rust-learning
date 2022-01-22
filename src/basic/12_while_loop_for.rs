// loop 使用
loop {
    println!("We loop forever!");
}

// loop 1. 中断循环
loop {
    // Keep printing, printing, printing...
    println!("We loop forever!");
    // On the other hand, maybe we should stop!
    break;                            
}

// loop 2. 中断循环，并返回值
let mut counter = 1;
let stop_loop = loop {
    counter += 1;
    if counter > 10 {
        break counter;
    }
}

println!("counter is {stop_loop}")

// loop 3. 中断循环，无返回值
let mut counter = 1;
let stop_loop = loop {
    counter += 1;
    if counter > 10 {
        break;
    }
};

println!("counter is {:?}", stop_loop)