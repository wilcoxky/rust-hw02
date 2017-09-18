
#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link
}

// We use ENUM here to ensure it is Empty of a ptr, this ensures Null Pointer optimization
#[derive(Debug)]
enum Link {
    Empty,
    // Box allocations it on the heap
    More(Box<Node>),
}

#[derive(Debug)]
pub struct BST {
    root: Link,    
}

// [ptr] => (Link { elem: left:: Empty, right::empty  })

impl Link {
    fn insert(link: &mut Self, n: i32) -> bool {
        // if empty set link to a new Node
        match *link {
            Link::Empty => {
                let new_node = Box::new(
                    Node::new(n)
                );
                *link = Link::More(new_node);
                true      
            },
            Link::More(ref mut node) => {
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

    fn search(link: &Self, n: i32) -> bool {
        match *link {
            Link::Empty => {
                false     
            },
            Link::More(ref node) => {
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

    fn length(link: &Self) -> i32 {
        match *link {
            Link::Empty => {
                0     
            },
            Link::More(ref node) => {
                let mut child_count = 0; 
                if let Link::More(_) = node.left {
                    child_count = Link::length(&node.left);
                }
                if let Link::More(_) = node.right {
                    child_count = Link::length(&node.right);
                }
                1 + child_count
            }
        }
    }
}

impl Node {
    pub fn new(n : i32) -> Self {
        Node { elem: n, left: Link::Empty, right: Link::Empty }
    }
}
impl BST {

    pub fn new() -> Self /* Compiler turns into BST */ {
        BST { root: Link::Empty }
    }

    fn insert(&mut self, n: i32) -> bool {
        Link::insert(&mut self.root, n) 
    }

    fn search(&self, n: i32) -> bool {
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

}
