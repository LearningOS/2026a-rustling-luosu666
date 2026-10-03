// 📖 讲解：threads3.rs —— mpsc 多生产者通道 + 线程 JoinHandle 的传递
// 【题目要求】Queue 的前半和后半分别由两个子线程通过 mpsc 通道发给主线程，主线程统计收到的总数并断言等于 10。原代码有两处问题：send_tx 里两个 move 闭包都要占有同一个 tx（tx 会被移走两次，编译不过）；而且 send_tx 丢掉了两个 JoinHandle，主线程无法确认发送线程全部结束。
// 【考察知识点】mpsc 通道：Sender 可 clone（多生产者），通道在所有 Sender 被丢弃后自动关闭，接收端 for 循环随之结束；thread::spawn 返回的 JoinHandle 必须保存并 join，函数签名可以改成返回 Vec<JoinHandle<()>>。
// 【对应教材】Rust Book 第 16 章 16.2 "Using Message Passing to Transfer Data between Threads"（clone Sender 创建多生产者的例子）；join 见 16.1。
// 【解法思路】(1) 在 send_tx 里 tx.clone() 出第二份发送端，两个线程各拿一个 Sender（mpsc = multi-producer）。(2) 把 send_tx 的返回类型从 () 改成 Vec<thread::JoinHandle<()>>，用 vec![spawn(...), spawn(...)] 收集两个句柄并返回。(3) main 里保存返回的句柄，在收完消息后逐个 join，确保全部发送线程都已收尾再打印总数。

// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    // 💡 返回两个发送线程的 JoinHandle，让 main 能 join 它们（原来返回 () 会把句柄直接丢掉）
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx1 = tx.clone(); // 💡 mpsc 是多生产者通道：两个 move 闭包不能同时占有同一个 tx，必须先 clone 出第二份 Sender

    vec![
        thread::spawn(move || {
            for val in &qc1.first_half {
                println!("sending {:?}", val);
                tx.send(*val).unwrap(); // 💡 线程 1 用原始 tx 发送前半部分
                thread::sleep(Duration::from_secs(1));
            }
        }), // 💡 换成 vec![] 收集两个线程句柄（也可以用 vec.push 的写法）
        thread::spawn(move || {
            for val in &qc2.second_half {
                println!("sending {:?}", val);
                tx1.send(*val).unwrap(); // 💡 线程 2 用克隆出来的 tx1 发送后半部分
                thread::sleep(Duration::from_secs(1));
            }
        }),
    ]
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let handles = send_tx(queue, tx); // 💡 接住返回的 JoinHandle 列表

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }
    // 💡 上面这个循环会在所有 Sender（两个子线程里的 tx/tx1）被 drop、通道关闭后自然结束

    for handle in handles {
        handle.join().unwrap(); // 💡 等两个发送线程彻底结束，保证所有消息都已发出并被统计
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
