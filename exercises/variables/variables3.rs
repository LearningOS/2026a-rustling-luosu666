// 📖 讲解：variables3
// 【题目要求】变量 x 只有类型标注没有赋值，补上初始值使程序编译通过。
// 【考察知识点】`let x: 类型 = 值;` 的完整声明语法（类型标注 + 初始化）。
// 【对应教材】Rust Book §3.1（变量与可变性）
// 【解法思路】写成 `let x: i32 = 10;`，类型标注和初始值缺一不可。

// variables3.rs
//
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let x: i32 = 10; // 💡 在原题的类型标注后补上 `= 10` 完成初始化
    println!("Number {}", x);
}
