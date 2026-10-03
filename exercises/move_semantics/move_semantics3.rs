// 📖 讲解：move_semantics3
// 【题目要求】不许新增任何一行，只改已有行，让程序编译通过。
// 【考察知识点】参数模式匹配处的 mut：与其在函数体内 `let mut vec = vec;` 再 shadow 一次，
//             不如直接在参数上写 `mut vec: Vec<i32>`，一步到位。
// 【对应教材】Rust Book §4.1-4.2（所有权 / 可变性）
// 【解法思路】两处修改（都是把已有行加个 mut）：
//             ① main：`let mut vec1 = fill_vec(vec0);`（后面要 push 88）；
//             ② fill_vec 签名：`fn fill_vec(mut vec: Vec<i32>)`（函数体内直接 push，无需 shadow）。
//             对比 move_semantics1：本题展示的是“在参数列表里声明可变”这一更简洁的等价写法。

// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // 💡 加 mut：之后 vec1.push(88) 需要可变

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> { // 💡 在参数上直接声明 mut（替代函数体内的 `let mut vec = vec;`），函数体内即可直接 push
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
