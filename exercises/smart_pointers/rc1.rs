// 📖 讲解：rc1.rs —— Rc 引用计数与多重所有权
// 【题目要求】用"太阳系"模型表达"多个行星共同拥有同一个太阳"这一概念：所有行星都持有同一个 Sun，让代码编译并通过两个引用计数断言（中途 count == 9，最后 count == 1）。
// 【考察知识点】Rc<T>（Reference Counted）引用计数智能指针：Rc::new 创建、Rc::clone 增加引用计数（只复制指针不复制数据）、Rc::strong_count 查看计数、drop 手动释放一个所有者。注意 Rc 只能用于单线程。
// 【对应教材】Rust Book 第 15 章（15.3 "Shared-State Concurrency"之前的 "Rc<T>, the Reference Counted Smart Pointer" 一节）。
// 【解法思路】题目里有三类坑：(1) Saturn/Uranus/Neptune 错写成 Rc::new(Sun {}) —— 这会造出三个新太阳，计数对不上；必须改成 Rc::clone(&sun)，与前面的行星共享同一个太阳。(2) 每颗行星 drop 一次，计数减 1。(3) 最后 sun 的计数要从 4 降到 1，还差 earth、venus、mercury 三颗没释放，逐个 drop 即可。Rc::clone 与 Rc::strong_count 都接收 &Rc<T>。

// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun)); // 💡 不能 Rc::new(Sun {})！用 Rc::clone(&sun) 共享同一个太阳，计数才会 +1
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun)); // 💡 同上：克隆的是 Rc 指针，不是 Sun 数据
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun)); // 💡 同上
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth); // 💡 后面三个 TODO：还剩 earth/venus/mercury 三颗行星持有太阳，逐个 drop 让计数降到 1
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    drop(venus); // 💡 释放第二颗
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury); // 💡 释放最后一颗，现在只剩 main 里的 sun 本身这一个所有者
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
