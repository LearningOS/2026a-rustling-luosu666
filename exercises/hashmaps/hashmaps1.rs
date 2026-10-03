// 📖 讲解：hashmaps1
// 【题目要求】创建一个水果篮 HashMap<String, u32>：至少 3 种水果、总数至少 5 个。
// 【考察知识点】HashMap 的创建与插入：HashMap::new() 必须绑定到 mut 变量才能 insert；
//             键类型 String、值类型 u32。
// 【对应教材】Rust Book §8.3（使用 Hash Map 存储键值对）
// 【解法思路】`let mut basket = HashMap::new();`（必须 mut！），已有 banana:2，
//             再随便加两种即可——apple:3 + mango:4，共 3 种、总数 9，满足两个测试。
//             （insert 时键要用 String::from(..) 创建，不能直接塞 &str。）

// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.
                                     // 💡 必须 let mut：之后要 insert；类型由函数返回类型推断为 HashMap<String, u32>

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 3); // 💡 键是 String，用 String::from 创建
    basket.insert(String::from("mango"), 4); // 💡 3 种水果共 9 个，满足 >=3 种且 >=5 个

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
