// 📖 讲解：macros1.rs —— 宏必须用感叹号调用
// 【题目要求】定义好的 my_macro 宏在 main 里调用，让程序编译通过并打印 "Check out my macro!"。
// 【考察知识点】声明宏的调用语法：macro_rules! 定义、用 `宏名!(...)` 调用。函数和宏是两套东西，宏调用少了 `!` 编译器会去找同名函数，报 "cannot find function `my_macro`"。
// 【对应教材】Rust Book 第 19 章 19.5 "Macros"（macro_rules! 与宏的声明、调用方式）。
// 【解法思路】把 main 里的 my_macro(); 改成 my_macro!(); 即可 —— 就差一个感叹号，这是学宏时最常见的笔误。

// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); // 💡 宏调用必须带 `!`（以及括号），写成 my_macro(); 会被当成普通函数而找不到定义
}
