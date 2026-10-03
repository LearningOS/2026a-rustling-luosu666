// 📖 讲解：tests7 —— 用 build.rs 设置环境变量
// 【题目要求】本文件不允许修改！解法在同目录的 `build.rs` 中：通过
//            `cargo:rustc-env=TEST_FOO=<当前时间戳>` 设置环境变量，使下面的测试
//            读到的 TEST_FOO 落在 [当前时间, 当前时间+10) 秒范围内。
// 【考察知识点】Cargo 构建脚本 build.rs、`cargo:` 指令中的 `rustc-env=VAR=VALUE`
//            （cargo 会把它提供给编译期 env! 宏，也会注入 cargo 所执行测试进程的
//            运行时环境）。
// 【对应教材】Cargo Book 构建脚本章节
//            https://doc.rust-lang.org/cargo/reference/build-scripts.html
// 【解法思路】在 build.rs 里取当前 Unix 时间戳，`println!("cargo:rustc-env=TEST_FOO={}", timestamp)`，
//            cargo 编译/运行测试时 TEST_FOO 即为该时间戳，测试断言通过。
//
// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap(); // 💡 此变量由 build.rs 里的 cargo:rustc-env=TEST_FOO=<时间戳> 提供
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
