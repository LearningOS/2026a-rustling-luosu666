// 📖 讲解：algorithm2 - doubly linked list reverse（双链表反转）
// 【题目要求】在不借助额外链表的前提下，原地反转一个用裸指针 NonNull 实现的双向链表。
// 【考察知识点】链表三指针迭代反转（prev/cur/next）；NonNull 裸指针的 unsafe 操作；
//             反转后头尾指针（start/end）也要交换。
// 【对应教材】数据结构与算法——链表章节（经典 reverse 算法）。
// 【解法思路】经典三指针法：从旧头开始逐个节点把 next 反指向 prev，同时把 prev 反指向（原来的 next），
//             直到走完。循环结束后 prev 恰好停在旧尾（新头）。
//             最后 start/end 互换：新头 = 旧尾 = prev，新尾 = 旧头。
//             时间 O(n)，空间 O(1)。
//             易错点：1) 循环里必须先保存 current 的 next 再改写，否则断链后无法继续；
//                    2) node.prev 应指向"反转前的下一个节点"（即更新后的 current）；
//                    3) 别忘了交换 self.start 和 self.end，否则 get(0) 从旧头开始遍历会得到空结果。

/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn reverse(&mut self){
		// 三指针迭代反转：prev 记录已反转部分的头，current 是当前待处理节点
		let mut current = self.start;
		let mut prev: Option<NonNull<Node<T>>> = None;
		while let Some(cur) = current {
			unsafe {
				// 💡 1. 先保存下一个节点（改 next 指针之前必须保存，否则断链）
				current = (*cur.as_ptr()).next;
				// 💡 2. 把 next 反过来指向前一个节点
				(*cur.as_ptr()).next = prev;
				// 💡 3. prev 反过来指向"原来的下一个节点"（此时 current 已更新为它）
				(*cur.as_ptr()).prev = current;
			}
			// 💡 4. 当前节点处理完毕，成为已反转部分的新头
			prev = Some(cur);
		}
		// 💡 5. 循环结束后 prev 停在旧尾（新头）；旧头变成新尾，交换 start/end
		let old_start = self.start;
		self.start = prev;
		self.end = old_start;
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}
