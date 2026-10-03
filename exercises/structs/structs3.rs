// 📖 讲解：structs3
// 【题目要求】为 Package 实现两个方法：is_international 判断是否国际件，get_fees 按单价计算运费，
//             补全返回类型和方法体，使测试通过。
// 【考察知识点】impl 块与关联函数/方法：`&self` 只读借用读取字段；方法返回类型的推导；
//             字段语法糖（字段名与局部变量同名时可简写，见 new 中的写法）。
// 【对应教材】Rust Book 第 5 章（§5.3 方法语法）
// 【解法思路】is_international -> bool：寄件国 != 收件国（String 可直接用 != 比较内容）。
//             get_fees -> i32：直接返回 weight_in_grams * cents_per_gram（测试用例 1500×3=4500、1500×6=9000）。

// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> bool { // 💡 返回 bool；&self 只读借用，不消耗 Package
        // Something goes here...
        self.sender_country != self.recipient_country // 💡 寄件国与收件国不同即国际件（String 支持 ==/!= 直接比较内容）
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32 { // 💡 返回 i32，与 weight_in_grams 的类型一致
        // Something goes here...
        self.weight_in_grams * cents_per_gram // 💡 运费 = 重量 × 每克单价
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
