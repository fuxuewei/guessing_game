[中文文档](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html)

Rust 无需 GC（Garbage collection 垃圾回收）即可保障内存安全

## 变量

    ```rust
    let x = 5; // 常量
    let mut y = 4; //变量
    ```

## 数据类型 data type

### 标量类型

> 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。

- 整型

        | 长度    | 有符号 | 无符号 |
        | ------- | ------ | ------ |
        | 8-bit   | i8     | u8     |
        | 16-bit  | i16    | u16    |
        | 32-bit  | i32    | u32    |
        | 64-bit  | i64    | u64    |
        | 128-bit | i128   | u128   |
        | arch    | isize  | usize  |

- 浮点型
  `let y: f32 = 3.0; `
- 布尔型
  `let f: bool = false; `
- 字符类型
  ```rust
  fn main() {
      let c = 'z';
      let z = 'ℤ';
      let heart_eyed_cat = '😻';
  }
  ```

### 复合类型

> 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）

- 元祖类型
  `let tup: (i32, f64, u8) = (500, 6.4, 1);`

  一旦声明，长度固定

- 数组类型
  `let a = [1, 2, 3, 4, 5];`
  数组元素访问`a[0]`

## 函数

> Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词

Rust 中的函数定义以 fn 开始并在函数名后跟一对圆括号。大括号告诉编译器哪里是函数体的开始和结尾。

### 带参数

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

// 必须声明每个参数的类型
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

```

### 包含语句

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

_注意结尾没有分号的那一行 x+1，与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。在接下来探索具有返回值的函数和表达式时要谨记这一点。_

### 具有返回值的函数

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 正确 表达式 ，会得到它的返回值
    x + 1; // 错误，带分号的是 语句 ，语句不会反回值
}

```

## 控制流

- if else
  _if 和 else 分支的类型必须相同，if 的每个分支的可能的返回值都也必须是相同类型_

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

- loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

```

- while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

```

- for in

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## 所有权（ownership）

> 它让 Rust 无需垃圾回收（GC）即可保障内存安全，通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。在运行时，所有权系统的任何功能都不会减慢程序。

### 所有权规则

- Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域（scope），这个值将被丢弃。

```rust

fn main() {
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s 不再有效
}

```

这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 `drop`，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

**`Copy trait`**

### 引用

**引用的规则：**

- 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。

```rust

fn main() {
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
}

```

> 可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用。

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2); // 报错 不能在同一时间多次将 s 作为可变变量借用


fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}
```

这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：

- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

### slice
