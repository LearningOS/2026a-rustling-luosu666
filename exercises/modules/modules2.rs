// 📖 讲解：modules2
// 【题目要求】修复 delicious_snacks 里的 use 语句，使 main 能用 delicious_snacks::fruit
//             和 delicious_snacks::veggie 访问内部常量。
// 【考察知识点】`use ... as ...` 重命名导入；`pub use` 再导出（re-export）：
//             普通的 use 是私有的，引入的名字只在当前模块可见；要让外部通过
//             `delicious_snacks::fruit` 访问，必须用 `pub use` 把名字重新导出到模块公开接口上。
// 【对应教材】Rust Book 第 7 章（§7.4 将路径引入作用域：use / as / pub use）
// 【解法思路】use self::fruits::PEAR as fruit; → 别名要和 main 中使用的名字一致（fruit / veggie），
//             并且 use 前必须加 pub（pub use self::fruits::PEAR as fruit;），
//             否则 main 里 delicious_snacks::fruit 会报“私有”错误。

// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these use statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;       // 💡 as 起别名 fruit；pub use 才能把该名字暴露给模块外部
    pub use self::veggies::CUCUMBER as veggie; // 💡 同上，别名与 main 中的用法一致

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,  // 💡 依赖上面的 pub use 再导出，才能从模块外访问
        delicious_snacks::veggie
    );
}
