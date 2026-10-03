// 📖 讲解：box1.rs —— Box 与递归类型（cons list）
// 【题目要求】定义一个函数式编程里常见的 cons list（链表）枚举 List。第 1 步：在枚举定义里用 Box 让代码能编译；第 2 步：把两个 todo!() 换成真正构造"空表"和"非空表"的代码。注意：测试不许改。
// 【考察知识点】Box<T> 智能指针：把数据放到堆上；递归类型必须有确定大小；Deref 解引用可以像普通值一样用 Box 里的内容。
// 【对应教材】Rust Book 第 15 章（15.1 "Using Box<T> to Point to Data on the Heap"、15.1 末尾 "Cons List" 例子，与本题几乎完全一致）。
// 【解法思路】直接写 Cons(i32, List) 会让编译器陷入"无限大小"（error[E0072]: recursive type has infinite size）。编译器在编译期必须知道每个类型占多少字节，而递归类型会无限嵌套下去。解决办法是在递归位置放一个 Box<List>：Box 是一个固定大小的"指针"，指向堆上的下一个 List，递归链到此就能算出大小。构造时用 Box::new(List::Nil) 把下一节点装箱。

// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 💡 Step 1：递归字段改成 Box<List>。Box 是固定大小的堆指针，打破了"无限大小"的死循环
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 💡 Step 2：空 cons list 就是单独一个 Nil 节点
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil)) // 💡 Step 2：非空表 = 当前元素 + 装箱的下一个节点（这里是 Nil 结尾）。数字随便填，只要不等于空表即可
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
