// 📖 讲解：options3
// 【题目要求】在不删除最后一行 `y;` 的前提下让程序编译通过。问题是 match 默认会把 y 里的 Point 移动（move）出去，之后再使用 y 就报"值已被移动"的错误。
// 【考察知识点】match 与所有权的关系、模式中使用 ref 进行借用（Point 没有 derive Copy）。
// 【对应教材】Rust Book §6.1-6.3（Option 与 match）、§4.2（引用与借用）
// 【解法思路】把模式写成 Some(ref p)，在匹配时只借用 Point 而不拿走所有权，这样最后一行的 y 仍然可用。另一种等价写法是 match &y { Some(p) => ... }。

// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // 💡 ref 让 p 只是引用，不移动 y 中 Point 的所有权
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
