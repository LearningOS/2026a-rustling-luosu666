// 📖 讲解：primitive_types1
// 【题目要求】照着上面的例子补全 is_evening 的声明，练习布尔类型。
// 【考察知识点】bool 类型：只有 true / false 两个值，可直接作 if 条件。
// 【对应教材】Rust Book §3.2（数据类型 - 布尔类型）
// 【解法思路】`let is_evening = false;`（写 true 也正确，只是会多打印一句 Good evening!）。

// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // Booleans (`bool`)

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // Finish the rest of this line like the example! Or make it be false!
    let is_evening = false; // 💡 布尔字面量只能是 true 或 false，这里选 false（选 true 也能通过）
    if is_evening {
        println!("Good evening!");
    }
}
