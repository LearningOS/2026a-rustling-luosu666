// 📖 讲解：traits4
// 【题目要求】compare_license_types 要能同时接收 SomeSoftware 和 OtherSoftware 两种不同类型作参数并调用它们的 licensing_info——只允许修改函数签名那一行。
// 【考察知识点】impl Trait 作函数参数（语法糖：等价于泛型 + trait bound），让函数接受"任何实现了 Licensed 的类型"。
// 【对应教材】Rust Book §10.2（Trait 作为参数）
// 【解法思路】两个参数都写成 impl Licensed。测试按值传参，所以不需要 &。注意 impl Licensed 表示"分别独立的任意实现了 Licensed 的类型"，两个参数可以是不同类型（例如 SomeSoftware 和 OtherSoftware 互换顺序也能通过测试）。

// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool { // 💡 impl Trait 作参数：任何实现了 Licensed 的类型都能传入
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
