// 📖 讲解：primitive_types6
// 【题目要求】用元组索引（点号语法）访问 numbers 的第二个元素，使测试通过。
// 【考察知识点】元组索引语法 `tuple.下标`，下标从 0 开始（第一个是 .0，第二个是 .1）。
// 【对应教材】Rust Book §3.2（数据类型 - 元组）
// 【解法思路】`numbers.1` 直接取出第二个元素 2。

// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1; // 💡 元组用 `.下标` 访问（下标从 0 起）：.1 就是第二个元素 2

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
