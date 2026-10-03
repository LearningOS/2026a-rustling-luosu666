// 📖 讲解：tests6 —— 用 Box::from_raw 重建所有权
// 【题目要求】补全 `raw_pointer_to_box`：把裸指针重建成 Box（Box::from_raw），
//            并按测试要求把 `b` 字段设置为 `Some("hello")` 后返回。
// 【考察知识点】`Box::into_raw` / `Box::from_raw` 这一对所有权转移 API、裸指针与
//            Box 的关系、避免双重释放（from_raw 后原指针不可再动）。
// 【对应教材】Rust Book 第 15 章智能指针（Box）/ std::Box::from_raw 文档
//            https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
// 【解法思路】契约保证 ptr 指向一个被拥有的 Box<Foo>，用 `Box::from_raw(ptr)`
//            重新接管所有权（地址不变，所以测试中 ptr_1 == ptr_2）；再把 ret.b
//            设置为 Some("hello".to_owned()) 满足第二条断言，最后返回 ret。
//
// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) }; // 💡 从裸指针重建 Box，重新接管所有权（地址不变）
    ret.b = Some("hello".to_owned()); // 💡 测试断言要求 b 为 Some("hello")
    ret // 💡 返回重建后的 Box；&ret.a 与原 &data.a 地址相同，故 ptr_1 == ptr_2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
