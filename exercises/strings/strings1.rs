// 📖 讲解：strings1
// 【题目要求】不改函数签名（返回 String），让程序编译通过。
// 【考察知识点】&str 与 String 是不同类型：函数声明返回 String，而 "blue" 是 &'static str 字面量，
//             类型不匹配（expected `String`, found `&str`）。
// 【对应教材】Rust Book §8.2（使用字符串存储 UTF-8 编码的文本）
// 【解法思路】把字面量转换成堆分配的 String 即可，任选其一：
//             "blue".to_string()（本答案采用）/ String::from("blue") / "blue".to_owned()。
//             反过来把函数签名改成 &str 是题目禁止的。

// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string() // 💡 字面量是 &str，用 to_string() 转成 String 才能满足返回类型
}
