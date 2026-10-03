// 📖 讲解：if1
// 【题目要求】实现 bigger(a, b) 返回较大的数；限制：不许调用其他函数、不许引入额外变量。
// 【考察知识点】if/else 是表达式，其值可直接作为函数返回值。
// 【对应教材】Rust Book §3.5（控制流 - if 表达式）
// 【解法思路】`if a > b { a } else { b }`——分支里不带分号的 a / b 就是整个 if 表达式的值。

// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.


pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables

    if a > b { // 💡 if 是表达式：整个 if/else 的值就是所选分支的值，直接作为返回值
        a
    } else {
        b
    }
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
