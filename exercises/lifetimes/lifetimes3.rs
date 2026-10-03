// 📖 讲解：lifetimes3
// 【题目要求】结构体 Book 的字段是字符串引用 &str，但结构体没有声明生命周期参数，编译报错。补上它。
// 【考察知识点】结构体持有引用时必须标注生命周期：struct Book<'a>，含义是"Book 实例存活期间，它借用的数据必须仍然存活"。
// 【对应教材】Rust Book §10.3（结构体定义中的生命周期标注）
// 【解法思路】在结构体名后声明 <'a>，把两个字段标成 &'a str。main 中 name/title 这两个 String 活得比 book 久，因此满足约束，无需改动 main。

// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> { // 💡 结构体持有引用：必须声明生命周期参数
    author: &'a str, // 💡 author 借用的数据至少要活得和 'a（也就是 Book 实例需要它的时间）一样久
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
