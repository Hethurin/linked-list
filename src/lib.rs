use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

pub struct NullError;

impl Display for NullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data structure is empty!")
    }
}

impl Debug for NullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Data structure is empty!")
    }
}

impl Error for NullError {}

// ============================================================================

pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }

    }
    
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {next: self.head.as_mut().map(|node| &mut **node) }
    }
} 

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node );
            &node.value
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}

// ============================================================================

pub struct BinaryTree<T> {
    head: BinaryTreeLink<T>, 
}

type BinaryTreeLink<T> = Option<Box<BinaryTreeNode<T>>>;

struct BinaryTreeNode<T> {
    value: T,
    count: usize,
    right: BinaryTreeLink<T>,
    left: BinaryTreeLink<T>,
}

impl<T: PartialOrd + Display> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { head: None }
    }

    pub fn from(tree: BinaryTree<T>) -> Self {
        BinaryTree { head: tree.head }
    }

    pub fn push(&mut self, value: T) {
        match &mut self.head {
            None => {
                self.head.take();
                self.head = Some(BinaryTree::generate_node(value))
            },
            Some(bst_node) => BinaryTree::insert(bst_node, value),
        }
        
    }

    fn insert(node: &mut BinaryTreeNode<T>, value: T) {
        if node.value > value {
            match &mut node.left {
                None => node.left = Some(BinaryTree::generate_node(value)),
                Some(bst_node) => BinaryTree::insert(bst_node, value),
            }
        }
        else if node.value < value {
            match &mut node.right {
                None => node.right = Some(BinaryTree::generate_node(value)),
                Some(bst_node) => BinaryTree::insert(bst_node, value),
            }
        }
        else { node.count += 1; }
    }

    pub fn min(&self) -> Result<&T, NullError> {
        match &self.head {
            None => Err(NullError{}),
            Some(bst_node) => BinaryTree::sub_min(&bst_node), 
        }
    }

    fn sub_min(start: &BinaryTreeNode<T>) -> Result<&T, NullError> {
        match &start.left { 
            None => Ok(&start.value),
            Some(left_bst_node) => BinaryTree::sub_min(left_bst_node),
        }
    }

    fn generate_node(value: T) -> Box<BinaryTreeNode<T>> {
        Box::new(BinaryTreeNode {
            value: value,
            count: 0,
            right: None,
            left: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;
    use crate::BinaryTree;

    #[test]
    fn push_value_to_list() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32);

        assert_eq!(list.pop(), Some(32));
    }

    #[test]
    fn iter_list() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32); list.push(-4);

        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&-4));
        assert_eq!(list_iter.next(), Some(&32));
        assert_eq!(list_iter.next(), None);
    }

    #[test]
    fn mut_iter_list() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32); list.push(-4);

        let mut list_mut_iter = list.iter_mut();
        assert_eq!(list_mut_iter.next(), Some(&mut -4));
        assert_eq!(list_mut_iter.next(), Some(&mut 32));
    }

    #[test]
    fn find_min_value_in_bst() {
        let mut bst = BinaryTree::new();
        bst.push(5); bst.push(6); bst.push(17); bst.push(10); bst.push(2);

        match bst.min() {
            Ok(min) => assert_eq!(min, &2),
            Err(err) => panic!("My Binary Search Tree doesn't work =(, {}", err) 
        }
    }
}