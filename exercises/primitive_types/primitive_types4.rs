// 📖 讲解：primitive_types4
// 【题目要求】从数组 a = [1, 2, 3, 4, 5] 中切出一个切片，使其恰好等于 [2, 3, 4]，让测试通过。
// 【考察知识点】切片语法 &数组[起..止]；Range 区间左闭右开（含起始下标、不含结束下标）。
// 【对应教材】Rust Book §3.2（数据类型 - 数组）与 §4.3（切片 slice）
// 【解法思路】`&a[1..4]` 取下标 1、2、3 三个元素（不含下标 4），得到 [2, 3, 4]。

// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // 💡 左闭右开区间：从下标 1 切到下标 4 之前 → [2, 3, 4]

    assert_eq!([2, 3, 4], nice_slice)
}
