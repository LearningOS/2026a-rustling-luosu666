// 📖 讲解：enums2
// 【题目要求】根据 main 中的使用方式，为 Message 定义四种不同形态的变体，使程序编译通过。
// 【考察知识点】枚举变体的三种携带数据的形式：
//             ① 结构体变体 `Move { x, y }`（具名字段）；② 元组变体 `Echo(String)`（匿名数据）；
//             ③ 单元变体 `Quit`（无数据）。
// 【对应教材】Rust Book 第 6 章（§6.1 定义枚举）
// 【解法思路】逐行看用法反推定义：
//             Message::Move { x: 10, y: 30 }   → Move { x: i32, y: i32 }
//             Message::Echo(String::from(...)) → Echo(String)
//             Message::ChangeColor(200,255,255)→ ChangeColor(u8, u8, u8)（三个数，u8/i32 均可，这里选 u8）
//             Message::Quit                    → Quit（单元变体）
//             变体数据类型可以各不相同——这正是枚举比结构体灵活的地方。

// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },          // 💡 结构体变体：像结构体一样带具名字段，实例化时用 Move { x, y }
    Echo(String),                     // 💡 元组变体：携带一个 String
    ChangeColor(u8, u8, u8),          // 💡 元组变体：携带三个 u8（RGB）
    Quit,                             // 💡 单元变体：不携带数据
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
