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

#[cfg(test)]
mod tests {
    use crate::LinkedList;

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
}
