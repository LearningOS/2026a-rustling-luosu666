// 📖 讲解：clippy3.rs —— 一次性修掉多个常见 clippy 警告
// 【题目要求】这道题埋了四个坑，要求全部修到 cargo clippy -D warnings 零警告：(1) 对值为 None 的 Option 调 unwrap；(2) 数组字面量漏了逗号；(3) 把 resize（返回 unit）赋给变量；(4) 用临时变量手工交换两个数。
// 【考察知识点】四个 lint：对 None 的 unwrap（panicking_unwrap，新版 clippy 还有 unnecessary_literal_unwrap —— 字面量 None 上的 unwrap 直接删掉才是唯一干净解法）；数组缺逗号（rustc 提示 possibly missing a comma，且 -3 -4 会被当成减法静默算出 -7，很危险）；clippy::let_unit_value（把 () 绑定到变量，resize(0,5) 的正确替代是 clear()）；clippy::manual_swap（手工交换应使用 std::mem::swap）。
// 【对应教材】Rust Book 附录 D（Clippy）；mem::swap 见第 13 章"swap"相关内容（标准库 std::mem）。
// 【解法思路】(1) 删掉整个 `if my_option.is_none() { my_option.unwrap(); }` 块（my_option 留着不碍事，main 上已有 allow(unused_variables)）。(2) 给数组每行末尾补逗号。(3) 先 let mut 绑定 Vec 再调用 clear()，别把 resize 的 unit 返回值绑给变量。(4) 用 std::mem::swap(&mut a, &mut b) 一行完成交换。修完运行输出：数组是 [-1,-2,-3,-4,-5,-6]，Vec 为空，value a: 66 / value b: 45。

// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 💡 修复 1：原来的 `if my_option.is_none() { my_option.unwrap(); }` 已删除。
    //    它必然 panic（clippy::panicking_unwrap）；在 Rust 1.98 的 clippy 下，
    //    即使改成 is_some() 也会触发 unnecessary_literal_unwrap / unnecessary_unwrap
    //    （编译期就知道是字面量 None，任何 unwrap 都是多余的），所以干脆整块删掉。

    let my_arr = &[
        -1, -2, -3, // 💡 修复 2：补上漏掉的逗号。原来写成 `-3\n-4`，会被解析成 -3 - 4 = -7（合法但完全是另一个值！）
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5]; // 💡 修复 3：resize 返回 ()，把它绑定给变量触发 clippy::let_unit_value
    my_empty_vec.clear(); // 💡 resize(0, 5) 的语义就是"清空"，用 clear() 表达更直接
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // 💡 修复 4：手工三行交换触发 clippy::manual_swap，标准库一行搞定（原来的写法还有 bug：第二行赋值前 a 已经被覆盖）
    println!("value a: {}; value b: {}", value_a, value_b);
}
