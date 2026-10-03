// 📖 讲解：generics1
// 【题目要求】购物清单程序编译不过：Vec<?> 不是合法类型。要让 Vec 能装 "milk" 这种字符串字面量。
// 【考察知识点】Vec<T> 的元素类型、&str 字符串切片（字符串字面量的类型）、类型推断。
// 【对应教材】Rust Book §10.1（泛型数据类型）、§4.3（字符串切片）
// 【解法思路】"milk" 是字符串字面量，类型为 &'static str，所以把 ? 换成 &str 即可。也可以不写类型标注让编译器推断，但按题意这里明确写出 Vec<&str>。

// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // 💡 "milk" 是 &str（字符串字面量），元素类型写成 &str
    shopping_list.push("milk");
}
