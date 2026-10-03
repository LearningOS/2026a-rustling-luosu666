// 📖 讲解：structs1
// 【题目要求】补全三种结构体的定义与实例化，使测试通过：经典 C 风格结构体、元组结构体、单元结构体。
// 【考察知识点】① 经典结构体：命名字段 + `Struct { field: value }` 语法实例化；
//             ② 元组结构体：匿名字段，用 `.0` `.1` `.2` 访问，`Struct(v0, v1, v2)` 实例化；
//             ③ 单元结构体：没有任何字段，本身就是一个值，需 `#[derive(Debug)]` 才能用 `{:?}` 打印。
// 【对应教材】Rust Book 第 5 章（使用结构体组织相关联的数据，§5.1 定义并实例化结构体）
// 【解法思路】字段类型选 i32 即可（断言里 0/255 是无后缀整数字面量，会按字段类型推断）。
//             三个测试分别用三种语法实例化 green / unit_like_struct；单元结构体直接写名字即可。

// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

struct ColorClassicStruct {
    // TODO: Something goes here
    red: i32,   // 💡 经典结构体：命名字段，类型 i32（断言中的 0/255 字面量会自动推断为 i32）
    green: i32, // 💡
    blue: i32,  // 💡
}

struct ColorTupleStruct(i32, i32, i32); // 💡 元组结构体：直接罗列字段类型，无字段名，用 .0/.1/.2 访问

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct { red: 0, green: 255, blue: 0 }; // 💡 用“字段名: 值”语法实例化

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green = ColorTupleStruct(0, 255, 0); // 💡 像元组一样按位置传值

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let unit_like_struct = UnitLikeStruct; // 💡 单元结构体无需任何数据，名字本身就是值；因要用 {:?} 打印，上方需要 #[derive(Debug)]
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
