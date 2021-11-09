use std::io; // io（输入/输出）库
use std::cmp::Ordering; // Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。这是比较两个值时可能出现的三种结果
use rand::Rng; // Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中

fn main(){
    println!("guess the number!"); // 打印字符串的宏
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("the secret number is: {}", secret_number);

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

}