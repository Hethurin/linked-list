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

impl<T: PartialOrd + Ord> BinaryTree<T> {
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

    fn sub_min_with_limit<'a>(head: &'a BinaryTreeLink<T>, min: &'a T, bottom_limit: &T) -> Result<&'a T, NullError> {
        let left_min: &T;
        let right_min: &T;
        
        match &head { 
            None => Ok(&min),
            Some(bst_node) => {
                if &bst_node.value > bottom_limit { 
                    left_min = BinaryTree::sub_min_with_limit(&bst_node.left, &bst_node.value, bottom_limit).unwrap();
                    right_min = BinaryTree::sub_min_with_limit(&bst_node.right, &bst_node.value, bottom_limit).unwrap();

                    Ok(BinaryTree::lesser_node(left_min, right_min))
                }
                else {  BinaryTree::sub_min_with_limit(&bst_node.right, min, bottom_limit) }
            }
        }
    }

    //WIP
    //Creates linked list with references to binary search tree ordered from min to max
    //Iterator workaround
    pub fn flatten(&self) -> LinkedList<&T> {
        LinkedList::new()
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
    fn last_node() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32); list.push(33); list.push(34);

        match list.last {
            None => panic!("Getting last list node is not working"),
            Some(node) => assert_eq!(node.value, &32),
        }
    }

    #[test]
    fn list_merge() {
        let mut legacy:LinkedList<i32> = LinkedList::new();
        let mut hype:LinkedList<i32> = LinkedList::new();

        legacy.push(1); hype.push(2); hype.push(3);
        legacy.merge(hype);

        let mut legacy_iter = legacy.iter();
        assert_eq!(legacy_iter.next(), Some(&1));
        assert_eq!(legacy_iter.next(), Some(&2));
        assert_eq!(legacy.count, &3);
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

    //remove later
    #[test]
    fn find_next_min_value_in_bst() {
        let mut bst = BinaryTree::new();
        bst.push(5); bst.push(6); bst.push(17); bst.push(10); bst.push(2);

        match BinaryTree::sub_min_with_limit(&bst.head, &5, &5) {
            Ok(min) => assert_eq!(min, &6),
            Err(err) => panic!("Next min value search is not working, {}", err),
        }
    }

    #[test]
    fn find_min_value_in_bst_complex() {
        let mut bst = BinaryTree::new();
        bst.push(20); bst.push(15); bst.push(17); bst.push(16); bst.push(19);
        bst.push(9); bst.push(12); bst.push(10); bst.push(6); bst.push(5);
        bst.push(21); bst.push(25); bst.push(23); bst.push(28); bst.push(40);

        match BinaryTree::sub_min_with_limit(&bst.head, &19, &19) {
            Ok(min) => assert_eq!(min, &20),
            Err(err) => panic!("Next min value search is not working, {}", err),
        }
    }
}