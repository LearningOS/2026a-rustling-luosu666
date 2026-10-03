// 📖 讲解：strings4
// 【题目要求】main 里的 10 个值有的是 String 有的是 &str，给每一行选择正确的函数
//             （string_slice 接收 &str / string 接收 String），使程序编译通过。
// 【考察知识点】辨别每个表达式产出的类型是 String（拥有所有权、堆分配）还是 &str（借用/切片/字面量）：
//             · 字面量 "..." 是 &str；
//             · .to_string() / .to_owned() / String::from(..) / format!(..) / replace(..) / to_lowercase(..)
//               都产出新 String；其中 "nice weather".into() 因函数参数要求 String，Into 目标类型被推断为 String；
//             · &String[..] 切片、str 的方法 trim()（不改内容的方法）返回 &str。
//             另注意 &String 可自动 deref 成 &str，但函数参数精确类型不同时（String vs &str）不能混用。
// 【对应教材】Rust Book §8.2（字符串，重点理解 String 与 &str 的关系）
// 【解法思路】逐行判断类型后填函数名，答案见行内注释。

// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");        // 💡 字符串字面量本身就是 &str
    string("red".to_string());   // 💡 to_string() 把 &str 转成 String
    string(String::from("hi"));  // 💡 String::from 显式创建 String
    string("rust is fun!".to_owned()); // 💡 to_owned() 同样产出 String
    string("nice weather".into());     // 💡 into() 的目标类型由函数参数 String 推断，此处等价于 to_string()
    string(format!("Interpolation {}", "Station")); // 💡 format! 宏返回 String
    string_slice(&String::from("abc")[0..1]);       // 💡 对 String 做索引切片，得到的是 &str
    string_slice("  hello there ".trim());          // 💡 trim() 返回原字符串的切片 &str（不分配新内存）
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // 💡 replace() 返回新 String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());           // 💡 to_lowercase() 返回新 String
}
