// 📖 讲解：using_as
// 【题目要求】修复 average 函数：total 是 f64，而 values.len() 返回 usize，二者不能直接相除；
//             需要把长度转换成 f64，让函数编译通过并返回正确的平均值 7.125。
// 【考察知识点】`as` 显式类型转换（usize → f64）；Rust 中不同类型不会隐式转换的强类型规则。
// 【对应教材】std::convert 模块文档 / Rust Book 中标量类型与 `as` 转换的章节。
// 【解法思路】`values.len()` 返回 usize，整数除法会丢精度且类型不匹配无法编译；
//             用 `values.len() as f64` 显式转换为浮点数后再做浮点除法。
//             注意：`as` 是显式且可能截断的转换（如 f64 as i32），本方向 usize→f64 无损。

// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64 // 💡 usize 不能与 f64 直接相除，用 `as f64` 显式转换后做浮点除法
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
