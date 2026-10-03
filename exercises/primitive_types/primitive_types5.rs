// 📖 讲解：primitive_types5
// 【题目要求】解构 cat 元组，把名字和年龄分别绑定到 name、age 两个变量，让 println 正常工作。
// 【考察知识点】元组解构模式匹配：`let (a, b) = tuple;` 一次性拆出所有字段。
// 【对应教材】Rust Book §3.2（数据类型 - 元组）
// 【解法思路】把注释处换成模式 `(name, age)`，两个字段按位置依次绑定。

// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // 💡 元组解构：模式 (name, age) 按位置依次绑定 &str 和 f64 两个字段

    println!("{} is {} years old.", name, age);
}
