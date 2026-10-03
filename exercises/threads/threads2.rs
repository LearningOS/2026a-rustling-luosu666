// 📖 讲解：threads2.rs —— Arc<Mutex<T>> 跨线程共享可变数据
// 【题目要求】10 个子线程并发地把同一个共享值 JobStatus.jobs_completed 各加 1，主线程在每个线程 join 后打印当前的完成数。
// 【考察知识点】Mutex<T>（互斥锁）内部可变性：修改共享数据前必须先 lock()；Arc<Mutex<T>> 组合 = "多线程共享 + 可变"。lock() 返回 Result<Guard, PoisonError>，惯例用 .unwrap()；Guard 离开作用域自动解锁（RAII）。
// 【对应教材】Rust Book 第 16 章 16.3 "Shared-State Concurrency"（Arc<Mutex<i32>> 计数器例子，与本题同构）。
// 【解法思路】(1) 创建时用 Mutex 包一层：Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }))。(2) 子线程更新前先 let mut status_locked = status_shared.lock().unwrap()，再 status_locked.jobs_completed += 1（这就是"更新共享值之前必须做的那个动作"）。(3) 主线程读取时同样要先 lock。观察输出：由于 10 个线程都先并发地睡 250ms 再更新，第一个 join 返回时它们几乎都已加完，所以打印出来的常常直接是 10（或接近 10）——打印的是"此刻"的完成数，不必等全部线程结束；但主线程必须逐个 join，否则 main 提前退出会把还没跑完的线程直接杀掉。

// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex}; // 💡 引入 Mutex（原题只有 Arc）
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 })); // 💡 只有 Arc 的话数据不可变，必须再包一层 Mutex 才能跨线程修改
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status_locked = status_shared.lock().unwrap(); // 💡 更新前先加锁：lock() 拿到 MutexGuard，unwrap 处理锁中毒的情况
            status_locked.jobs_completed += 1; // 💡 通过 Guard 解引用修改共享数据；Guard 在线程结束时自动释放锁
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed); // 💡 主线程读共享值同样要先拿锁；每 join 完一个线程打印一次"此刻"的完成数（各线程几乎同时睡醒，所以常直接打出 10）
    }
}
