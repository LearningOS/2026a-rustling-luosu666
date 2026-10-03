// 📖 讲解：if3
// 【题目要求】animal_habitat 根据动物名（crab/gopher/snake）返回栖息地，未知动物返回 "Unknown"；原题第一个 if 链的 else 分支类型不匹配，修好让 4 个测试通过。
// 【考察知识点】if/else if/else 各分支必须返回同一类型（这里统一为 i32），否则编译错误；第二条 if 链统一返回 &'static str。
// 【对应教材】Rust Book §3.5（控制流 - if 表达式）
// 【解法思路】identifier 的 else 分支返回 0（一个普通的 i32 字面量），未知动物经第二条 if 链落到 "Unknown"；两条 if 链各自类型一致即可。

// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        0 // 💡 else 分支也必须返回 i32：用 0 表示"未知动物"，与 1/2/3 类型统一
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown" // 💡 identifier 为 0 的未知动物（如 "dinosaur"）落到这里
    };

    habitat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
