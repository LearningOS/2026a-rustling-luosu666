// 📖 讲解：functions5
// 【题目要求】square 函数应返回 num 的平方；补全函数体。
// 【考察知识点】表达式作为返回值：函数体最后一行不带分号即为返回值（也可写 return num * num;）。
// 【对应教材】Rust Book §3.3（函数 - 有返回值的函数）
// 【解法思路】函数体只写 `num * num`（无分号），它就是返回的表达式；加分号会变成语句、返回 () 而报错。

// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num // 💡 末行表达式不加分号，直接作为返回值
}
