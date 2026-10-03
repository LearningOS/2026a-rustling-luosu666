// 📖 讲解：tests1 —— 第一个测试：assert! 宏
// 【题目要求】修复空参数的 `assert!()`，让测试文件能编译并通过。
// 【考察知识点】`#[cfg(test)]` / `#[test]` 属性、`assert!` 宏的基本用法（参数必须是一个 bool 表达式）。
// 【对应教材】Rust Book 第 11 章 11.1 编写测试
//            https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
// 【解法思路】`assert!` 至少要传入一个求值为 bool 的表达式，传 `true`（或任何为真的
//            表达式，如 `1 == 1`）即可让测试通过。
//
// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(true); // 💡 assert! 需要一个 bool 参数；传 true 让断言通过（也可写 1 == 1 等）
    }
}
