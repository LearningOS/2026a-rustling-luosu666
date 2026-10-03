// 📖 讲解：algorithm4 - binary search tree（二叉搜索树）
// 【题目要求】实现 BST 的 insert（插入）与 search（查找）：按 Ord 比较递归放到左/右子树；
//             查找返回 bool；重复值不重复插入（测试要求插入两次 1 后根节点无子节点）。
// 【考察知识点】二叉搜索树的性质（左<根<右）；Option<Box<TreeNode>> 递归结构上的插入/查找；
//             std::cmp::Ordering 与 match。
// 【对应教材】数据结构与算法——二叉搜索树章节。
// 【解法思路】插入：树空则新建根；否则在节点上递归——小于当前值走左子树，大于走右子树，
//             子树为空就挂上新节点，相等则直接忽略（去重）。
//             查找：迭代地从根出发，等于返回 true，小于走左、大于走右，走到空返回 false。
//             两个操作平均 O(log n)（随机数据），最坏退化为 O(n)（有序插入成链）。
//             易错点：1) Ordering::Equal 分支必须"什么都不做"来去重，否则重复插入测试失败；
//                    2) 查找别写成递归比较 self.root 一层就返回。

/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 💡 树为空：直接创建根节点
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        // 💡 否则委托给根节点递归插入
        self.root.as_mut().unwrap().insert(value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // 💡 迭代查找：从根出发，利用 BST 性质每次只需看一棵子树
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            match node.value.cmp(&value) {
                Ordering::Equal => return true,     // 💡 找到了
                Ordering::Less => current = node.right.as_ref(), // 💡 目标更大 → 走右子树
                Ordering::Greater => current = node.left.as_ref(), // 💡 目标更小 → 走左子树
            }
        }
        false // 💡 走到空节点，说明不存在
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // 💡 相等直接返回：重复值不插入（保证 test_insert_duplicate 通过）
        if value == self.value {
            return;
        }
        // 💡 小于当前值放左边，大于放右边（借用可变引用选择子树，避免借用冲突）
        let child = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };
        match child {
            Some(node) => node.insert(value),                       // 💡 子树存在 → 递归下去
            None => *child = Some(Box::new(TreeNode::new(value))),  // 💡 子树为空 → 挂上新节点
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();


        assert_eq!(bst.search(1), false);


        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);


        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();


        bst.insert(1);
        bst.insert(1);


        assert_eq!(bst.search(1), true);


        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
