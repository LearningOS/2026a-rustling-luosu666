// 📖 讲解：clippy2.rs —— 别用 for 循环遍历 Option
// 【题目要求】代码想"如果 option 是 Some 就把里面的值加到 res 上"，用了 `for x in option`。clippy 会警告这种写法，请改成 clippy 建议的形式，做到零警告。
// 【考察知识点】clippy::for_loops_over_fallibles：Option（和 Result）虽然实现了 Iterator，但对它做 for 循环语义含混（最多只迭代一次），clippy 建议改写为 `if let Some(x) = ...`，更清晰地表达"可能有一个值"。
// 【对应教材】Rust Book 附录 D（Clippy 介绍）；if let 模式匹配见第 6 章 6.3 "Concise Control Flow with if let"。
// 【解法思路】把 for x in option { res += x; } 换成 if let Some(x) = option { res += x; }。程序输出仍为 54，clippy 不再报警。

// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option { // 💡 修复点：`for x in option` 触发 clippy::for_loops_over_fallibles（对 Option 做 for 循环），改成 if let 模式匹配更清晰
        res += x;
    }
    println!("{}", res);
}
