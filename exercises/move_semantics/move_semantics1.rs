// 📖 讲解：move_semantics1
// 【题目要求】让程序编译通过：main 把 vec0 传给 fill_vec，函数向其中 push 元素后返回，
//             main 里还要继续对返回值 push(88)。
// 【考察知识点】所有权移动（move）与变量可变性（mut）的配合：Vec 按值传参即所有权被移入函数，
//             函数内要修改它必须声明为可变；返回后 main 接住的新绑定要继续修改也必须可变。
// 【对应教材】Rust Book §4.1-4.2（什么是所有权 / 引用与借用）
// 【解法思路】只需两处加 mut：
//             ① main 中 `let mut vec1 = ...`（后面 vec1.push(88) 需要可变）；
//             ② fill_vec 中 `let mut vec = vec;`（用 mut 重新 shadow 一份可变绑定，才能 push）。
//             所有权本身没有问题：vec0 被 move 进函数，函数把修改后的 Vec 返回给 main 接住。

// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0); // 💡 之后要 vec1.push(88)，所以绑定必须声明为 mut

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // 💡 参数 vec 默认不可变；shadow 成 mut 绑定后才能调用 vec.push（所有权此时已在函数里）

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
