// 📖 讲解：iterators4 —— 函数式写法求阶乘
// 【题目要求】实现 factorial(num)：不许用 return，尽量不用命令式循环（for/while）
//            和额外变量，进阶要求也不用递归。
// 【考察知识点】范围迭代器 `1..=num`、`Iterator::product()`（以及 fold 的等价写法），
//            零负担抽象的函数式风格。
// 【对应教材】Rust Book 第 13 章 13.2-13.4（迭代器适配器与消费器）
//            https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
// 【解法思路】`(1..=num).product()`：把 1 到 num 连乘。num 为 0 时 `1..=0` 是空
//            范围，空迭代器 product 返回乘法单位元 1，恰好就是 0! = 1。
//            等价写法：`(1..=num).fold(1u64, |acc, x| acc * x)`。
//
// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).product() // 💡 1..=num 连乘；num == 0 时为空范围，product 返回单位元 1，即 0! = 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
