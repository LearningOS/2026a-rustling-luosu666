// 📖 讲解：errors4
// 【题目要求】实现 PositiveNonzeroInteger::new：参数为负数返回 Err(CreationError::Negative)，为 0 返回 Err(CreationError::Zero)，为正数才返回 Ok。
// 【考察知识点】用 match 对数值分类并构造对应的 Result；match 守卫（match guard，x if x < 0）的写法。
// 【对应教材】Rust Book 第 9 章（Result 与匹配）以及 §6.2（match 守卫）
// 【解法思路】对 value 做 match：带条件的分支用 `x if x < 0` / `x if x == 0` 分别返回两种 Err，最后的 `x =>` 兜底返回 Ok，此时 x 一定 > 0。

// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            x if x < 0 => Err(CreationError::Negative), // 💡 负数：返回 Negative 错误（match 守卫做条件判断）
            x if x == 0 => Err(CreationError::Zero),    // 💡 零：返回 Zero 错误
            x => Ok(PositiveNonzeroInteger(x as u64)),  // 💡 其余情况必为正数，可以安全地 as 转成 u64 包进 Ok
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
