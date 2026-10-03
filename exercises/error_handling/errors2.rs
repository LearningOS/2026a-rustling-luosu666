// 📖 讲解：errors2
// 【题目要求】total_cost 接收用户输入的字符串并 parse 成数字。当 parse 失败时要把 ParseIntError 立即作为错误返回，而不是继续运算。
// 【考察知识点】? 运算符的错误传播（最简写法），或手动 match Result；parse::<i32>() 的 turbofish 写法。
// 【对应教材】Rust Book 第 9 章（§9.2 ? 运算符传播错误）
// 【解法思路】parse 返回 Result<i32, ParseIntError>，直接在表达式末尾加 ?：成功时得到 i32 继续计算，失败时函数提前返回 Err。这比手写 match 短得多。

// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; // 💡 加一个 ? ：出错时立即把 ParseIntError 返回给调用者，成功时 qty 就是 i32

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
