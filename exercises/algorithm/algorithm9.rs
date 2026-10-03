// 📖 讲解：algorithm9 - heap（二叉堆）
// 【题目要求】补全二叉堆的 add（插入上滤）、smallest_child_idx（取优先级最高的孩子）、
//             Iterator::next（弹出堆顶并下滤）。comparator 决定堆序：MinHeap 传 a<b，MaxHeap 传 a>b。
// 【考察知识点】用数组实现的二叉堆（下标 1 起，父=i/2、左孩子=2i、右孩子=2i+1）；
//             上滤（sift-up）/下滤（sift-down）两个核心操作；用函数指针自定义比较器实现大小顶堆统一。
// 【对应教材】数据结构与算法——堆与优先队列章节。
// 【解法思路】items[0] 是占位哨兵，真实元素在 1..=count。
//             add：新元素放末尾，一路与父节点比较、不满足堆序就交换（上滤）。
//             smallest_child_idx：在左右孩子中挑"按 comparator 更优先"的那个
//                                  （右孩子存在且比左孩子更优先才选右，否则选左）。
//             next：count==0 返回 None；否则弹出 items[1]，把最后一个元素搬到堆顶再下滤。
//             add 与 next 均为 O(log n)。
//             易错点：1) count 要先减再操作，children_present 的判断依赖减完的 count；
//                    2) 只有 1 个元素时 next 要直接 pop，不能走"替换堆顶+下滤"路径；
//                    3) comparator 的语义是"第一个参数更优先时返回 true"，MinHeap 是 a<b、MaxHeap 是 a>b，
//                       上滤/下滤的比较方向必须一致。

/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 💡 新元素追加到数组末尾（逻辑上成为最后一个叶子），堆大小 +1
        self.count += 1;
        self.items.push(value);
        // 💡 上滤（sift-up）：与父节点比较，违反堆序就交换，直到满足为止
        let mut idx = self.count;
        while idx > 1 {
            let parent = self.parent_idx(idx);
            // 💡 comparator(孩子, 父亲) 为 true 表示孩子"更优先"、应往上走
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break; // 💡 已满足堆序，提前结束
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        // 💡 返回"按 comparator 更优先"的孩子：右孩子存在且比左孩子更优先才选右，否则选左
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 💡 空堆直接返回 None
        if self.count == 0 {
            return None;
        }
        // 💡 先把堆大小减 1（后续 children_present / smallest_child_idx 都以新大小为准）
        self.count -= 1;
        // 💡 只剩一个元素：直接弹出即可，无需下滤
        if self.count == 0 {
            return self.items.pop();
        }
        // 💡 摘走堆顶 items[1]，用最后一个元素补位（mem::replace 取旧根、放新值）
        let last = self.items.pop().unwrap();
        let root = std::mem::replace(&mut self.items[1], last);
        // 💡 下滤（sift-down）：与"更优先"的孩子比较，违反堆序就交换，直到叶子或满足为止
        let mut idx = 1;
        while self.children_present(idx) {
            let child = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child], &self.items[idx]) {
                self.items.swap(idx, child);
                idx = child;
            } else {
                break; // 💡 已满足堆序，提前结束
            }
        }
        Some(root)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
