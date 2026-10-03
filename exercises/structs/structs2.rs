// 📖 讲解：structs2
// 【题目要求】利用结构体更新语法（struct update syntax）基于 order_template 创建 your_order，
//             只改 name 和 count，其余字段沿用模板，使测试通过。
// 【考察知识点】`..base` 结构体更新语法：未显式写出的字段从 base 中取值；
//             非 Copy 类型（String）的字段会被 move 出 base（部分移动），Copy 类型字段仅复制，
//             因此之后仍可访问 order_template.year 等字段，但不能再访问 order_template.name。
// 【对应教材】Rust Book 第 5 章（§5.1 中“使用结构体更新语法从其他实例创建实例”）
// 【解法思路】测试要求 name == "Hacker in Rust"、count == 1，其余与模板相同：
//             `Order { name: ..., count: 1, ..order_template }`。
//             注意 `..order_template` 必须放在最后。

// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        // let your_order =
        let your_order = Order {
            name: String::from("Hacker in Rust"), // 💡 测试要求的名字
            count: 1,                             // 💡 测试要求的数量
            ..order_template                      // 💡 其余字段全部沿用模板：year/电话/邮件/编号都和模板一致
        };
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year); // 💡 year 是 u32（Copy），模板被部分移动后仍可读取这些 Copy 字段
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
