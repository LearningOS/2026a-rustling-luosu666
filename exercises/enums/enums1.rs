// 📖 讲解：enums1
// 【题目要求】定义 Message 枚举，使 main 中的四个变体 Message::Quit / Echo / Move / ChangeColor
//             都能编译并用 {:?} 打印出来。
// 【考察知识点】枚举变体的基本形式。本题主打“单元变体”（unit variant，不带数据的变体）——
//             每个变体就是枚举类型的一个可能取值；#[derive(Debug)] 使 {:?} 打印变体名。
// 【对应教材】Rust Book 第 6 章（§6.1 定义枚举）
// 【解法思路】main 只是打印这四个变体，没有携带任何数据，所以四个全部定义为单元变体即可：
//             Quit, Echo, Move, ChangeColor（对比 enums2，那里会用到带数据的变体）。

// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,        // 💡 单元变体：不携带数据，名字本身就是值
    Echo,        // 💡
    Move,        // 💡
    ChangeColor, // 💡
}

fn main() {
    println!("{:?}", Message::Quit);        // 💡 derive(Debug) 后可用 {:?} 打印，输出 "Quit"
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
