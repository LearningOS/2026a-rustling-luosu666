// 📖 讲解：cow1.rs —— Cow 写时克隆智能指针
// 【题目要求】abs_all 接收 &mut Cow<[i32]>，把负数取绝对值（需要写入时会调用 to_mut 触发克隆）。题目要求在 4 个单元测试的 match 里判断 abs_all 返回后 Cow 处于 Cow::Owned(_) 还是 Cow::Borrowed(_)，让所有测试通过。
// 【考察知识点】Cow<T>（Clone on Write）：Borrowed 变体包"借来的数据"（零拷贝），Owned 变体包"自己的数据"。规则：a) 从切片创建（borrow）且发生写入 → to_mut() 克隆一份，变成 Owned；b) 从切片创建但没发生写入 → 一直是 Borrowed；c) 从 Vec 创建（直接拥有）→ 无论是否写入，都保持 Owned（已拥有的数据 to_mut() 不会再克隆）。
// 【对应教材】Rust Book 第 15 章（智能指针章节；Cow 本身是标准库 std::borrow::Cow，可作为 15 章拓展阅读，也见第 13 章迭代器/克隆相关讨论）。
// 【解法思路】三个 TODO 分别对应上面 b、c、c 三种情形：reference_no_mutation 匹配 Cow::Borrowed(_)；owned_no_mutation 与 owned_mutation 都匹配 Cow::Owned(_)。第一个测试（借用 + 写入 → Owned）题目已给出，照着写即可。

// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // 题目已给出：借用数据 + 需要写入 → to_mut() 克隆成 Owned
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()), // 💡 借用创建 + 全是非负数、从未写入 → 保持 Borrowed，不发生克隆
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // 💡 一开始就是 Owned（从 Vec 创建），没有写入自然还是 Owned
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()), // 💡 已经拥有数据，to_mut() 直接返回对原有 Vec 的引用，不克隆，仍是 Owned
            _ => Err("Expected owned value"),
        }
    }
}
