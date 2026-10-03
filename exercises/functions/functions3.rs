// 📖 讲解：functions3
// 【题目要求】call_me 的定义需要一个参数，但 main 里的调用没传实参；修改调用处使其编译通过。
// 【考察知识点】带参函数的调用：实参个数与类型必须与签名一致。
// 【对应教材】Rust Book §3.3（函数）
// 【解法思路】调用改为 `call_me(3);`，传入一个 u32 字面量，函数会响铃打印 3 次。

// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3); // 💡 按函数签名传入一个参数（原来是无参调用，与签名不符）
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
