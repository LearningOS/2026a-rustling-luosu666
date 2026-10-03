// 📖 讲解：strings3
// 【题目要求】实现三个字符串处理函数 trim_me / compose_me / replace_me，使测试通过。
// 【考察知识点】str 的常用方法及其“输入/输出所有权”规律：
//             借用不改变内容的方法（trim、replace）返回借用或新字符串；
//             trim 返回 &str（原字符串的切片），replace 返回新 String；
//             format! 宏与 + 拼接 String 的方式。
// 【对应教材】Rust Book §8.2（字符串）
// 【解法思路】① trim_me：input.trim() 得到 &str（去掉两端空白后的切片），再 .to_string() 满足返回 String；
//             ② compose_me：format!("{} world!", input)（也可以 input.to_string() + " world!"）；
//             ③ replace_me：input.replace("cars", "balloons") 直接返回新 String。
//             规律：返回类型是 String 时，从 &str 出发几乎总要 .to_string() / .to_owned() / format!。

// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string() // 💡 trim() 返回去两端空白的 &str 切片，再 to_string() 变成拥有的 String
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input) // 💡 也可写成 input.to_string() + " world!"（注意 + 要求左边是 String）
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons") // 💡 replace 返回全新的 String，直接返回即可
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
