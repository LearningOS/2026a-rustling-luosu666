// 📖 讲解：iterators1 —— 迭代器基础：iter() 与 next()
// 【题目要求】补全 4 个 `???`：把 Vec 变成可调用 next() 的迭代器，并按顺序给出
//            每次 next() 应返回的值（最后耗尽时返回 None）。
// 【考察知识点】`Vec::iter()` 创建迭代器、`Iterator::next()` 的行为（每次返回
//            Option<&T>，逐个前进，耗尽后永远返回 None）、迭代器需要 `mut`。
// 【对应教材】Rust Book 第 13 章 13.2 用迭代器处理一系列元素
//            https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
// 【解法思路】Step 1 用 `my_fav_fruits.iter()`（元素是 &str，next() 得到 Some(&"banana")，
//            与断言里的 & 引号形式一致）；Step 2~4 依次填 Some(&"custard apple")、
//            Some(&"peach")、None（第五个元素之后迭代器耗尽）。
//
// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter(); // 💡 Step 1: iter() 借出元素，next() 返回 Some(&"banana") 这样的引用

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple")); // 💡 Step 2: 第二个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // 💡 Step 3: 第四个元素
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None); // 💡 Step 4: 迭代器耗尽，之后 next() 永远返回 None
}
