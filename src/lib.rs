use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use compare::{Compare, natural};
use std::cmp::Ordering::{Less, Equal, Greater};

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
    count: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { 
            head: None,
            count: 0, 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count > 0
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.count = self.count + 1;
    }

    pub fn count(&self) -> &usize {
        &self.count
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                self.count = self.count - 1;
                Some(node.value)
            },
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

//Currently designed to store a small number of elements only
struct BinaryTreeNode<T> {
    value: T,
    count: usize,
    right: BinaryTreeLink<T>,
    left: BinaryTreeLink<T>,
}

impl<T: PartialOrd + Ord + Clone> BinaryTree<T> {
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
            Some(bst_node) => BinaryTree::sub_min(&self.head, &bst_node.value),
        }
    }

    fn sub_min<'a>(head: &'a BinaryTreeLink<T>, min: &'a T) -> Result<&'a T, NullError> {
        let left_min: &T;
        let right_min: &T;
        
        match &head { 
            None => Ok(&min),
            Some(bst_node) => {
                if &bst_node.value <= min { 
                    left_min = BinaryTree::sub_min(&bst_node.left, &bst_node.value).unwrap();
                    right_min = BinaryTree::sub_min(&bst_node.right, &bst_node.value).unwrap();

                    Ok(BinaryTree::lesser_node(left_min, right_min))
                }
                else { Ok(&min) }
            },
        }
    }

    //Iterator workaround
    //Creates linked list with references to binary search tree ordered from min to max
    //To achieve this, Binary tree traversal is done in Reverse order 
    //Because LinkedList push implementation adds new nodes atop, not at the end of List
    pub fn flatten(&self) -> LinkedList<T> {
        let mut flattened_bst = LinkedList::new();

        BinaryTree::sub_flatten(&self.head, &mut flattened_bst);
        flattened_bst
    }

    fn sub_flatten(start: &BinaryTreeLink<T>, list: &mut LinkedList<T>) {
        match start {
            None => {},
            Some(node) => {
                BinaryTree::sub_flatten(&node.right, list);
                list.push(node.value.clone());
                BinaryTree::sub_flatten(&node.left, list);
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.head {
            None => true,
            Some(node) => false,
        }
    }

    //returns NullError if value is missing from Binary Tree
    //or a reference to a value if it was found
    pub fn find<'a>(&'a self, value: &'a T) -> Result<&'a T, NullError> {
        match &self.head {
            None => Err(NullError{}),
            Some(node) => BinaryTree::sub_find(&node, value),
        }
    }

    fn sub_find<'a>(start: &'a BinaryTreeNode<T>, value: &'a T) -> Result<&'a T, NullError> {
        let cmp = natural();
        
        match cmp.compare(&start.value, value) {
            Less => {
                match &start.right {
                    None => return Err(NullError{}),
                    Some(node) => BinaryTree::sub_find(node, value),
                }
            },
            Equal => Ok(&start.value),
            Greater => {
                match &start.left {
                    None => return Err(NullError{}),
                    Some(node) => BinaryTree::sub_find(node, value),
                }
            },
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

    fn lesser_node<'a>(node: &'a T, other_node: &'a T) -> &'a T {
        let cmp = natural();

        match cmp.compare(node, other_node) {
            Less => node,
            Equal => node,
            Greater => other_node,
        } 
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
    fn list_node_count() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32); list.push(32); list.push(32);

        assert_eq!(list.count(), &3);
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
            Err(err) => panic!("Min search not working, {}", err),
        }
    }

    #[test]
    fn bst_iter() {
        let mut bst = BinaryTree::new();
        bst.push(5); bst.push(6); bst.push(17); bst.push(10); bst.push(2);

        let flat_bst = bst.flatten();
        let mut flat_bst_iter = flat_bst.iter();

        assert_eq!(flat_bst_iter.next(), Some(&2));
        assert_eq!(flat_bst_iter.next(), Some(&5));
        assert_eq!(flat_bst_iter.next(), Some(&6));
        assert_eq!(flat_bst_iter.next(), Some(&10));
        assert_eq!(flat_bst_iter.next(), Some(&17));
    }

    #[test]
    fn find_node_in_bst() {
        let mut bst = BinaryTree::new();
        bst.push(5); bst.push(4); bst.push(18);

        let node = bst.find(&18);

        match node {
            Err(err) => panic!("Existing node was not found!, {}", err),
            Ok(node) => assert_eq!(node, &18),
        }
    }
}