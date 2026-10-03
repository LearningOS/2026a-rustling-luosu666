// 📖 讲解：primitive_types3
// 【题目要求】创建一个至少包含 100 个元素的数组。
// 【考察知识点】数组的重复初始化语法 `[元素; 个数]`；数组长度固定、编译期确定。
// 【对应教材】Rust Book §3.2（数据类型 - 复合类型之数组）
// 【解法思路】`let a = [0; 1000];` 用重复语法快速生成 1000 个 0（≥100 即满足要求，写 [0; 100] 也可以）。

// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = [0; 1000]; // 💡 数组重复初始化语法 [值; 数量]：生成 1000 个 0，满足 len() >= 100

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
