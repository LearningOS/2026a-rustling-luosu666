// 📖 讲解：iterators2 —— capitalize：chars、map 与 collect
// 【题目要求】三步：1) 实现把首字母大写的 capitalize_first；2) 对字符串切片数组
//            逐个 capitalize，返回 Vec<String>；3) 同样逐个 capitalize 后拼接成
//            一个 String。
// 【考察知识点】`char::to_uppercase()`（返回迭代器，需 collect 成 String）、
//            `Chars::as_str()` 取剩余子串、`iter().map(...).collect()` 的灵活
//            收集（收集目标由返回类型决定：Vec<String> 或 String）。
// 【对应教材】Rust Book 第 13 章 13.2-13.4（迭代器与 map/collect）
//            https://doc.rust-lang.org/stable/book/ch13-02-iterators.html
// 【解法思路】Step 1：首字符大写 collect 成 String，再拼接剩余部分 `c.as_str()`；
//            Step 2：`words.iter().map(|w| capitalize_first(w)).collect()`，返回类型
//            标注为 Vec<String>，collect 自动收集成向量；
//            Step 3：同样的 map 链，但返回类型是 String，collect 直接把所有
//            String 拼接成一个字符串（空格 " " 大写后不变，故得到 "Hello World"）。
//
// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(), // 💡 首字符大写收集成 String，as_str() 拼上剩余部分
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| capitalize_first(w)).collect() // 💡 逐词 capitalize，collect 成 Vec<String>
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|w| capitalize_first(w)).collect() // 💡 同样的 map 链，collect 目标是 String：直接把所有结果拼接
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
