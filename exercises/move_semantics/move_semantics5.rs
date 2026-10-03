// 📖 讲解：move_semantics5
// 【题目要求】只能调整 main 中各行的顺序（不增、不改、不删任何一行），让程序编译通过。
// 【考察知识点】借用检查规则：同一时刻对同一变量最多只能有一个可变引用（&mut）；
//             NLL（非词法作用域生命周期）——可变引用的借用期到“最后一次使用”为止，
//             而不是到作用域结束，因此前一个 &mut 用完之后就可以再借第二个。
// 【对应教材】Rust Book §4.2（引用与借用）
// 【解法思路】不能同时存在两个 &mut x，所以必须“串行”借用：
//             先建 y，用完 y（*y += 100），y 的借用即结束；
//             再建 z，用完 z（*z += 1000）；
//             最后两个借用都已结束，才能读取 x 做断言。
//             100 + 100 + 1000 = 1200，正好等于断言值。

// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x; // 💡 第一个可变借用
    *y += 100;      // 💡 y 的最后一次使用——此后 y 的借用结束（NLL），x = 200
    let z = &mut x; // 💡 此时才能开始第二个可变借用（如果这行放在 *y += 100 之前，两个 &mut 同时存活，编译报错）
    *z += 1000;     // 💡 x = 1200
    assert_eq!(x, 1200); // 💡 z 的借用也已结束，可以直接读取 x
}
