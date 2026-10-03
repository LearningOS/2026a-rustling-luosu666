// 📖 讲解：if2
// 【题目要求】补全 foo_if_fizz："fizz" → "foo"，"fuzz" → "bar"，其余任意输入 → "baz"，让 3 个测试通过。
// 【考察知识点】else if 多分支链；各分支必须返回同一类型（这里都是 &str）；&str 字符串比较。
// 【对应教材】Rust Book §3.5（控制流 - if 表达式）
// 【解法思路】中间补 `else if fizzish == "fuzz" { "bar" }`，最后的 else 兜底返回 "baz"。

// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.


pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" { // 💡 补上 fuzz 分支，返回 "bar"
        "bar"
    } else { // 💡 其余所有输入（如 "literally anything"）都落到这里，返回 "baz"
        "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
