use std::cmp::PartialOrd;



#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,    
}

// Homework #3 iterator struct
pub struct IntoIter<T>(BST<T>);

impl <T: Copy> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T: Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // self is a mututable reference
        if let Some(node) = self.0.root.take() {
            let v = node.elem;
            self.0.root = node.right;
            Some(v)
        } else {
            None
        }
    }
}
// This constraint basically says the data inside the reference will live as long
// as the reference
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl <'a, T: Copy> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        // Take root as_ref 
       let node = self.root.as_ref().map(|root| &**root); 
        Iter {
            next: node
        }
    }
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|right| &**right);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl <'a, T: Copy> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        // Take root as_ref 
       let node = self.root.as_mut().map(|root| &mut **root); 
        IterMut {
            next: node
        }
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|right| &mut **right);
            &mut node.elem
        })
    }
}

trait Set<T> {
    fn insert(&mut self, e: T) -> bool;
    fn search(&self, e: T) -> bool;
    fn length(&self) -> i32;
}

impl <T: PartialOrd> Set<T> for Link<T> {
    fn insert(&mut self, n: T) -> bool {
        // if empty set link to a new Node
        match *self {
            None  => {
                let new_node = Box::new(
                    Node::new(n)
                );
                *self = Some(new_node);
                true      
            },
            Some(ref mut node) => {
                // if less than current node insert to left 
                if n < node.elem {
                    true && Link::insert(&mut node.left, n)
                // if greater than current node insert to left         
                } else if n > node.elem {
                    true && Link::insert(&mut node.right, n)
                // Return false since elem == n
                } else {
                    return false
                }
            }
        }
    }

    fn search(&self, n: T) -> bool {
        match *self {
            None => {
                false     
            },
            Some(ref node) => {
                // if less than current node insert to left 
                if n < node.elem {
                    false || Link::search(&node.left, n)
                // if greater than current node insert to left         
                } else if n > node.elem {
                    false || Link::search(&node.right, n)
                // Return false since elem == n
                } else {
                    return true
                }
            }
        }
    }

    fn length(&self) -> i32 {
        match *self {
            None => {
                0     
            },
            Some(ref node) => {
                let mut child_count = 0; 
                if let Some(_) = node.left {
                    child_count = Link::length(&node.left);
                }
                if let Some(_) = node.right {
                    child_count = Link::length(&node.right);
                }
                1 + child_count
            }
        }
    }
}

impl <T> Node<T> {
    pub fn new(n : T) -> Self {
        Node { elem: n, left: None, right: None }
    }
}

impl <T: PartialOrd> BST<T> {

    pub fn new() -> Self /* Compiler turns into BST */ {
        BST { root: None }
    }

    fn insert(&mut self, n: T) -> bool {
        Link::insert(&mut self.root, n) 
    }

    fn search(&self, n: T) -> bool {
        // TODO searching empty Link = false
        // left if less than node, rioght if great than
        Link::search(&self.root, n) 
    }

    fn length(&self) -> i32 {
        Link::length(&self.root)
    }
}


#[cfg(test)]
mod test {
    use super::BST;
    #[test]
    pub fn no_duplicates() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        assert_eq!(bst.insert(2), false);
    }

    #[test]
    fn one_element() {
        let mut root = BST::new();
        root.insert(1);
        assert_eq!(root.length(), 1);
    }

    #[test]
    fn length_of_tree() {
        let mut root = BST::new();
        root.insert(1);
        root.insert(2);
        root.insert(3);
        root.insert(4);
        root.insert(5);
        root.insert(6);
        root.insert(7);        
        assert_eq!(root.length(), 7);
    }


    #[test]
    fn search_in_tree() {
        let mut root = BST::new();
        root.insert(1);
        root.insert(2);
        root.insert(3);
        root.insert(4);
        root.insert(5);
        root.insert(6);
        root.insert(7);        
        assert_eq!(root.search(5), true);
    }

    #[test]
    fn test_iteration() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        let mut i = 1;
        for el in bst {
            assert_eq!(el, i);
            i += 1;
        }
    }

    #[test]
    fn test_iteration_by_ref() {
        let mut bst = BST::new();
        bst.insert(1);
        bst.insert(2);
        bst.insert(3);
        let mut i = 1;
        for el in &bst {
            assert_eq!(*el, i);
            i += 1;
        }
    }

}
