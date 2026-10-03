// 📖 讲解：vecs1
// 【题目要求】创建一个 Vec，元素与数组 a 完全相同，让测试通过。
// 【考察知识点】用 vec! 宏创建动态数组；数组 [T; N] 与 Vec<T> 可通过切片比较相等。
// 【对应教材】Rust Book §8.1（Vector）
// 【解法思路】`let v = vec![10, 20, 30, 40];` 逐个列出元素；测试里 `assert_eq!(a, v[..])` 会把 Vec 转成切片再与数组比较。

// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    // TODO: declare your vector here with the macro for vectors
    let v = vec![10, 20, 30, 40]; // 💡 已完成：用 vec! 宏创建，元素与数组 a 相同（也可写 a.to_vec()）

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
