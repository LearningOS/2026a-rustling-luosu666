// 📖 讲解：tests3 —— 让测试真正调用被测函数
// 【题目要求】把空的 `assert!()` 改成真正调用 `is_even` 的断言：一个测试验证偶数
//            返回 true，另一个测试验证 `is_even(5)` 返回 false。
// 【考察知识点】在测试中调用函数并断言其返回值（`assert!(...)` / `assert!(!...)`）、
//            `use super::*` 引入父模块内容、偶数判断 `num % 2 == 0`。
// 【对应教材】Rust Book 第 11 章 11.1 编写测试
//            https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
// 【解法思路】is_even 已实现（`num % 2 == 0`），只需在测试里调用它：
//            偶数断言 `assert!(is_even(2))`；题目明确要求测 `is_even(5)`，
//            5 是奇数，所以断言取反 `assert!(!is_even(5))`。
//
// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2)); // 💡 真正调用 is_even：2 是偶数，返回 true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5)); // 💡 按题目要求测试 is_even(5)：5 是奇数返回 false，需取反 !
    }
}
