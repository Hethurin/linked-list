pub mod linked_list {
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
    }
}

#[cfg(test)]
mod tests {
    use crate::linked_list::LinkedList;

    #[test]
    fn push_value_to_list() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.push(32);

        assert_eq!(list.pop(), Some(32));
    }
}
