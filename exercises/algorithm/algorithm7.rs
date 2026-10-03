// 📖 讲解：algorithm7 - stack（括号匹配）
// 【题目要求】补全 Stack::pop 和 bracket_match：用栈判断字符串中 ()、[]、{} 三种括号是否完全匹配。
// 【考察知识点】栈的 LIFO 特性与括号匹配这一经典应用；match 表达式匹配字符字面量；
//             Stack 结构体 size 字段与 Vec 数据的同步维护。
// 【对应教材】数据结构与算法——栈章节（括号匹配是栈的标准应用）。
// 【解法思路】pop：data.pop() 取出栈顶，成功时同步 size -= 1，空栈返回 None。
//             bracket_match：扫描每个字符——
//               左括号入栈；右括号必须与栈顶左括号配对（(,)/[,]/{,}），不配对或栈空则失败；
//               其它字符（数字、字母、运算符）直接忽略。
//             扫描结束后栈必须恰好为空（还有剩余左括号也算失败）。
//             时间 O(n)，空间 O(n)。
//             易错点：1) size 与 data 长度要一起维护，否则 is_empty/len 失真；
//                    2) "[[[]]]]]]]]]" 这类右括号多余的情况要在"栈空时遇右括号"立即返回 false；
//                    3) 结尾必须检查栈空，"(2+3)*(3-1" 这种左括号剩余才不会漏判。

/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
    }
impl<T> Stack<T> {
    fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
    }
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// 💡 从 Vec 弹出栈顶；成功时同步维护 size，空栈返回 None
		let item = self.data.pop();
		if item.is_some() {
			self.size -= 1;
		}
		item
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter {
			stack: Vec::new()
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut {
			stack: Vec::new()
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		}
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	// 💡 用本题实现的栈保存"尚未闭合的左括号"
	let mut stack = Stack::new();
	for c in bracket.chars() {
		match c {
			// 💡 左括号一律入栈
			'(' | '[' | '{' => stack.push(c),
			// 💡 右括号必须与栈顶左括号配对：pop 出来检查三种合法组合
			')' | ']' | '}' => {
				match stack.pop() {
					// 💡 栈顶左括号与当前右括号是同一类 → 配对成功，继续
					Some(open) if matches!(
						(open, c),
						('(', ')') | ('[', ']') | ('{', '}')
					) => {}
					// 💡 栈空（右括号多余）或类型不匹配 → 立即失败
					_ => return false,
				}
			}
			// 💡 非括号字符（数字/字母/运算符）不影响匹配，直接跳过
			_ => {}
		}
	}
	// 💡 扫描结束：栈恰好为空（没有剩余未闭合的左括号）才算匹配成功
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}
