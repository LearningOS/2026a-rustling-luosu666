// 📖 讲解：iterators3 —— divide 与 collect 的两种收集方式
// 【题目要求】1) 实现 divide：整除返回 Ok(商)，除零/不能整除返回对应错误；
//            2) 补全 result_with_list 与 list_of_results 的返回类型，让 Debug
//            输出分别为 `Ok([1, 11, 1426, 3])` 与 `[Ok(1), Ok(11), Ok(1426), Ok(3)]`。
// 【考察知识点】Result 与自定义错误枚举、迭代器 map + collect 的"魔法"：
//            把 `Result<i32, E>` 的迭代器 collect 成 `Vec<Result<..>>`（逐个收集），
//            或 collect 成 `Result<Vec<..>, E>`（遇 Err 短路，全部成功才 Ok）。
// 【对应教材】Rust Book 第 13 章 13.2-13.4（迭代器）+ 9.2（Result 与 ? 的传播思想）
//            https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
// 【解法思路】divide 按 b == 0、a % b != 0、整除三种情况返回；两个函数体只差返回
//            类型：`Result<Vec<i32>, DivisionError>` 会触发 collect 的短路收集，
//            `Vec<Result<i32, DivisionError>>` 则逐个收集保留每个 Result。
//
// iterators3.rs
//
// This is a bigger exercise than most of the others! You can do it! Here is
// your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
//
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero) // 💡 先处理除零
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible(NotDivisibleError {
            // 💡 不能整除：按测试要求携带被除数与除数
            dividend: a,
            divisor: b,
        }))
    } else {
        Ok(a / b) // 💡 能整除：返回商
    }
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    // 💡 收集成 Result<Vec<i32>, _>：任一 Err 短路返回，全部成功则 Ok(整个列表)
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    // 💡 收集成 Vec<Result<..>>：保留每一个 Result，不短路
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
    division_results.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
