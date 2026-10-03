// 📖 讲解：functions4
// 【题目要求】sale_price 和 is_even 两个函数的签名缺少返回类型标注，补全使程序编译通过。
// 【考察知识点】返回值类型标注 `-> T`；if/else 各分支的值（表达式）可作为函数返回值。
// 【对应教材】Rust Book §3.3（函数 - 有返回值的函数）
// 【解法思路】sale_price 返回 i32（价格减折扣的结果），is_even 返回 bool（偶数判断的结果）。

// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 { // 💡 返回类型 i32：两个分支都是"整数 - 整数"的结果
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool { // 💡 返回类型 bool：`num % 2 == 0` 是比较表达式，结果为布尔值
    num % 2 == 0
}
