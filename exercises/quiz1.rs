// 📖 讲解：quiz1
// 【题目要求】苹果每个 2 rustbucks；若购买数量"多于 40 个"（more than 40），每个只卖 1 rustbuck。实现 calculate_price_of_apples 使测试通过。
// 【考察知识点】变量、函数、if 三章综合运用；重点是边界条件——"more than 40"是严格大于，恰好 40 个不打折。
// 【对应教材】Rust Book §3.1 + §3.3 + §3.5（变量/函数/if 综合测验）
// 【解法思路】`if number > 40 { number } else { number * 2 }`；对照测试：35→70、40→80（不打折！）、41→41、65→65，所以必须用 > 而不是 >=。

// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought. No hints this time!
//
// No hints this time ;)


// Put your function here!
fn calculate_price_of_apples(number: i32) -> i32 {
    // 💡 "more than 40" 是严格大于：40 个仍按单价 2 计价，41 个起单价降为 1
    if number > 40 {
        number // 💡 单价 1：总价 = 数量 × 1，直接返回 number
    } else {
        number * 2 // 💡 单价 2：总价 = 数量 × 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
