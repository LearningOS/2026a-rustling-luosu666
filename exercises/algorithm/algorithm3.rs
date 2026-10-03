// 📖 讲解：algorithm3 - sort（排序）
// 【题目要求】实现一个通用的 sort 函数，对任意可比较类型的切片原地升序排序（测试只用了 i32，含单元素、逆序等边界）。
// 【考察知识点】泛型函数 + PartialOrd 约束（原签名没有约束无法比较，需要自己补上）；
//             插入排序/冒泡排序等基本排序的实现与原地交换 slice::swap。
// 【对应教材】数据结构与算法——排序章节（插入排序、冒泡排序、堆排序等任选其一）。
// 【解法思路】这里用插入排序：把第 i 个元素向前冒泡到正确位置。
//             时间复杂度 O(n^2)，空间 O(1)，稳定排序；
//             数据量小（测试规模）完全够用。也可以用快排/归并/堆排达到 O(n log n)。
//             易错点：1) 必须给 T 加 PartialOrd 约束否则 array[j-1] > array[j] 无法编译；
//                    2) 内层 while 要先判断 j > 0 再取 array[j-1]，避免下标越界（usize 下溢 panic）。

/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    // 💡 泛型参数必须加 PartialOrd 约束，否则元素之间无法用 > 比较
    // 插入排序：O(n^2) 时间、O(1) 空间、稳定
    for i in 1..array.len() {
        let mut j = i;
        // 💡 先判 j > 0（防止 usize 下溢），再判前大后小需要交换
        while j > 0 && array[j - 1] > array[j] {
            array.swap(j - 1, j); // 💡 原地交换相邻两个元素
            j -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
	fn test_sort_2() {
		let mut vec = vec![1];
		sort(&mut vec);
		assert_eq!(vec, vec![1]);
	}
	#[test]
	fn test_sort_3() {
		let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
		sort(&mut vec);
		assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
	}
}
