// 📖 讲解：variables5
// 【题目要求】number 先是字符串 "T-H-R-E-E"，之后要当作数字做 +2 运算；不许改名，只能用 let 再声明一次同名变量。
// 【考察知识点】变量遮蔽（shadowing）：用 let 重新声明同名变量，甚至可以改变类型。
// 【对应教材】Rust Book §3.1（变量与可变性 - 遮蔽）
// 【解法思路】第二个 `let number = 3;` 遮蔽掉字符串版本，从这一行起 number 就是 i32，可以参与加法。

// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    // 💡 用 let 重新声明同名变量（遮蔽 shadowing），类型从 &str 变为 i32，之后即可做算术
    println!("Number plus two is : {}", number + 2);
}
