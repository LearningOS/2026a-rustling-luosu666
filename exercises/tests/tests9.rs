// 📖 讲解：tests9 —— extern 块、#[no_mangle] 与 #[link_name] 链接别名
// 【题目要求】只允许添加两行属性：让 extern 块声明的 `my_demo_function` 和
//            `my_demo_function_alias` 都能链接到 mod Foo 里的同名 Rust 函数。
// 【考察知识点】`extern "Rust"` 块声明外部函数、符号默认会被 name-mangling（混淆）、
//            `#[no_mangle]` 以原名导出符号、`#[link_name = "..."]` 为声明指定实际
//            链接的符号名（做别名）。
// 【对应教材】Rust Book 第 19 章高级特性（调用 Rust 或 C 编写的外部代码）/ Rust
//            Reference 的 ABI 与 Linkage 章节
//            https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
// 【解法思路】两行属性缺一不可：
//            1) 在 mod Foo 的函数定义上加 `#[no_mangle]`，关闭符号混淆，把函数以
//               符号名 `my_demo_function` 导出到链接环境；
//            2) 在 extern 块的 `my_demo_function_alias` 声明上加
//               `#[link_name = "my_demo_function"]`，说明这个声明实际链接到上面的符号。
//            这样两个声明解析到同一个函数，测试即可通过。
//
// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.

extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"] // 💡 属性 2：别名声明实际链接到符号 my_demo_function
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle] // 💡 属性 1：不混淆符号名，将函数以原名 my_demo_function 导出供 extern 块链接
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
