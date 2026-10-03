// 📖 讲解：traits2
// 【题目要求】为 Vec<String> 实现 AppendBar：思考"给一个字符串向量 append Bar"意味着什么。
// 【考察知识点】为具体类型（Vec<String>）实现 trait；按值获取 self 并修改（mut self）；trait 一致性（孤儿规则下为本crate trait 实现外部类型没问题）。
// 【对应教材】Rust Book §10.2（Trait：为类型实现 trait）
// 【解法思路】"追加 Bar"= 向向量末尾 push 一个新的 String "Bar"，然后把向量原样返回。测试正是先 pop 出 "Bar" 再 pop 出 "Foo"，验证了这一点。

// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> { // 💡 为 Vec<String> 这个具体类型实现 trait
    fn append_bar(mut self) -> Self { // 💡 mut self：按值拿到 Vec，并允许修改它
        self.push(String::from("Bar")); // 💡 "追加 Bar" = 往末尾 push 一个字符串 "Bar"
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
