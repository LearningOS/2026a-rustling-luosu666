// 📖 讲解：errors5
// 【题目要求】main 里同时用 ? 传播两种不同类型的错误：parse 的 ParseIntError 和 PositiveNonzeroInteger::new 的 CreationError。需要找一个公共的"错误 trait"作为统一的错误类型。
// 【考察知识点】Box<dyn Error> 这一"任何实现了 Error 的类型"的包装类型；trait 对象 dyn Trait；? 会自动用 From 把具体错误装箱。
// 【对应教材】Rust Book 第 9 章（§9.2 从 boxed trait 对象的错误中使用 ?）
// 【解法思路】把 ??? 换成 error::Error（文件已 use std::error）。这样 ParseIntError 和实现了 error::Error 的 CreationError 都能被 ? 自动 Box 起来放进同一个 Result。

// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> { // 💡 用 trait 对象 dyn error::Error 统一两种错误；? 会自动 Box 并转换
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?; // 💡 ParseIntError 实现了 Error，可被 ? 装进 Box<dyn Error>
    println!("output={:?}", PositiveNonzeroInteger::new(x)?); // 💡 CreationError 同样实现了 Error（见文件底部）
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
