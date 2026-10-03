// 📖 讲解：vecs2
// 【题目要求】vec_loop 用循环把 Vec 中每个元素乘 2（原地修改）；vec_map 用迭代器 map 生成一个每个元素乘 2 的新 Vec。
// 【考察知识点】iter_mut() 拿可变引用并解引用 `*element` 修改元素；iter().map(闭包).collect() 的函数式写法。
// 【对应教材】Rust Book §8.1（Vector - 遍历）与 §13.2（迭代器）
// 【解法思路】循环体写 `*element *= 2;`（element 是 &mut i32，要先解引用）；map 闭包里直接写 `element * 2` 返回新值，再 collect() 汇总。

// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2; // 💡 element 是 &mut i32 可变引用，必须先解引用 *element 再 *= 2
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2 // 💡 闭包直接返回新值（element 是 &i32，数值运算会自动解引用），由 collect() 汇总成新 Vec
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
