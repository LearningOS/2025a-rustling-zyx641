/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

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
        // 将新元素添加到向量末尾
        self.count += 1;
        
        // 如果向量容量不足，扩展它
        if self.count < self.items.len() {
            self.items[self.count] = value;
        } else {
            self.items.push(value);
        }
        
        // 执行上浮操作，维护堆的性质
        self.sift_up(self.count);
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
        // 检查是否有子节点
        if !self.children_present(idx) {
            return 0; // 没有子节点时返回0（无效索引）
        }
        
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        // 检查是否有右子节点
        if right > self.count {
            return left; // 只有左子节点时返回左子节点索引
        }
        
        // 根据比较器选择"较小"的子节点
        let comparator = self.comparator;
        if comparator(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
    
    // 上浮操作：将指定索引的元素向上移动到正确位置
    fn sift_up(&mut self, mut idx: usize) {
        let comparator = self.comparator;
        
        // 当元素不是根节点且满足比较条件时，继续上浮
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if comparator(&self.items[idx], &self.items[parent]) {
                // 交换元素
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break; // 已经在正确位置
            }
        }
    }
    
    // 下沉操作：将指定索引的元素向下移动到正确位置
    fn sift_down(&mut self, mut idx: usize) {
        let comparator = self.comparator;
        
        // 当元素有子节点时，继续下沉
        while self.children_present(idx) {
            let child = self.smallest_child_idx(idx);
            if comparator(&self.items[child], &self.items[idx]) {
                // 交换元素
                self.items.swap(idx, child);
                idx = child;
            } else {
                break; // 已经在正确位置
            }
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
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // 如果堆为空，返回None
        if self.is_empty() {
            return None;
        }
        
        // 保存堆顶元素（索引1处的元素）
        let result = std::mem::replace(&mut self.items[1], T::default());
        
        // 将最后一个元素移到堆顶
        if self.count > 1 {
            self.items[1] = self.items[self.count].clone();
        }
        
        // 减少计数
        self.count -= 1;
        
        // 如果堆不为空，执行下沉操作
        if !self.is_empty() {
            self.sift_down(1);
        }
        
        // 返回保存的堆顶元素
        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
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