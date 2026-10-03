// 📖 讲解：modules3
// 【题目要求】用一条 use 语句把 std::time 模块中的 SystemTime 和 UNIX_EPOCH 引入作用域。
// 【考察知识点】从标准库引入路径；use + 花括号一次导入多个项目；
//             绝对路径 std::time 的写法。
// 【对应教材】Rust Book 第 7 章（§7.3 使用 use 关键字将路径引入作用域 / §7.4 分离模块到不同文件）
// 【解法思路】一行搞定（也符合“加分”要求）：
//             use std::time::{SystemTime, UNIX_EPOCH};
//             若写成两行 `use std::time::SystemTime; use std::time::UNIX_EPOCH;` 也可以编译通过。

// modules3.rs
//
// You can use the 'use' keyword to bring module paths from modules from
// anywhere and especially from the Rust standard library into your scope. Bring
// SystemTime and UNIX_EPOCH from the std::time module. Bonus style points if
// you can do it with one line!
//
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a
// hint.

// TODO: Complete this use statement
use std::time::{SystemTime, UNIX_EPOCH}; // 💡 一行导入两个项目：花括号列出多个名字（UNIX_EPOCH 是常量，同样是 std::time 下的项目）

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
