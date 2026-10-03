// 📖 讲解：lifetimes2
// 【题目要求】longest 已带生命周期标注 'a。现在 main 里 result 在内层作用域赋值、却在 string2 已被销毁之后使用，违反了 'a 的约束。要让程序通过借用检查。
// 【考察知识点】生命周期标注只是"约束的检查"，真正决定能否通过的是引用的实际使用位置——要么延长数据存活时间，要么把引用的使用移到数据还活着的作用域里。
// 【对应教材】Rust Book §10.3（生命周期与作用域）
// 【解法思路】标准解法：把 result 的绑定和 println! 都移进内层作用域，在 string2 仍然存活时完成对 result 的使用。删掉外层提前声明的 let result;（悬垂声明本身没问题，问题出在跨作用域使用）。

// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the
// annotated parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz"); // 💡 string2 只活到这个内层作用域结束
        let result = longest(string1.as_str(), string2.as_str()); // 💡 result 移进内层作用域声明
        println!("The longest string is '{}'", result); // 💡 在 string2 还活着时使用 result，满足 'a 的约束
    }
}
