// 📖 讲解：functions1
// 【题目要求】main 里调用了 call_me()，但这个函数不存在；定义它使程序编译通过。
// 【考察知识点】fn 定义函数；无参数、无返回值函数的最简形式。
// 【对应教材】Rust Book §3.3（函数）
// 【解法思路】补上 `fn call_me() {}`——函数名用蛇形命名，函数体可以为空。

// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me();
}

fn call_me() { // 💡 补上被调用函数的定义：fn 名字(参数) { 函数体 }，这里无参数、函数体留空即可

}
