// 📖 讲解：clippy1.rs —— 不要手写数学常量（approx_constant）
// 【题目要求】本组练习要求 cargo clippy 完全零警告（rustlings 用 -D 把警告当错误）。本题主函数计算圆面积，用手写的 3.14 当 π，clippy 不答应，请按它的建议修好。
// 【考察知识点】clippy::approx_constant：检测手写的"近似常量"（3.14、3.14159、2.718 等），要求改用标准库常量 std::f32::consts::PI / f64::consts::PI 等；顺带体会"clippy 的建议输出"怎么读。
// 【对应教材】Rust Book 附录 D（Clippy 实用工具介绍）；常量 PI 见标准库文档 std::f32::consts。
// 【解法思路】把 let pi = 3.14f32 改成 let pi = f32::consts::PI（顶部已有 use std::f32;，正好用上它，避免 unused import 警告）。其余代码不动。这就是 clippy 报 "approximate value of `f32::consts::PI` found" 的标准修法。

// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

use std::f32;

fn main() {
    let pi = f32::consts::PI; // 💡 修复点：3.14f32 触发 clippy::approx_constant（近似常量），改用标准库自带的精确 PI；同时保住了上面的 use std::f32 不至于变成未使用导入
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
