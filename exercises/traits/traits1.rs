// 📖 讲解：traits1
// 【题目要求】为 String 实现 trait AppendBar：append_bar(self) 消费原字符串，返回末尾追加了 "Bar" 的新字符串。
// 【考察知识点】impl Trait for Type 语法、trait 方法的实现（签名必须与 trait 声明一致）、self 按值传递。
// 【对应教材】Rust Book §10.2（Trait：定义与实现）
// 【解法思路】用 format!("{}Bar", self) 生成新 String。因为 trait 方法签名是 fn append_bar(self) -> Self，返回值仍是 String，所以可以链式调用 .append_bar().append_bar()。

// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(self) -> Self {
        format!("{}Bar", self) // 💡 self 是被消费的 String，用 format! 在末尾拼上 "Bar" 并返回新 String
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
