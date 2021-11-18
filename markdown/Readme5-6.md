## 5.结构体

用 `struct` 定义结构体

```rust
// 定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建结构体的 实例
let user1 = User {
    email: String::from("jereau@126.com"),
    username: String::from("vv"),
    active: true,
    sign_in_count: 1,
}

// 对结构体字段赋值
user1.email = String::from("476541140@qq.com");

// 创建返回实例的方法
fn build_user(email: String,username: String) -> User {
    User {
        email: email, // 参数名与字段名相同，可简写成email
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// ..语法  类似js的扩展运算符...
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

### 元组结构体（tuple structs）

**没有具体的字段名，只有字段的类型**

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### 类单元结构体（unit-like structs）

**类似 `unit` 类型 常用在想要在某个类型上实现 `trait` 但不需要在类型中存储数据时**

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

## 结构体的使用

**打印结构体**

- #[derive(Debug)]
- dbg!

> 我们希望借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 `&`

```rust
// 增加属性来派生 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn main(){
    // 结构体写法
    let rect1 = Rectangle { width: 30, height: 50};
    println!(
        "area: {}",
        area(&rect1),
    );
    println!("rect1 is {:?}", rect1); // 调试格式打印 Rectangle 实例

    // 使用 dbg! 宏：打印到标准错误输出控制台流
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}
```

### impl（implementation）块

感觉有点类似 js 的 class

**关联函数（associated functions）** 所有在 impl 块中定义的函数被称为 关联函数

**每个结构体都允许拥有多个 impl 块**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 这个 impl 块中的所有内容都将与 Rectangle 类型相关联
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

```

## 6.枚举 & 条件

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 创建不同的成员实例
let home = IpAddr::V4(String::from(127, 0, 0, 1));
let loopback = IpAddr::V6(String::from("::1"));

```

### Options 枚举

`Some` `None` 泛型`T`

> Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域
>
> 另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None
>
> 即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

### match 控制流运算符

匹配模式 绑定匹配的模式的部分值

```rust
fn main() {
    enum UsState {
    Alabama,
    Alaska,
    }

    enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

```

`_`通配符：会匹配所有的值

### if let

```rust
fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

```
