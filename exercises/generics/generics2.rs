// 📖 讲解：generics2
// 【题目要求】Wrapper 目前只能包装 u32。把它改写成泛型版本，使 Wrapper::new(42) 和 Wrapper::new("Foo") 都能用。
// 【考察知识点】结构体泛型参数 <T>、impl<T> 泛型实现块（方法对任意 T 都可用）。
// 【对应教材】Rust Book §10.1（泛型数据类型：结构体定义中的泛型、方法定义中的泛型）
// 【解法思路】struct Wrapper 声明一个类型参数 T，字段 value: T；impl 块写成 impl<T> Wrapper<T>，new 也接收 T。编译器会为实际用到的每种 T 单态化出一份代码。

// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> { // 💡 结构体声明泛型参数 T
    value: T,       // 💡 字段类型由 T 决定，任意类型都能装
}

impl<T> Wrapper<T> { // 💡 impl<T> 表示为所有 T 实现这些方法
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
