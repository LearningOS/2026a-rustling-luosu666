// 📖 讲解：traits3
// 【题目要求】为 SomeSoftware 和 OtherSoftware 实现 Licensed，让 licensing_info 返回相同的信息，但不允许修改两个空的 impl 块（"不写两遍同样的函数"）。
// 【考察知识点】trait 方法的默认实现（default implementation）。
// 【对应教材】Rust Book §10.2（Trait：默认实现）
// 【解法思路】既然 impl Licensed for SomeSoftware {} 是空的也能编译通过，就把方法体直接写进 trait 定义里作为默认实现——两个类型自动获得相同行为，一处定义、处处复用。

// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String { // 💡 直接在 trait 里给出默认实现，空的 impl 块即可继承它
        "Some information".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
