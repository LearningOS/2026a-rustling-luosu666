// 📖 讲解：algorithm8 - queue（用两个队列实现栈）
// 【题目要求】只允许使用 Queue（FIFO）的 enqueue/dequeue，实现 myStack 的 push/pop/is_empty（LIFO），
//             空栈 pop 必须返回 Err("Stack is empty")（注意不是 Queue 的错误信息）。
// 【考察知识点】两个队列模拟栈的经典技巧；std::mem::swap；错误信息的精确匹配。
// 【对应教材】数据结构与算法——栈与队列章节。
// 【解法思路】约定 q1 始终持有栈的全部内容（栈顶在 q1 的队头），q2 只是入栈时的临时缓冲：
//             push：新元素先入 q2，再把 q1 的旧元素依次搬到 q2 尾部（新元素恰好排在最前），
//                   最后 swap(q1, q2)。这样 q1 队头永远是最后入栈的元素。
//             pop：q1.dequeue() 直接弹栈顶；q1 为空时返回 Err("Stack is empty")。
//             is_empty：q1 是否为空。
//             push O(n)（搬运旧元素）、pop O(1)。
//             易错点：1) 测试断言的错误字符串是 "Stack is empty"，直接透传 q1.dequeue()
//                    会得到 "Queue is empty" 导致失败，必须先判空再出队；
//                    2) 搬运循环 while let Ok(v) = q1.dequeue() 在 q1 空时自动停止。

/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	// 约定：q1 保存栈的全部内容（栈顶 = q1 队头），q2 仅作 push 时的临时缓冲
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 💡 1. 新元素先入 q2
        self.q2.enqueue(elem);
        // 💡 2. 把 q1 的旧元素依次搬到 q2 尾部 → 新元素位于 q2 队头（即栈顶）
        while let Ok(v) = self.q1.dequeue() {
            self.q2.enqueue(v);
        }
        // 💡 3. 交换 q1/q2，恢复"q1 持有栈内容"的约定
        std::mem::swap(&mut self.q1, &mut self.q2);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 💡 空栈必须返回 "Stack is empty"（透传 q1 会得到 "Queue is empty"，测试会挂）
        if self.q1.is_empty() {
            Err("Stack is empty")
        } else {
            // 💡 q1 队头就是栈顶（最后入栈的元素）
            self.q1.dequeue()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}
