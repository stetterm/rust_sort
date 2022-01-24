pub mod link {
    pub struct LinkedList<'a, T: Copy> {
        head: Option<Box<Node<'a, T>>>,
        tail: Option<&'a Node<'a, T>>,
        singleton: bool,
    }

    struct Node<'a, T: Copy> {
        value: T,
        next: Option<&'a mut Box<Node<'a, T>>>,
    }

    impl <'a, T: Copy>LinkedList<'a, T> {
        pub fn new() -> LinkedList<'a, T> {
            LinkedList {
                head: None,
                tail: None,
                singleton: false,
            }
        }

        pub fn append(&mut self, value: T) {
            match self.tail {
                None => {
                    self.head = Some(Box::new(Node {
                        value: value,
                        next: None,
                    }));
                    // self.tail = Some();
                    self.singleton = true;
                },
                Some(ref mut t) => {
                    if self.singleton {
                        let mut new_node = Box::new(Node {
                            value: value,
                            next: None,
                        });
                        let tail_value = t.value;
                        
                        // self.head = Some(Box::new(Node {
                        //     value: tail_value,
                        //     next: Some(&mut new_node),
                        // }));
                    }
                }
            }
        }
    }
}