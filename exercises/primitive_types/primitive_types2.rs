// 📖 讲解：primitive_types2
// 【题目要求】给 your_character 赋一个你喜欢的 char 字面量，练习字符类型。
// 【考察知识点】char 用单引号书写、恰好一个字符；char 是 4 字节 Unicode 标量值（可放字母、数字、符号、中文、emoji）。
// 【对应教材】Rust Book §3.2（数据类型 - 字符类型）
// 【解法思路】`let your_character = '@';`——注意单引号（双引号是 &str 字符串，类型不同）。

// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    let your_character = '@'; // 💡 char 用单引号，恰好一个 Unicode 字符（'中'、'7'、'😀' 也都可以）
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
