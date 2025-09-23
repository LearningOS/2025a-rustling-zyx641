/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// I AM NOT DONE
#[derive(Debug)]
struct Stack<T> {
	_size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			_size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self._size
	}
	fn len(&self) -> usize {
		self._size
	}
	fn clear(&mut self) {
		self._size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self._size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// 实现pop方法：如果栈不为空，减少大小并从数据向量中弹出最后一个元素
		if !self.is_empty() {
			self._size -= 1;
			self.data.pop()
		} else {
			None
		}
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self._size {
			return None;
		}
		self.data.get(self._size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self._size {
			return None;
		}
		self.data.get_mut(self._size - 1)
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
			self.0._size -= 1;
			self.0.data.pop()
		} else {
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

fn bracket_match(bracket: &str) -> bool {
	// 实现括号匹配函数
	let mut stack = Stack::new();
	
	// 遍历字符串中的每个字符
	for c in bracket.chars() {
		match c {
			// 如果是开括号，压入栈中
			'(' | '[' | '{' => stack.push(c),
			// 如果是闭括号，检查栈顶元素是否匹配
			')' => {
				if stack.pop() != Some('(') {
					return false;
				}
			},
			']' => {
				if stack.pop() != Some('[') {
					return false;
				}
			},
			'}' => {
				if stack.pop() != Some('{') {
					return false;
				}
			},
			// 忽略其他字符
			_ => {}
		}
	}
	
	// 所有括号都匹配的话，栈应该为空
	stack.is_empty()
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s), true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s), false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s), true);
	}
}