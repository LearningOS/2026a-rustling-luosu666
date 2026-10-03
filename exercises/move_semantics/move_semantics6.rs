// 📖 讲解：move_semantics6
// 【题目要求】只允许“增加或删除引用”（&），让程序编译通过：
//             get_char 只读取不该拿走所有权；string_uppercase 应取得所有权并真正改成大写后打印。
// 【考察知识点】① 什么时候用 &String / &str 借用而不是拿走所有权；
//             ② to_uppercase() 返回一个全新的 String（不修改原字符串），必须用返回值重新赋值。
// 【对应教材】Rust Book §4.2（引用与借用）
// 【解法思路】get_char 改为借用：调用处 `get_char(&data)`，签名改为 `data: &String`，
//             这样 data 的所有权仍留在 main，之后还能传给 string_uppercase。
//             string_uppercase 保持按值接收（拿到所有权），但 `to_uppercase()` 不会原地修改，
//             需写成 `data = data.to_uppercase();` 把新 String 赋回给已声明 mut 的参数。
//             （仓库中曾出现的 `get_char(data.clone())` 写法也能编译，但改动方式是加 clone 而非加引用，
//             且 get_char 实际拿走的是克隆的所有权，不符合题目“只增删引用”的约束，故未采用。）

// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 💡 加 & 传引用：get_char 只借用 data，不拿走所有权

    string_uppercase(data); // 💡 最后一次使用 data，把所有权交出去（move）
}

// Should not take ownership
fn get_char(data: &String) -> char { // 💡 签名改为 &String：只借用，只读不拥有
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 💡 to_uppercase() 返回全新 String 并不修改自身，必须重新赋值（参数已声明 mut）

    println!("{}", data);
}
