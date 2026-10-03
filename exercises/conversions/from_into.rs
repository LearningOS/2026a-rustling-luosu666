// 📖 讲解：from_into
// 【题目要求】为 Person 实现 From<&str>，把 "Mark,20" 这样的 "名字,年龄" 字符串转换成 Person；
//             任何不合法输入（空串、缺字段、字段过多、名字为空、年龄解析失败）都回退到默认值 John/30。
// 【考察知识点】From trait 的实现（实现 From 即免费获得 Into）；split 迭代器；
//             Result 的 match/ok 处理；Default 作为兜底值。
// 【对应教材】std::convert::From 文档（https://doc.rust-lang.org/std/convert/trait.From.html）。
// 【解法思路】一次 `s.split(',')` 惰性迭代最多取三段：
//             （第 1 段，第 2 段 parse 的结果，第 3 段是否存在）。
//             只有当模式是 (Some(名字), Some(Ok(年龄)), None) —— 即恰好两段且年龄合法 —— 才构造 Person；
//             其余情况（包括 "Mike,32," 这种三段输入，第 3 段为 Some）一律返回 Person::default()。
//             最后再单独检查名字为空的情形（如 ",1"），也回退默认值。

// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // 💡 惰性 split：一次同时拿到「第 1 段 / 第 2 段 parse 结果 / 是否存在第 3 段」
        let mut split = s.split(',');
        let (name, age) = match (
            split.next(),                                // 💡 第 1 段：名字（split 至少产出一段，空串时为 Some("")）
            split.next().map(|age| age.parse::<usize>()), // 💡 第 2 段：把年龄解析成 usize，包成 Option<Result>
            split.next(),                                // 💡 第 3 段：若为 Some 说明逗号多于 1 个（如 "Mike,32,"），不合法
        ) {
            // 💡 只有「恰好两段 且 年龄解析成功」才是合法输入
            (Some(name), Some(Ok(age)), None) => (name, age),
            // 💡 其余情况（空串、缺名字/年龄、字段过多、年龄非法）一律回退默认值
            _ => return Person::default(),
        };
        // 💡 名字为空（如 ",1" 或 ","）也不合法，回退默认值
        if name.is_empty() {
            return Person::default();
        }
        Person {
            name: name.to_string(), // 💡 &str -> String 需要分配，用 to_string()
            age,
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
