// 📖 讲解：variables2
// 【题目要求】变量 x 只声明未初始化就被拿去比较，导致编译错误；给 x 一个初始值。
// 【考察知识点】变量必须初始化后才能读取；if 条件表达式。
// 【对应教材】Rust Book §3.1（变量与可变性）
// 【解法思路】`let x = 10;`（赋 10 走 if 分支；赋其他合法值也能编译，只是走 else 分支）。

// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x = 10; // 💡 声明的同时必须初始化，未初始化的变量不能用于比较
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
