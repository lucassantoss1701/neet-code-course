use std::rc::Rc;
use std::cell::RefCell;

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Node<T> {
    fn new(val: i64) -> Self{
        Node {
            value: val,
            next: None
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList{head: None, tail: None}
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
        let mut curr = self.head.clone();

        while i < index {
            if let Some(ref current_node) = curr {
                curr = current_node.borrow().next.clone();
                i += 1;
            } else {
                return;
            }
        }

        if let Some(ref curr_node) = curr {
            if let Some(ref next_node) = curr_node.borrow().next {
                if Rc::ptr_eq(next_node, &self.tail.clone().unwrap()) {
                    self.tail = Some(curr_node.clone());
                }

                curr_node.borrow_mut().next = next_node.borrow().next.clone();
            }
        }
    }

    pub fn print(&self) {
        let curr = self.head.clone();
        for curr_node in curr {
            println!("{}", curr_node.borrow().value);
            curr_node.borrow().next.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&curr_node));
        }
        println!();
    }
}