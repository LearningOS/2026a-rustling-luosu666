// 📖 讲解：as_ref_mut
// 【题目要求】给三个函数补上合适的泛型约束并实现函数体：
//             byte_counter/char_counter 统计字节/字符数（需要 AsRef<str>，兼容 &str 和 String）；
//             num_sq 用 as_mut() 原地平方一个数（需要 AsMut<u32>，兼容 Box<u32> 等）。
// 【考察知识点】AsRef/AsMut 引用到引用的廉价转换；泛型 trait bound 的书写；
//             &str 与 String 都实现 AsRef<str>，Box<u32> 实现 AsMut<u32>。
// 【对应教材】std::convert::AsRef / AsMut 文档。
// 【解法思路】T: AsRef<str> 时 arg.as_ref() 得到 &str，即可用 as_bytes().len() 和 chars().count()；
//             num_sq 先通过 *arg.as_mut() 读出当前值（u32 是 Copy），再写回 v*v。
//             注意不能一行写 `*arg.as_mut() *= *arg.as_mut()`，两次可变借用会冲突。

// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// Obtain the number of bytes (not characters) in the given argument.
// Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    // 💡 T: AsRef<str> 让 &str / String 都能传入；as_bytes() 按 UTF-8 字节计数
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    // 💡 chars() 按 Unicode 标量值计数（"é" 是 1 个字符但占 2 个字节）
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // 💡 先通过 as_mut() 取出当前值（u32 是 Copy 类型），避免两次可变借用
    let v = *arg.as_mut();
    // 💡 再拿到 &mut u32 原地写回平方值；Box<u32> 等容器都实现了 AsMut<u32>
    *arg.as_mut() = v * v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
