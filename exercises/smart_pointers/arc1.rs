// 📖 讲解：arc1.rs —— Arc 原子引用计数跨线程共享数据
// 【题目要求】把一个含 0..99 的 Vec 交给 8 个线程同时使用：每个线程统计"下标偏移量 offset 上所有 8 的倍数余 offset 的数"之和。要求填两处 TODO，且不要复制 numbers 这个 Vec。
// 【考察知识点】Arc<T>（Atomically Reference Counted）：线程安全版的 Rc；Arc::new 把数据装箱为共享所有权，Arc::clone 只克隆句柄（原子地 +1 计数），move 闭包把克隆出的 Arc 所有权移交给子线程。Rc 不能 Send，跨线程必须用 Arc（只读共享时无需 Mutex）。
// 【对应教材】Rust Book 第 16 章 16.3 "Shared-State Concurrency"（Arc 的动机：Rc 不能安全地跨线程计数）。
// 【解法思路】TODO1：shared_numbers = Arc::new(numbers)，把 Vec 的所有权装进 Arc（不是复制！）。TODO2：每个线程循环里 child_numbers = Arc::clone(&shared_numbers)，拿到自己的句柄，随 move 闭包进线程；线程只读，8 个 Arc 指向堆上同一份 Vec。主线程最后对每个 JoinHandle 调 join（原题已写好），等 8 个线程全部算完。

// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // 💡 TODO1：把 Vec 的所有权装进 Arc，之后谁也不能直接改它，只能共享
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // 💡 TODO2：每轮循环克隆一个新的 Arc 句柄（指向同一份堆数据），move 进子线程
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
