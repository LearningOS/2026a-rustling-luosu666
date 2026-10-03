// 📖 讲解：tests2 —— assert_eq! 宏
// 【题目要求】修复空参数的 `assert_eq!()`，让测试编译并通过。
// 【考察知识点】`assert_eq!` 宏：比较两个值是否相等（相当于 `left == right`），
//            两边类型必须相同且实现 PartialEq/Debug。
// 【对应教材】Rust Book 第 11 章 11.1 编写测试（用 assert_eq! 宏检查相等）
//            https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
// 【解法思路】给 `assert_eq!` 传入两个相等（且类型相同）的值即可。
//
// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(1, 1); // 💡 两个参数相等（且类型一致），断言通过
    }
}
