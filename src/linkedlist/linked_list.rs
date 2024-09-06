use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: Debug> Node<T> {
    fn new(val: T) -> Self {
        Node {
            value: val,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn insert_end(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn remove(&mut self, index: usize) {
        let mut i = 0;
        let mut curr: Option<Rc<RefCell<Node<T>>>> = self.head.clone();

        while i < index {
            if let Some(current_node) = curr {
                curr = current_node.borrow().next.clone();
                i += 1;
            } else {
                return;
            }
        }

        if let Some(curr_node) = curr {
            if let Some(ref next_node) = curr_node.borrow().next {
                if Rc::ptr_eq(next_node, &self.tail.clone().unwrap()) {
                    self.tail = Some(curr_node.clone());
                }

                curr_node.borrow_mut().next = next_node.borrow().next.clone();
            }
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}
