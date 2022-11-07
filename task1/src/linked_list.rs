use std::ptr::NonNull;

pub(crate) type Link<T> = NonNull<Node<T>>;

pub(crate) struct Node<T> {
    pub(super) value: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

pub(crate) struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        return LinkedList { head: None, tail: None };
    }

    pub fn push_left(&mut self, value: T) -> Link<T> {
        unsafe {
            let new_node = NonNull::new_unchecked(
                Box::into_raw(Box::new(Node {
                    value,
                    next: self.head,
                    prev: None,
                })));
            if let Some(old_head) = self.head {
                (*old_head.as_ptr()).prev = Some(new_node);
                (*new_node.as_ptr()).next = Some(old_head);
            } else {
                self.tail = Some(new_node);
            }
            self.head = Some(new_node);
            new_node
        }
    }

    pub fn pop_right(&mut self) -> Option<T> {
        unsafe {
            self.tail.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.value;

                self.tail = boxed_node.prev;
                if let Some(new) = self.tail {
                    (*new.as_ptr()).next = None;
                } else {
                    self.head = None;
                }
                result
            })
        }
    }

    pub unsafe fn move_to_left(&mut self, mut node: Link<T>) {
        let node_ref = node.as_mut();

        if let Some(prev) = node_ref.prev {
            (*prev.as_ptr()).next = node_ref.next;
        } else {
            return;
        }

        if let Some(next) = node_ref.next {
            (*next.as_ptr()).prev = node_ref.prev;
        } else {
            self.tail = node_ref.prev;
        }

        node_ref.prev = None;
        node_ref.next = self.head;
        // this .unwrap is actually assert
        // can happen only in empty list
        // but we have at least one node (which is arg)
        (*self.head.unwrap().as_ptr()).prev = Some(node);
        self.head = Some(node);
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_right().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushes_and_pops() {
        let mut result: LinkedList<i32> = LinkedList::new();
        result.push_left(4);
        result.push_left(3);
        result.push_left(2);
        result.push_left(1);
        assert_eq!(result.pop_right().unwrap(), 4);
        assert_eq!(result.pop_right().unwrap(), 3);
        assert_eq!(result.pop_right().unwrap(), 2);
        assert_eq!(result.pop_right().unwrap(), 1);
    }

    #[test]
    fn really_linked_list() {
        let mut result: LinkedList<i32> = LinkedList::new();
        let four = result.push_left(4);
        let three = result.push_left(3);
        let two = result.push_left(2);
        let one = result.push_left(1);

        unsafe {
            let first = result.head.unwrap();
            let second = first.as_ref().next.unwrap();
            let third = second.as_ref().next.unwrap();
            let fourth = third.as_ref().next.unwrap();

            assert_eq!(first, one);
            assert!(first.as_ref().prev.is_none());
            assert_eq!(first.as_ref().next.unwrap(), two);

            assert_eq!(second, two);
            assert_eq!(second.as_ref().prev.unwrap(), one);
            assert_eq!(second.as_ref().next.unwrap(), three);

            assert_eq!(third, three);
            assert_eq!(third.as_ref().prev.unwrap(), two);
            assert_eq!(third.as_ref().next.unwrap(), four);

            assert_eq!(fourth, fourth);
            assert_eq!(fourth.as_ref().prev.unwrap(), three);
            assert!(fourth.as_ref().next.is_none());

            assert_eq!(result.tail.unwrap(), four);
        }
    }

    #[test]
    fn one_move() {
        let mut result: LinkedList<i32> = LinkedList::new();
        result.push_left(4);
        let three = result.push_left(3);
        result.push_left(2);
        result.push_left(1);

        unsafe {
            result.move_to_left(three);
        }
        assert_eq!(result.pop_right().unwrap(), 4);
        assert_eq!(result.pop_right().unwrap(), 2);
        assert_eq!(result.pop_right().unwrap(), 1);
        assert_eq!(result.pop_right().unwrap(), 3);
    }

    #[test]
    fn moves() {
        let mut result: LinkedList<i32> = LinkedList::new();
        let four = result.push_left(4);
        let three = result.push_left(3);
        let two = result.push_left(2);
        let one = result.push_left(1);
        unsafe {
            result.move_to_left(one);
            result.move_to_left(two);
            result.move_to_left(three);
            result.move_to_left(four);
        }
        assert_eq!(result.pop_right().unwrap(), 1);
        assert_eq!(result.pop_right().unwrap(), 2);
        assert_eq!(result.pop_right().unwrap(), 3);
        assert_eq!(result.pop_right().unwrap(), 4);
    }
}