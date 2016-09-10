

struct Rbtree<T: Ord>{
    root: Link<T>,
    count: usize
}

type Link<T> = Box<Node<T>>;

enum Node<T: Ord>{
    Leaf,
    Internal(Internal<T>)
}

struct Internal<T: Ord>{
    color: Color,
    left: Link<T>,
    right: Link<T>,
}

enum InsertionResult{
    Inserted,
    RequireRebalance,
    NoProblem
}

enum Color{
    Red,
    Black
}

impl<T: Ord> Rbtree<T>{
    fn new() -> Self{
        Rbtree{
            root: Node::leaf(),
            count: 0
        }
    }

    fn insert(&mut self,value: T){
        match self.root.insert(value){
            InsertionResult::NoProblem => {},
            InsertionResult::Inserted => {
                if let &mut Internal(ref node) = &mut self.root{
                    node.color = Color::Black;
                }
                else{
                    panic!("why leaf here");
                }
            },
            InsertionResult::RequireRebalance => panic!("zettai ni okoran")
        }
    }
}

impl<T: Ord> Node<T>{
    fn internal(value: T) -> Link<T>{
        Box::new(Node::Internal(Internal{
            color: Color::Red,
            value: value,
            left: Node::leaf(),
            right: Node::leaf()
        }))
    }

    fn leaf() -> Link<T>{
        Box::new(Node::Leaf)
    }

    fn color(&self) -> Color{
        match self{
            &Node::Leaf => Color::Black,
            &Node::Internal(ref node) => node.color
        }
    }

    fn insert(self: &mut Link<T>,value: T) -> InsertionResult{
        match self{
            &mut Node::Leaf => {
                replace(self,Node::internal(value));
                InsertionResult::Inserted
            },
            &mut Node::Internal(ref mut node) => {
                match value.cmp(&node.value){
                    Ordering::Equal => panic!("key juuhuku"),
                    Ordering::Less => {
                        match node.left.insert(value){
                            InsertionResult::NoProblem => InsertionResult::NoProblem,
                            InsertionResult::Inserted => {
                                if self.color() == Color::Red{
                                    InsertionResult::RequireRebalance
                                }
                                else{
                                    InsertionResult::NoProblem
                                }
                            },
                            InsertionResult::RequireRebalance => {
                                let child = &mut node.left;
                                node.color = Color::Red;
                                child.unwrap_internal_mut().color = Color::Black;
                                if child.unwrap_internal_mut().right.color() == Color::Red{
                                    child.left_rotate();
                                }
                                self.right_rotate();
                                InsertionResult::NoProblem
                            }
                        }
                    },
                    Ordering::Greater => {
                        match node.left.insert(value){
                            InsertionResult::NoProblem => InsertionResult::NoProblem,
                            InsertionResult::Inserted => {
                                if self.color() == Color::Red{
                                    InsertionResult::RequireRebalance
                                }
                                else{
                                    InsertionResult::NoProblem
                                }
                            },
                            InsertionResult::RequireRebalance => {
                                let child = &mut node.right;
                                node.color = Color::Red;
                                child.unwrap_internal_mut().color = Color::Black;
                                if child.unwrap_internal_mut().left.color() == Color::Red{
                                    child.right_rotate();
                                }
                                self.left_rotate();
                                InsertionResult::NoProblem
                            }
                        }
                    }
                }
            }
        }
    }

    fn unwrap_internal_mut(self: &mut Link<T>) -> &mut Internal{
        if let &mut Internal(ref node) = self{
            node
        }
        else{
            panic!("not a internal")
        }
    }

    fn left_rotate(self: &mut Link<T>){

    }
}
