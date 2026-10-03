// 📖 讲解：lifetimes1
// 【题目要求】longest 函数返回两个字符串引用中较长的一个，但编译器无法确定返回值的引用来自 x 还是 y，因此报"缺少生命周期标注"错误。请补上生命周期标注。
// 【考察知识点】生命周期标注 'a 的语法：&'a str 与函数签名 fn longest<'a>(...) -> &'a str；生命周期是描述引用之间关系的"元数据"，不改变任何实际存活时间。
// 【对应教材】Rust Book §10.3（生命周期：函数签名中的生命周期标注）
// 【解法思路】给函数加 <'a>，并把 x、y 和返回值都标成 &'a str。含义是：返回值的存活时间不会超过 x 和 y 中较短命的那个（借用检查器据此在调用处做验证）。

// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 💡 声明生命周期参数 'a：返回值与两个入参共同绑定，不活得比任何一方久
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
