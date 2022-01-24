pub mod link {
    use std::rc::Rc;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct LinkedList<T: Copy> {
        head: Option<Rc<RefCell<Node<T>>>>,
        tail: Option<Rc<RefCell<Node<T>>>>,
    }

    #[derive(Debug)]
    struct Node<T: Copy> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl <T: Copy>Iterator for LinkedList<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.pop()
        }
    }

    impl <T: Copy>LinkedList<T> {
        pub fn new() -> LinkedList<T> {
            LinkedList {
                head: None,
                tail: None,
            }
        }

        pub fn append(&mut self, value: T) {
            let new_node = Rc::new(RefCell::new(Node {
                value: value,
                next: None,
            }));
            match self.tail {
                None => {
                    self.head = Some(Rc::clone(&new_node));
                    self.tail = Some(Rc::clone(&new_node));
                },
                Some(ref mut t) => {
                    t.borrow_mut().next = Some(Rc::clone(&new_node));
                    self.tail = Some(new_node);
                },
            }
        }

        pub fn push(&mut self, value: T) {
            match self.head {
                None => {
                    let new_node = Rc::new(RefCell::new(Node {
                        value: value,
                        next: None,
                    }));
                    self.head = Some(Rc::clone(&new_node));
                    self.tail = Some(Rc::clone(&new_node));
                },
                Some(ref h) => {
                    let new_node = Rc::new(RefCell::new(Node {
                        value: value,
                        next: Some(Rc::clone(h)),
                    }));
                    self.head = Some(new_node);
                }
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            match self.head {
                None => None,
                Some(ref h) => {
                    let val = h.borrow().value;
                    let temp = Rc::clone(&self.head.as_ref().unwrap());
                    match temp.borrow_mut().next {
                        None => {
                            self.head = None;
                            self.tail = None;
                        },
                        Some(ref mut n) => self.head = Some(Rc::clone(n)),
                    }
                    Some(val)
                },
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::link::*;

    #[test]
    pub fn append_values() {
        let mut test_list: LinkedList<i32> = LinkedList::new();
        test_list.append(45);
        test_list.append(53);
        test_list.append(96);
        test_list.push(87);
        let test_list: Vec<i32> = test_list.into_iter().map(|n| {
            n + 6
        }).collect();
        dbg!(&test_list);
    }
}