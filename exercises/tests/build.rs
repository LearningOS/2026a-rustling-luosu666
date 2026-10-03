// 📖 讲解：build.rs —— tests7 / tests8 共用的构建脚本
// 【题目要求】补全两条 `cargo:` 指令：为 tests7 设置环境变量 TEST_FOO（当前时间戳），
//            为 tests8 打开 `feature = "pass"` 的 cfg。
// 【考察知识点】Cargo 构建脚本 build.rs 的 `cargo:` 输出协议：
//            - `cargo:rustc-env=VAR=VALUE`：设置环境变量（编译期 env! 可读，
//              cargo 运行测试/二进制时也会注入进程环境，故 std::env::var 也能读到）；
//            - `cargo:rustc-cfg=CFG[="VALUE"]`：设置条件编译 cfg。
// 【对应教材】Cargo Book 构建脚本章节
//            https://doc.rust-lang.org/cargo/reference/build-scripts.html
// 【解法思路】tests7 的测试要求 TEST_FOO 是"最近 10 秒内"的时间戳，所以 build.rs
//            在构建时取当前 Unix 时间戳写入 TEST_FOO（构建与测试运行只差几秒）；
//            tests8 的测试在 `#[cfg(feature = "pass")]` 时提前 return，因此用
//            `rustc-cfg=feature="pass"` 设置该 cfg（注意必须带 ="pass" 值）。

//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // 💡 时间戳作为 TEST_FOO 的值：构建时刻的秒数，恰好落在测试允许的 10 秒窗口内
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = r#"rustc-cfg=feature="pass""#; // 💡 设置 cfg feature="pass"，与 #[cfg(feature = "pass")] 匹配
    println!("cargo:{}", your_command);
}
