// 📖 讲解：threads1.rs —— JoinHandle 与收集线程返回值
// 【题目要求】主线程 spawn 10 个子线程，每个至少睡 250ms 并返回自己实际耗时。程序必须等全部子线程结束，并把它们的返回值收集进 results 向量，最后校验 results.len() == 10。
// 【考察知识点】thread::spawn 返回 JoinHandle<T>；handle.join() 会阻塞等待线程结束并返回线程闭包的返回值（Result<T>，线程 panic 时返回 Err）。join 的另一个作用：保证子线程在主线程退出前跑完。
// 【对应教材】Rust Book 第 16 章 16.1 "Using Threads to Run Code Simultaneously"（spawn 与 join 一节，书中正是用 move 闭包 + join 收集结果的例子）。
// 【解法思路】TODO 在 for handle in handles 循环里：对每个 JoinHandle 调 handle.join().unwrap()，把拿到的 u128 耗时 push 进 results。这样既有"等待"又有"收集"两个效果，一处改动同时满足校验。

// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        results.push(handle.join().unwrap()); // 💡 join() 阻塞等待该线程结束，并取回闭包返回值（as_millis() 的 u128）；join 返回 Result，用 unwrap 解包
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
