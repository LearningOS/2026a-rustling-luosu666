// 📖 讲解：quiz3
// 【题目要求】成绩单系统目前只支持数字成绩（f32）。要修改 ReportCard 结构体和 impl 块，让它也支持字母成绩（如 "A+"），并把第二个测试的 grade 改成 "A+" 且仍然通过。
// 【考察知识点】结构体泛型化、impl 块上的 trait bound 约束（print 用 {} 打印 grade，要求 T: Display）。
// 【对应教材】Rust Book §10.1（泛型）、§10.2（trait bound）
// 【解法思路】把 grade 的类型 f32 换成泛型参数 T：struct ReportCard<T>。print 中 format! 的 {} 要求 T 实现 Display，因此 impl 块写成 impl<T: std::fmt::Display> ReportCard<T>。f32 和 String 都实现了 Display，两种成绩单都能正确打印。

// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard<T> { // 💡 结构体引入泛型参数 T，成绩类型不再写死
    pub grade: T,          // 💡 grade 可以是 f32、String 等任意类型
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> { // 💡 print 里用 {} 打印 grade，因此约束 T 必须实现 Display
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(), // 💡 成绩改成字母等级 "A+"，证明泛型化后同样适用
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
