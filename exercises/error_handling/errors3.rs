// 📖 讲解：errors3
// 【题目要求】main 函数里用了 total_cost(pretend_user_input)?，但 main 的返回类型是 ()，导致 ? 无法使用而编译失败。修好它。
// 【考察知识点】? 运算符要求所在函数返回 Result（或其它实现了 FromResidual 的类型）；main 也可以返回 Result<(), E>。
// 【对应教材】Rust Book 第 9 章（§9.2 ? 运算符、main 返回 Result）
// 【解法思路】把 main 的签名改成 fn main() -> Result<(), ParseIntError>。这样 ? 就能正常传播错误；main 返回 Err 时进程以非零退出码结束并打印错误。

// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> { // 💡 main 也可以返回 Result：给 ? 一个可以传播错误的出口
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(()) // 💡 成功路径返回 Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
