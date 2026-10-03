// 📖 讲解：from_str
// 【题目要求】为 Person 实现 FromStr：解析 "名字,年龄" 字符串，非法时返回具体错误枚举而非默认值。
//             空串→Empty；逗号分出的字段数不是 2→BadLen；名字为空→NoName；年龄解析失败→ParseInt(包装 ParseIntError)。
// 【考察知识点】FromStr trait 与 `str::parse`；自定义错误枚举；错误分支的精确对应。
// 【对应教材】std::str::FromStr 文档（https://doc.rust-lang.org/std/str/trait.FromStr.html）。
// 【解法思路】依次判断：1) s.is_empty() → Empty；
//             2) split(',') 逐段取出，第二段不存在 → BadLen（缺年龄）；
//             3) 第三段存在 → BadLen（"John,32," / "John,32,man" 逗号过多，这是最容易漏掉的分支！）；
//             4) 名字为空 → NoName（注意 "," 和 ",one" 测试要求 NoName 优先于年龄解析错误）；
//             5) 年龄 parse::<usize>() 失败 → ParseInt(e)；
//             否则返回 Ok(Person)。

// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an
//    error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error
//    should be returned
// If everything goes well, then return a Result of a Person object
//
// As an aside: `Box<dyn Error>` implements `From<&'_ str>`. This means that if
// you want to return a string error message, you can do so via just using
// return `Err("my error message".into())`.

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 💡 1. 空串 → Empty
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 💡 2. 惰性切分，逐段取出检查
        let mut split = s.split(',');
        let name = split.next().unwrap(); // 💡 split 至少产出一段，空串情形已在上面排除
        let age = match split.next() {
            Some(age) => age,                        // 💡 有第二段才能当年龄用
            None => return Err(ParsePersonError::BadLen), // 💡 缺第二段（如 "John"）→ BadLen
        };
        // 💡 3. 必须恰好两段：第三段存在（如 "John,32," / "John,32,man"）→ BadLen（易漏分支！）
        if split.next().is_some() {
            return Err(ParsePersonError::BadLen);
        }
        // 💡 4. 名字为空（如 ",1"、","）→ NoName（测试要求 NoName 分支能匹配 "," 与 ",one"）
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }
        // 💡 5. 解析年龄，失败时把 ParseIntError 包装成 ParseInt 变体
        match age.parse::<usize>() {
            Ok(age) => Ok(Person {
                name: name.to_string(),
                age,
            }),
            Err(e) => Err(ParsePersonError::ParseInt(e)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
