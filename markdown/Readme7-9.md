## 包（package） Crate 模块管理

- 包（Packages）： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use： 允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式

### package & Crate

> crate root 是一个源文件，是 Rust 编译器的起点，并构成你的 crate 的根模块
>
> package 是提供一系列功能的一个或者多个 crate。一个 package 会包含一个 Cargo.toml 文件，阐述如何构建这些 crate

### 模块 & 模块系统

以 `mod` 关键字来定义模块

```rust

fn main() {
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn server_order() {}

            fn take_payment() {}
        }
    }
}

// 模块树的结构
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

```

### 路径

- **绝对路径（absolute path）** 从 crate 根开始，以 crate 名火字面值 crate 开头
- **相对路径（relative path）** 从当前模块开始，以 self super 或当前模块的标识符开头

> 绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。
