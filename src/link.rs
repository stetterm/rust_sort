macro_rules! link {
    () => { LinkedList::new() };
    ($($elem : expr),+ $(,)*) => {{
        let mut new = link![];
        $( new.append($elem); )*
        new
    }};
}

pub mod link {
    use std::cell::RefCell;
    use std::iter::FromIterator;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct LinkedList<T: Copy> {
        head: Option<Rc<RefCell<Node<T>>>>,
        tail: Option<Rc<RefCell<Node<T>>>>,
        size: i32,
    }

    #[derive(Debug)]
    struct Node<T: Copy> {
        pub value: T,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl <T: Copy>FromIterator<T> for LinkedList<T> {
        fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
            let mut new_list: LinkedList<T> = LinkedList::new();
            for i in iter {
                new_list.append(i);
            }
            new_list
        }
    }

    impl <T: Copy>Iterator for LinkedList<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.pop()
        }
    }

    pub type Result<T> = std::result::Result<T, IndexError>;

    #[derive(Debug, Clone)]
    pub struct IndexError;

    impl <T: Copy>LinkedList<T> {
        pub fn new() -> LinkedList<T> {
            LinkedList {
                head: None,
                tail: None,
                size: 0,
            }
        }

        pub fn add(&mut self, value: T, index: usize) {
            if index >= self.size as usize {
                self.append(value);
                return;
            } else if index == 0 {
                self.push(value);
                return;
            }
            let mut prev = Rc::clone(self.head.as_mut().unwrap());
            let mut cur = Rc::clone((*self.head.as_mut().unwrap()).borrow_mut().next.as_mut().unwrap());
            for _ in 1..index {
                prev = Rc::clone(&cur);
                cur = Rc::clone(&prev.borrow_mut().next.as_mut().unwrap());
            }
            let new_node = Rc::new(RefCell::new(Node {
                value: value,
                next: Some(cur),
            }));
            prev.borrow_mut().next = Some(Rc::clone(&new_node));
            self.size += 1;
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
            self.size += 1;
        }

        pub fn get(&self, index: usize) -> Result<T> {
            if index >= self.size as usize {
                match &self.head {
                    Some(val) => return Ok(val.borrow().value),
                    None => return Err(IndexError),
                };
            }
            let mut cur = Rc::clone(self.head.as_ref().unwrap());
            for _ in 1..index+1 {
                let temp = Rc::clone(&cur);
                cur = Rc::clone(&temp.borrow().next.as_ref().unwrap());
            }
            return Ok(cur.borrow().value);
        }

        pub fn len(&self) -> usize {
            return self.size as usize;
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
                    self.size -= 1;
                    Some(val)
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
            self.size += 1;
        }

        pub fn remove(&mut self, index: usize) -> Result<T> {
            if index >= self.size as usize {
                return Err(IndexError);
            }
            let mut prev = Rc::clone(&self.head.as_mut().unwrap());
            let mut cur = Rc::clone(&self.head.as_mut().unwrap().borrow_mut().next.as_mut().unwrap());
            for _ in 1..index {
                prev = Rc::clone(&cur);
                cur = Rc::clone(&prev.borrow_mut().next.as_mut().unwrap());
            }
            let ret_val = cur.borrow().value;
            prev.borrow_mut().next = match &cur.borrow().next {
                None => {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                    self.size -= 1;
                    return Ok(ret_val);
                },
                Some(val) => {
                    Some(Rc::clone(val))
                },
            };
            self.size -= 1;
            return Ok(ret_val);
        }

        pub fn pull(&mut self) -> Result<T> {
            if self.size == 0 {
                return Err(IndexError);
            }
            return self.remove((self.size as usize) - 1);
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
        let mut test_list: LinkedList<i32> = test_list.into_iter().map(|n| {
            n + 6
        }).collect();
        dbg!(&test_list);
        test_list.add(19, 0);
        dbg!(&test_list);
        test_list.add(16, 2);
        test_list.add(95, 6);
        dbg!(&test_list);
        dbg!(&test_list.get(0));
        dbg!(&test_list.len());
        dbg!(&test_list.remove(2));
        dbg!(&test_list);
        dbg!(&test_list.pull());
        dbg!(&test_list);
    }

    #[test]
    pub fn macro_test() {
        let mut x = link![];
        x.append(5);
        assert_eq!(5, x.get(0).unwrap());

        x = link![1, 5, 1, 8];
        assert_eq!(1, x.get(0).unwrap());
        assert_eq!(5, x.get(1).unwrap());
        assert_eq!(1, x.get(2).unwrap());
        assert_eq!(8, x.get(3).unwrap());
    }
}