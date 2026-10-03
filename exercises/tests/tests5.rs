// 📖 讲解：tests5 —— unsafe 函数契约与裸指针写内存
// 【题目要求】实现 `modify_by_address`：接收一个 usize 类型的地址，通过裸指针把
//            该地址上的 u32 改成测试期望的 0xAABBCCDD，并为 unsafe 块写安全说明。
// 【考察知识点】`unsafe fn` 与 `# Safety` 文档契约、`usize` 与 `*mut u32` 的相互
//            转换、裸指针解引用写值、SAFETY 注释规范。
// 【对应教材】Rust Book 第 19 章高级特性（unsafe Rust）/ The Rustonomicon "Safe & Unsafe"
//            https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
// 【解法思路】把 address 转回 `*mut u32` 裸指针，解引用并写入 0xAABBCCDD。
//            调用方（测试）已用契约保证地址有效且唯一，因此解引用是健全的。
//
// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The `address` is guaranteed by the contract of this function
    // to be a valid, uniquely owned mutable reference to a `u32`, so it is
    // sound to cast it back to `*mut u32` and write through it.
    unsafe {
        *(address as *mut u32) = 0xAABBCCDD; // 💡 usize 转回裸指针并解引用写入，使测试中的 t 变为 0xAABBCCDD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
