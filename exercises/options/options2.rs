// 📖 讲解：options2
// 【题目要求】把两处残缺语句补全：第一处用 if let 从 Some 中解构出字符串；第二处用 while let 循环弹出 Vec 里的元素，注意 Vec::pop 返回的是 Option，元素本身又是 Option，即嵌套的 Option<Option<i8>>。
// 【考察知识点】if let / while let 模式匹配；模式的嵌套解构能力（模式可以直接写 Some(Some(x)) 一次剥掉两层包装）。
// 【对应教材】Rust Book §6.1-6.3（Option 与模式匹配）
// 【解法思路】if let Some(word) = optional_target { ... }；while let Some(Some(integer)) = optional_integers.pop() { ... }。当 pop 出最底部的 None 元素（或 Vec 被弹空返回 None）时，模式不匹配，循环自然结束。

// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target { // 💡 if let 只关心 Some 变体，把内部值绑定到 word
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() { // 💡 pop 返回 Option<Option<i8>>，嵌套模式 Some(Some(integer)) 一次解掉两层 Some
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
