// 📖 讲解：options1
// 【题目要求】实现 maybe_icecream(time_of_day) 函数：22 点（10PM）之前冰箱里剩 5 支冰淇淋，22~23 点已被吃光剩 0 支，超过 23 点属于非法时间要优雅地返回 None；另外修复 raw_value 测试，让它能取出 Option 里包装的值。
// 【考察知识点】Option<T> 的构造（Some/None）、用 if/else 表达式按分支返回不同的 Option、用 unwrap 取出 Option 内部的值。
// 【对应教材】Rust Book §6.1-6.3（Option<T> 枚举）
// 【解法思路】用 if / else if / else 三段表达式分别返回 Some(5)、Some(0)、None（注意 24 也是非法值，所以第二个分支用 < 24）；测试里对 Option 调用 .unwrap() 解包成 u16 再与 5 比较。

// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day < 22 {
        Some(5) // 💡 22 点之前：还剩 5 支冰淇淋
    } else if time_of_day < 24 {
        Some(0) // 💡 22、23 点：已被吃光剩 0 支（合法时间只有 0~23，所以用 < 24）
    } else {
        None // 💡 大于 23 是非法时间：返回 None 而不是 panic，体现 Option 的优雅处理
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5); // 💡 unwrap() 取出 Some 里的值；若是 None 则会 panic
    }
}
