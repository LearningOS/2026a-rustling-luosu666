// 📖 讲解：variables1
// 【题目要求】原代码在使用变量 x 之前没有声明它，请补上声明使程序编译通过。
// 【考察知识点】用 let 声明变量；Rust 变量必须先声明后使用。
// 【对应教材】Rust Book §3.1（变量与可变性）
// 【解法思路】在 println! 之前写 `let x = 5;` 声明并初始化 x 即可。

// variables1.rs
//
// Make me compile!
//
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x = 5; // 💡 用 let 声明变量；Rust 变量默认不可变（immutable）
    println!("x has the value {}", x);
}
