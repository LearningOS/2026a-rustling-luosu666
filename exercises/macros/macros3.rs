// 📖 讲解：macros3.rs —— #[macro_export] 导出模块内的宏
// 【题目要求】宏定义被包在 mod macros 模块里，且题目要求不许把宏挪出模块，要让 main 里的 my_macro!() 能用。
// 【考察知识点】宏的路径/作用域规则：普通 use 不能直接引入宏；用 #[macro_export] 把宏导出到 crate 根（crate 级作用域），之后整个 crate 的任何地方（包括其他模块、其他 crate 用路径引入）都能按名字直接调用。
// 【对应教材】Rust Book 第 19 章 19.5 "Macros"（"On Macro Export"小节，讲解 #[macro_export] 与 #[macro_use]）。
// 【解法思路】在 macro_rules! my_macro 上面加一行 #[macro_export]。导出后宏进入 crate 根作用域，main 中的 my_macro!() 即可解析，模块本身保持原样不动。

// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    #[macro_export] // 💡 关键一行：把宏导出到 crate 根作用域，模块外的代码也能直接用宏名调用（宏仍留在模块里）
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
