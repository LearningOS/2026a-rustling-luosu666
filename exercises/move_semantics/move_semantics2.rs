// 📖 讲解：move_semantics2
// 【题目要求】fill_vec 按值传参会拿走 vec0 的所有权，但 main 后面还要打印 vec0，
//             要求编译通过（rustlings 本题按“能否编译+运行”判定）。
// 【考察知识点】move 之后再使用原变量会报 "borrow of moved value"；官方 hint 给出三种解法：
//             ① 调用时传克隆（clone）；② 让 fill_vec 用不可变引用借用（&Vec）再在函数内复制出新的 Vec；
//             ③ 让 fill_vec 用可变引用（&mut Vec）原地修改、不返回值，main 里去掉 vec1 直接用 vec0。
// 【对应教材】Rust Book §4.1-4.2（所有权 / 引用与借用 / Clone）
// 【解法思路】本文件采用官方解法①：`fill_vec(vec0.clone())`，把克隆的所有权交给函数，
//             vec0 的所有权留在 main，之后仍可使用。
//             注意：解法①②下 vec0 打印时仍是空的（长度 0）；只有解法③（&mut 原地 push）
//             才能让 vec0 真正变成 `[22, 44, 66]`，即文件头部 “Expected output” 描述的效果。
//             三种都是官方认可的答案，本题只需编译通过即可。

// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone()); // 💡 解法①：克隆一份传给函数，vec0 的所有权留在 main，后面才能继续使用

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // 💡 shadow 为可变绑定，才能 push（与 move_semantics1 相同）

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
