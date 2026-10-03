// 📖 讲解：move_semantics4
// 【题目要求】重构：不再把 vec0 传进 fill_vec，而是让函数自己创建 Vector 并返回给 main。
// 【考察知识点】所有权转移的另一个方向——函数内部创建数据并通过返回值把所有权交还给调用者；
//             以及“创建可变容器”的标准写法 `let mut vec = Vec::new();`。
// 【对应教材】Rust Book §4.1-4.2（所有权 / 返回值与作用域）
// 【解法思路】① main 中删掉 vec0（原文件已注释），改成无参调用 `fill_vec()`；
//             ② fill_vec 不再接收参数，改为 `let mut vec = Vec::new();` 在函数内新建，
//                push 完三个元素后把 vec 返回——所有权随返回值 move 回 main 的 vec1。

// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.


fn main() {
    //let vec0 = Vec::new(); // 💡 不再需要传入 vec0，此行删除（原文件已注释掉）

    let mut vec1 = fill_vec(); // 💡 无参调用；返回值的所有权 move 给 vec1，且需 mut 以便后续 push

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // 💡 函数内部新建可变 Vec，所有权从函数内产生

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // 💡 返回 vec：所有权随返回值交给调用者（move out）
}
