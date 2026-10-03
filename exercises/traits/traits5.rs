// 📖 讲解：traits5
// 【题目要求】some_func 的参数 item 要同时能调用 some_function（SomeTrait 的方法）和 other_function（OtherTrait 的方法）——只允许修改函数签名那一行。
// 【考察知识点】复合 trait bound：impl TraitA + TraitB，要求参数类型同时实现多个 trait。
// 【对应教材】Rust Book §10.2（使用 trait bound 约束实现多个 trait）
// 【解法思路】把参数写成 impl SomeTrait + OtherTrait。SomeStruct 和 OtherStruct 都实现了这两个 trait，所以都能传入。等价写法是泛型函数 fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool。

// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: impl SomeTrait + OtherTrait) -> bool { // 💡 加号连接多个 trait bound：要求 item 同时实现两个 trait
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
