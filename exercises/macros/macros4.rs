// 📖 讲解：macros4.rs —— macro_rules! 多条规则之间要加分号
// 【题目要求】my_macro 定义了两个匹配分支（arm）：无参版本和带 $val:expr 的版本，main 里分别以 my_macro!() 和 my_macro!(7777) 调用。原代码编译不过，要求修好宏定义。
// 【考察知识点】macro_rules! 的多条规则之间必须用分号 `;` 分隔（类似 match 分支用逗号）；片段说明符 $val:expr 表示匹配任意表达式；一个宏可以按参数形态匹配到不同的展开分支（从上到下依次尝试）。
// 【对应教材】Rust Book 第 19 章 19.5 "Macros"（"Argument patterns / 多分支宏"与附录 "Macro Rules" 相关内容，`$x:expr` 片段说明符在书中均有出现）。
// 【解法思路】在第一条规则 `() => { ... }` 的右花括号后面补一个分号。原文只在两个规则之间少了分号，第二个分支末尾（} 后面接的是宏结束的 }）可以不加，但补上更规范。其余代码不变：my_macro!() 命中第一分支，my_macro!(7777) 命中第二分支。

// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 💡 修复点：两条规则之间必须用分号分隔（原代码这里漏了 `;`，导致第二条规则无法解析）
    ($val:expr) => {
        println!("Look at this other macro: {}", $val); // 💡 $val:expr 匹配任意表达式，展开时原样代入
    }; // 💡 最后一条规则后面的分号可写可不写，写上更统一
}

fn main() {
    my_macro!(); // 💡 无参调用 → 命中第一条规则
    my_macro!(7777); // 💡 带表达式参数 → 命中第二条规则，打印 "Look at this other macro: 7777"
}
