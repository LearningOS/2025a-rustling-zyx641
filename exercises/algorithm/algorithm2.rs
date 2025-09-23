/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

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
}

impl<T:Clone> LinkedList<T> {
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

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 合并两个有序链表
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Ord,  // 要求元素可比较大小
    {
        let mut result = LinkedList::new();
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;
        
        // 遍历两个链表，比较并添加较小的元素
        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            let a_node = unsafe { a_ptr.as_ref() };
            let b_node = unsafe { b_ptr.as_ref() };
            
            if a_node.val <= b_node.val {
                // 将a_node的值添加到结果链表
                result.add(a_node.val.clone());
                // 移动a链表指针
                a_current = a_node.next;
            } else {
                // 将b_node的值添加到结果链表
                result.add(b_node.val.clone());
                // 移动b链表指针
                b_current = b_node.next;
            }
        }
        
        // 添加剩余的元素
        while let Some(a_ptr) = a_current {
            let a_node = unsafe { a_ptr.as_ref() };
            result.add(a_node.val.clone());
            a_current = a_node.next;
        }
        
        while let Some(b_ptr) = b_current {
            let b_node = unsafe { b_ptr.as_ref() };
            result.add(b_node.val.clone());
            b_current = b_node.next;
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

// 为需要的类型实现Clone，以便在合并时复制值
impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            val: self.val.clone(),
            next: None, // 克隆时不复制next指针，只复制值
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
        let list_c = LinkedList::<i32>::merge(list_a,list_b);
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
        let list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
}

fn main() {
    // 创建两个有序链表
    let mut list_a = LinkedList::<i32>::new();
    let mut list_b = LinkedList::<i32>::new();
    
    // 向链表a添加元素
    list_a.add(1);
    list_a.add(3);
    list_a.add(5);
    list_a.add(7);
    
    // 向链表b添加元素
    list_b.add(2);
    list_b.add(4);
    list_b.add(6);
    list_b.add(8);
    
    println!("链表A: {}", list_a);
    println!("链表B: {}", list_b);
    
    // 合并两个链表
    let merged_list = LinkedList::<i32>::merge(list_a, list_b);
    
    println!("合并后的链表: {}", merged_list);
    println!("合并后链表长度: {}", merged_list.length);
    
    // 验证合并结果
    for i in 0..8 {
        if let Some(val) = merged_list.get(i) {
            println!("索引{}处的元素: {}", i, val);
        }
    }
}