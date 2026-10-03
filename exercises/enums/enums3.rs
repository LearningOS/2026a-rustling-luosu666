// 📖 讲解：enums3
// 【题目要求】实现 Message 的变体类型，并在 State::process 中用 match 把每种消息分发给
//             State 已有的对应方法，使状态机测试通过。
// 【考察知识点】枚举 + match 穷尽匹配驱动的状态机；match 绑定变体中的数据并把值传给方法；
//             元组作为参数时的书写方式。
// 【对应教材】Rust Book 第 6 章（§6.1 枚举、§6.2 match 控制流运算符）
// 【解法思路】① 从测试用例反推变体：ChangeColor(255,0,255) → ChangeColor(u8,u8,u8)；
//             Echo(String::from(...)) → Echo(String)；Move(Point{x,y}) → Move(Point)（复用已有结构体 Point）；Quit → 单元变体。
//             ② process 里 match message，四个分支分别调用 self.change_color/echo/move_position/quit。
//             match 必须穷尽所有变体，四个分支一个不能少（这里不用通配符 _，让编译器帮我们检查）。
//             ChangeColor 的 (r, g, b) 解构后再组合成元组 (r, g, b) 传给 change_color。

// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

enum Message {
    // TODO: implement the message variant types based on their usage below
    ChangeColor(u8, u8, u8), // 💡 携带 RGB 三个分量的元组变体
    Echo(String),            // 💡 携带要回显的字符串
    Move(Point),             // 💡 直接复用 Point 结构体作为数据
    Quit,                    // 💡 单元变体：无需数据
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))
        match message {
            // 💡 解构出 r/g/b，再打包成元组传给 change_color（它接收 (u8,u8,u8)）
            Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
            // 💡 Echo(s) 中的 String 按 move 语义交给 echo，不产生额外拷贝
            Message::Echo(s) => self.echo(s),
            // 💡 Move(p) 中的 Point 直接 move 给 move_position
            Message::Move(p) => self.move_position(p),
            // 💡 单元变体不携带数据，直接调用 quit
            Message::Quit => self.quit(),
        } // 💡 match 穷尽四个变体，没有遗漏（不加 _ 通配符，编译器会保证全覆盖）
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
