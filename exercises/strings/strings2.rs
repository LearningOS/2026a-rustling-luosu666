// 📖 讲解：strings2
// 【题目要求】不改函数签名（is_a_color_word 接收 &str），也不改 word 的定义，让程序编译通过。
// 【考察知识点】String 与 &str 的自动转换（Deref 强制转换 / deref coercion）：
//             函数要 &str 时，传 &String 即可自动降级为 &str——因为 String 实现了 Deref<Target = str>。
//             直接传 String 会报类型不匹配，因为参数要的是“引用”。
// 【对应教材】Rust Book §8.2（字符串切片）；Deref 强制转换见 §15.2
// 【解法思路】调用处加一个 &：is_a_color_word(&word)。
//             &String → &str 是 Rust 中最常见的自动转换，随时都在发生（例如 &word 传给接收 &str 的函数）。

// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) { // 💡 加 & 传 &String，自动 deref 强制转换为函数需要的 &str
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
