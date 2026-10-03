// 📖 讲解：functions2
// 【题目要求】main 调用 call_me(3) 传了一个参数，但函数定义没有形参；给函数定义补上参数。
// 【考察知识点】函数参数必须写成 `名字: 类型`——Rust 不做函数参数类型推断。
// 【对应教材】Rust Book §3.3（函数）
// 【解法思路】`fn call_me(num: i32)`，让形参类型与调用处传入的整数 3 匹配。

// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num: i32) { // 💡 补上形参及其类型标注 `num: i32`，与调用处 call_me(3) 匹配
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
