// 📖 讲解：intro2
// 【题目要求】补全 println! 宏，让程序打印出对世界的问候 "Hello world!"。
// 【考察知识点】println! 宏的基本用法、格式化占位符 {} 必须有对应参数。
// 【对应教材】Rust Book §1.2（Hello, World!）
// 【解法思路】原题占位符 {} 缺少实参导致编译错误；最简做法是直接打印完整字符串，也可写成 println!("Hello {}!", "world")。

// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    println!("Hello world!"); // 💡 直接输出整句字符串；等价写法：println!("Hello {}!", "world")
}
