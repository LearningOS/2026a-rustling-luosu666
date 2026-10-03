// 📖 讲解：macros2.rs —— 宏的定义必须出现在调用之前
// 【题目要求】main 在前、宏定义在后，编译报错 "cannot find macro `my_macro` in this scope"。要求调整代码顺序使程序编译通过。
// 【考察知识点】macro_rules! 宏遵循"文本作用域（textual scoping）"：宏只在定义处之后的代码里可见（同文件内），这一点和函数不同 —— 函数只要在同一作用域，先调用后定义也没问题。
// 【对应教材】Rust Book 第 19 章 19.5 "Macros"（提到必须在作用域中先定义后使用，Rust 会为宏展开扫描整个源文件）。
// 【解法思路】把 macro_rules! my_macro 定义整块移到 main 函数上方，保证"先定义、后调用"。不需要 #[macro_export]（那是跨模块才需要的）。

// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    // 💡 宏定义整块搬到 main 之前：宏按文本顺序生效，必须先定义后使用
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
