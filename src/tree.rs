pub mod tree {
    #[derive(Debug)]
    pub struct Node<T: PartialOrd + Ord + Copy> {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }

    impl <T: PartialOrd + Ord + Copy>FromIterator<T> for Node<T> {
        fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Node<T> {
            let mut data: Vec<T> = Vec::new();
            for i in iter {
                data.push(i);
            }
            let mut new_bst: Node<T> = Node::new(*data.get(0).unwrap());
            for value in data.iter() {
                new_bst.add_node(*value);
            }
            new_bst
        }
    }

    impl <T: PartialOrd + Ord + Copy>Node<T> {
        pub fn new(root: T) -> Node<T> {
            Node {
                value: root,
                left: None,
                right: None,
            }
        }

        pub fn add_node(&mut self, node: T) {
            if node <= self.value {
                match self.left {
                    Some(ref mut n) => n.add_node(node),
                    None => self.left = Some(Box::new(Node {
                        value: node, 
                        left: None, 
                        right: None
                    })),
                }
            } else {
                match self.right {
                    Some(ref mut n) => n.add_node(node),
                    None => self.right = Some(Box::new(Node {
                        value: node,
                        left: None,
                        right: None,
                    })),
                }
            }
        }

        pub fn get_inorder(&self, list: &mut [T]) {
            let mut count = 0;
            self._inorder(&mut list[..], &mut count);
        }

        fn _inorder(&self, list: &mut [T], count: &mut usize) {
            if let Some(node) = &self.left {
                node._inorder(list, count);
            }
            list[*count] = self.value;
            *count += 1;
            if let Some(node) = &self.right {
                node._inorder(list, count);
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::alg;

    #[test]
    pub fn make_tree() {
        let mut test_tree = tree::Node::new(5);
        test_tree.add_node(7);
        test_tree.add_node(3);
        let mut out: Vec<i32> = vec![0; 3];
        test_tree.get_inorder(&mut out);
        assert_eq!(true, alg::is_sorted(&out[..]));
    }


}