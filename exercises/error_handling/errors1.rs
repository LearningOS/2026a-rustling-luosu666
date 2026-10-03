// 📖 讲解：errors1
// 【题目要求】把 generate_nametag_text 的返回类型从 Option<String> 改成 Result<String, String>：空名字时返回带解释信息的 Err，非空时返回 Ok（生成的名牌文本）。
// 【考察知识点】Result<T, E> 枚举、Ok/Err 变体的构造；为什么 Result 比 Option 更适合表达"可解释的错误"。
// 【对应教材】Rust Book 第 9 章（§9.1 用 Result 处理可恢复的错误）
// 【解法思路】Option 只能表示"有/无"，无法说明失败原因；改成 Result 后错误分支可以携带字符串信息。注意 Err 里的文字必须和测试断言的字符串一字不差。

// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

pub fn generate_nametag_text(name: String) -> Result<String, String> { // 💡 返回类型改为 Result，错误类型也用 String
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into()) // 💡 失败时返回 Err，并携带与测试一致的解释信息
    } else {
        Ok(format!("Hi! My name is {}", name)) // 💡 成功时把结果包在 Ok 里
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
