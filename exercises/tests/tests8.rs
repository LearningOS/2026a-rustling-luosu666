// 📖 讲解：tests8 —— 用 build.rs 设置 cfg（feature）
// 【题目要求】本文件不需要修改！与 tests7 共用同一个 `build.rs`：在 build.rs 中
//            追加 `cargo:rustc-cfg=feature="pass"`，让测试里的
//            `#[cfg(feature = "pass")]` 生效，测试提前 return 而不走到 panic。
// 【考察知识点】`cargo:rustc-cfg=CFG[="VALUE"]` 指令与 `#[cfg(...)]` 条件编译的
//            联动；build.rs 可以同时为多个练习服务。
// 【对应教材】Cargo Book 构建脚本章节 + Rust Reference 条件编译
//            https://doc.rust-lang.org/cargo/reference/build-scripts.html
// 【解法思路】build.rs 中 `println!("cargo:rustc-cfg=feature=\"pass\"")`。
//            注意：cfg 名叫 feature，值是字符串 "pass"，与 `#[cfg(feature = "pass")]`
//            精确对应（只写 feature 不带值是匹配不到 `feature = "pass"` 的）。
//
// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")] // 💡 该 cfg 由 build.rs 的 cargo:rustc-cfg=feature="pass" 设置
        return;

        panic!("no cfg set");
    }
}
