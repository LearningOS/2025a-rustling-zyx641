/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*; // 未使用的导入，会产生警告

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
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

impl<T: Clone> LinkedList<T> {
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

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(ptr) => {
                let node_ref = unsafe { ptr.as_ref() };
                match index {
                    0 => Some(&node_ref.val),
                    _ => self.get_ith_node(node_ref.next, index - 1),
                }
            }
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut result = LinkedList::new();
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;

        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            let a_node = unsafe { a_ptr.as_ref() };
            let b_node = unsafe { b_ptr.as_ref() };

            if a_node.val <= b_node.val {
                result.add(a_node.val.clone());
                a_current = a_node.next;
            } else {
                result.add(b_node.val.clone());
                b_current = b_node.next;
            }
        }

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
            Some(ptr) => write!(f, "{}", unsafe { ptr.as_ref() }),
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
            Some(ptr) => write!(f, "{}, {}", self.val, unsafe { ptr.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            val: self.val.clone(),
            next: None,
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
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }

        let list_c = LinkedList::merge(list_a, list_b);
        println!("Merged List is {}", list_c);

        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }

        let list_c = LinkedList::merge(list_a, list_b);
        println!("Merged List is {}", list_c);

        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as i32).unwrap());
        }
    }
}
