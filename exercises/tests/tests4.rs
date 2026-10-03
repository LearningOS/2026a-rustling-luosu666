// 📖 讲解：tests4 —— 验证字段与 should_panic
// 【题目要求】只修改测试函数：第一个测试断言构造出的 Rectangle 宽高正确；
//            后两个测试要验证传入负宽/负高时程序会 panic。
// 【考察知识点】`#[should_panic]` 属性（断言测试体内必须发生 panic）、
//            子模块 tests 可以直接访问父模块结构体的私有字段。
// 【对应教材】Rust Book 第 11 章 11.1 「用 should_panic 检查 panic」
//            https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic
// 【解法思路】宽高断言用 `rect.width` / `rect.height`（私有字段在同文件子模块可见）；
//            负数用例只需在 `#[test]` 下加一行 `#[should_panic]`，构造时构造函数
//            会 panic，测试即视为通过。
//
// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width // 💡 子模块可访问父模块的私有字段，直接比较宽
        assert_eq!(rect.height, 20); // check height // 💡 同上，比较高
    }

    #[test]
    #[should_panic] // 💡 声明本测试"应当 panic"：Rectangle::new(-10, 10) 触发 panic 后测试才算通过
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // 💡 同上：负高触发 panic，测试通过
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
