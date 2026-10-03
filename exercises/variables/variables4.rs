// 📖 讲解：variables4
// 【题目要求】x 声明后被重新赋值为 5，但变量默认不可变；只能修改声明处让赋值合法（赋值行不许改）。
// 【考察知识点】可变性关键字 mut：不可变变量二次赋值会报 E0384。
// 【对应教材】Rust Book §3.1（变量与可变性）
// 【解法思路】声明改为 `let mut x = 3;`，加了 mut 之后才能对 x 重新赋值。

// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut x = 3; // 💡 加 mut 让变量可变，否则下面 x = 5 会报"不能对不可变变量赋值"
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
