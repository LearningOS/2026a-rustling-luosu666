// 📖 讲解：variables6
// 【题目要求】用 const 声明常量 NUMBER，使程序编译通过。
// 【考察知识点】常量语法：const 必须显式标注类型，命名采用全大写蛇形（SCREAMING_SNAKE_CASE），不能用 mut。
// 【对应教材】Rust Book §3.1（变量与可变性 - 常量）
// 【解法思路】`const NUMBER: i32 = 3;`——与 let 不同，常量的类型标注不能省略。

// variables6.rs
//
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a
// hint.


const NUMBER: i32 = 3; // 💡 常量必须写明类型 i32，不能省略类型标注
fn main() {
    println!("Number {}", NUMBER);
}
