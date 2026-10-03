// 📖 讲解：modules1
// 【题目要求】模块 sausage_factory 里的 make_sausage 要在模块外（main）被调用，让程序编译通过；
//             而 get_secret_recipe 不能暴露到模块外。
// 【考察知识点】模块与可见性：模块内的项默认私有（private），只有加了 pub 才能被父模块访问；
//             私有项在同一个模块内部可以自由互相调用——make_sausage 调 get_secret_recipe 没问题，
//             跨模块才受可见性限制。
// 【对应教材】Rust Book 第 7 章（§7.3 使用 use 关键字将路径引入作用域 / §7.2 引用模块项目的路径）
//             可见性核心内容在 §7.3“使用 pub 关键字暴露路径”
// 【解法思路】只改一处：给 make_sausage 加 pub（`pub fn make_sausage()`）。
//             get_secret_recipe 保持私有，满足 “Don't let anybody outside of this module see this!”。

// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String { // 💡 保持私有：模块外不可见，模块内可正常调用
        String::from("Ginger")
    }

    pub fn make_sausage() { // 💡 加 pub：让 main 能通过 sausage_factory::make_sausage() 调用
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
