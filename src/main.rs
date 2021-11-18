use std::io; // io（输入/输出）库
use std::cmp::Ordering; // Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果
use rand::Rng; // Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中

// 结构体
// 增加属性来派生 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn main(){
    println!("guess the number!"); // 打印字符串的宏
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("the secret number is: {}", secret_number);

    loop {
        println!("please input your guess");

        let mut guess = String::new(); // 储存用户输入的地方 mut表示可变

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // 将 expect 调用换成 match 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值

        println!("you guessed: {}", guess); // {} 占位符

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

    // 常规写法
    let width = 30;
    let height = 50;
    // 元祖写法 
    let rect2 = (30, 50);
    // 结构体写法
    let rect3 = Rectangle { width: 30, height: 50};
    println!(
        "area1: {}, area1: {}, area3: {}",
        area1(width, height),
        area2(rect2),
        area3(&rect3), // 我们希望借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect3 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &
    );
    println!("rect3 is {:?}", rect3); // 调试格式打印 Rectangle 实例

    // 使用 dbg! 宏：打印到标准错误输出控制台流
    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect4);
}

// 非元祖写法
fn area1(width: u32, height: u32) -> u32{
    width * height
}
// 元祖写法
fn area2(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// 结构体写法
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}