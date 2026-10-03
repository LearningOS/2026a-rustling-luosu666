// 📖 讲解：quiz2
// 【题目要求】实现 my_module::transformer：接收 Vec<(String, Command)>，按每条命令
//             （Uppercase 大写 / Trim 去空白 / Append(n) 追加 n 个 "bar"）变换对应字符串，
//             返回 Vec<String>，并补全测试模块的 use 导入。
// 【考察知识点】综合题：函数签名设计（Vec<(String, Command)> -> Vec<String>）、
//             枚举 match 处理（含携带数据的 Append(usize) 变体）、迭代器 iter() 借用遍历、
//             String 方法（to_uppercase / trim / push_str）、模块与 use 路径导入。
// 【对应教材】综合：Rust Book §4.1-4.2（move）、第 6 章（枚举与 match）、第 7 章（模块）、§8.1-8.2（Vec/String）
// 【解法思路】① 签名：input: Vec<(String, Command)>，返回 Vec<String>；
//             ② output 声明为 Vec<String>；
//             ③ 用 input.iter() 借用遍历（不必消耗 input），解构出 (&String, &Command)，
//                match command 按三种命令生成新字符串 push 进 output；
//                Append(n) 中 n 绑定为 &usize，用 *n 取值循环 push_str("bar")；
//             ④ 测试模块需要 use super::my_module::transformer; 把函数引入作用域。

// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> { // 💡 入参类型与返回类型都由测试的调用方式反推出来
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![]; // 💡 保存变换后的字符串，需要 mut 才能 push
        for (string, command) in input.iter() { // 💡 iter() 借用遍历：string 是 &String，command 是 &Command
            // TODO: Complete the function body. You can do it!
            match command { // 💡 对 &Command 做 match，三种命令分别处理，分支必须穷尽
                Command::Uppercase => output.push(string.to_uppercase()),       // 💡 to_uppercase 返回新 String
                Command::Trim => output.push(string.trim().to_string()),        // 💡 trim 得 &str，再 to_string 变 String
                Command::Append(n) => {                                         // 💡 n 绑定为 &usize（match 引用时的自动解构）
                    let mut s = string.clone();                                 // 💡 借用遍历拿不到所有权，克隆一份来追加
                    for _ in 0..*n {                                            // 💡 *n 解引用取循环次数
                        s.push_str("bar");                                      // 💡 追加 n 个 "bar"
                    }
                    output.push(s);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer; // 💡 transformer 定义在 my_module 里，用 use 路径引入（也可用 use super::my_module::*;）
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
