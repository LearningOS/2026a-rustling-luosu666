// 📖 讲解：algorithm1 - single linked list merge（有序单链表归并）
// 【题目要求】把两个升序单链表归并成一个升序单链表，通过全部 get(i) 断言。
// 【考察知识点】归并思想（merge，归并排序的核心步骤）；NonNull 裸指针链表的遍历取值；
//             泛型方法上加 where T: PartialOrd 约束。
// 【对应教材】数据结构与算法——链表 + 归并排序章节。
// 【解法思路】链表节点值在堆上由裸指针管理，直接改指针拼接较繁琐（还要处理 end 指针），
//             这里采用清晰等价的做法：
//             1) 顺着 next 指针把两个链表的值搬出来（ptr::read 移动语义，原节点内存泄漏但不影响正确性）；
//             2) 用"双路归并"（peekable 比较两路队头）合成一个有序序列；
//             3) 依次 add 到新链表（add 是尾插，顺序即链表顺序）。
//             时间 O(n+m)，空间 O(n+m)。
//             易错点：1) merge 需要比较，必须给方法补 T: PartialOrd 约束，否则编译不过；
//                    2) <= 保证稳定性（相等时优先取 list_a 的元素）；
//                    3) 一条链先耗尽后要把另一条链剩余元素全部接上，否则丢数据。

/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
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
	// 💡 归并需要比较元素大小，给泛型方法补上 T: PartialOrd 约束
	pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
	where
		T: PartialOrd,
	{
		// 步骤 1：顺着 next 指针把两个链表的值依次搬出来
		let read_all = |list: &LinkedList<T>| -> Vec<T> {
			let mut vals = Vec::new();
			let mut node = list.start;
			while let Some(ptr) = node {
				unsafe {
					// 💡 ptr::read 按移动语义取出值（LinkedList 没有 Drop，
					//    原节点内存被泄漏，但不会重复释放、不影响正确性）
					vals.push(std::ptr::read(&ptr.as_ref().val));
					node = ptr.as_ref().next;
				}
			}
			vals
		};
		let a = read_all(&list_a);
		let b = read_all(&list_b);

		// 步骤 2：双路归并 —— 每次取两路中较小的队头（<= 保证稳定性）
		let mut result = LinkedList::new();
		let mut a = a.into_iter().peekable();
		let mut b = b.into_iter().peekable();
		loop {
			let take_a = match (a.peek(), b.peek()) {
				(Some(x), Some(y)) => x <= y, // 💡 谁小取谁；相等时优先 a 保持稳定
				(Some(_), None) => true,      // 💡 b 已取完，只能取 a
				(None, Some(_)) => false,     // 💡 a 已取完，只能取 b
				(None, None) => break,        // 💡 两路都空，归并结束
			};
			if take_a {
				result.add(a.next().unwrap());
			} else {
				result.add(b.next().unwrap());
			}
		}
		result
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
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}
