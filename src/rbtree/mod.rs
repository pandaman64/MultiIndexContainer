use std::mem::swap;
use std::mem::replace;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Debug)]
enum Node<T: Ord> {
    Leaf,
    Internal(Internal<T>),
}

type Link<T> = Box<Node<T>>;

#[derive(Debug)]
struct Internal<T: Ord> {
    value: T,
    color: Color,
    left: Link<T>,
    right: Link<T>,
}

pub struct Rbtree<T: Ord> {
    root: Link<T>,
    count: usize,
}

enum InsertionResult {
    Inserted,
    RequireRebalance,
    NoProblem,
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Iter<'a, T: Ord + 'a> {
    stack: Vec<&'a Internal<T>>,
}

impl<'a, T: Ord + 'a> Iter<'a, T> {
    fn new(tree: &'a Rbtree<T>) -> Self {
        let mut iter = Iter { stack: vec![] };
        iter.push(&tree.root);
        iter
    }

    fn push(&mut self, mut node: &'a Link<T>) {
        while let &Node::Internal(ref internal) = node as &Node<T> {
            self.stack.push(internal);
            node = &internal.left;
        }
    }
}

impl<'a, T: Ord + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            self.push(&node.right);
            Some(&node.value)
        } else {
            None
        }
    }
}

struct DebugNode<'a, T: Ord + Debug + 'a> {
    node: &'a Internal<T>,
    level: usize,
}

impl<'a, T: Ord + Debug + 'a> Debug for DebugNode<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let indent = "+-".chars().cycle().take(self.level * 2).collect::<String>();
        write!(f, "{}", indent).unwrap();
        f.debug_struct("Node")
            .field("color", &self.node.color)
            .field("value", &self.node.value)
            .finish()
            .unwrap();
        if let &Node::Internal(ref node) = &self.node.left as &Node<T> {
            write!(f,
                   "\n{:?}",
                   DebugNode {
                       node: node,
                       level: self.level + 1,
                   })
                .unwrap();
        }
        if let &Node::Internal(ref node) = &self.node.right as &Node<T> {
            write!(f,
                   "\n{:?}",
                   DebugNode {
                       node: node,
                       level: self.level + 1,
                   })
                .unwrap();
        }
        Ok(())
    }
}

impl<T: Ord + Debug> Debug for Rbtree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Rbtree({})", self.count).unwrap();
        if let &Node::Internal(ref node) = &self.root as &Node<T> {
            write!(f,
                   "{:?}",
                   DebugNode {
                       node: node,
                       level: 0,
                   })
                .unwrap();
        }
        Ok(())
    }
}


impl<T: Ord> Rbtree<T> {
    pub fn new() -> Self {
        Rbtree {
            root: Node::leaf(),
            count: 0,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }

    pub fn insert(&mut self, value: T) {
        self.count += 1;
        match insert(&mut self.root, value) {
            InsertionResult::NoProblem => {}
            InsertionResult::Inserted => {
                self.root.unwrap_internal_mut().color = Color::Black;
            }
            InsertionResult::RequireRebalance => panic!("zettai ni okoran"),
        }
    }
}

impl<T: Ord> Node<T> {
    fn internal(value: T) -> Link<T> {
        Box::new(Node::Internal(Internal {
            color: Color::Red,
            value: value,
            left: Node::leaf(),
            right: Node::leaf(),
        }))
    }

    fn leaf() -> Link<T> {
        Box::new(Node::Leaf)
    }

    fn color(&self) -> Color {
        match self {
            &Node::Leaf => Color::Black,
            &Node::Internal(ref node) => node.color,
        }
    }

    fn unwrap_internal_mut(&mut self) -> &mut Internal<T> {
        if let &mut Node::Internal(ref mut node) = self {
            node
        } else {
            panic!("not a internal")
        }
    }

    fn is_internal(&self) -> bool {
        if let &Node::Internal(_) = self {
            true
        } else {
            false
        }
    }
}

fn insert<T: Ord>(this: &mut Link<T>, value: T) -> InsertionResult {
    let is_left_rotation;
    if this.is_internal() {
        let node = this.unwrap_internal_mut();
        match value.cmp(&node.value) {
            Ordering::Equal => panic!("key juuhuku"),
            Ordering::Less => {
                match insert(&mut node.left, value) {
                    InsertionResult::NoProblem => return InsertionResult::NoProblem,
                    InsertionResult::Inserted => {
                        if node.color == Color::Red {

                            return InsertionResult::RequireRebalance;
                        } else {
                            return InsertionResult::NoProblem;
                        }
                    }
                    InsertionResult::RequireRebalance => {

                        if node.right.color() == Color::Red {
                            node.right.unwrap_internal_mut().color = Color::Black;
                            node.left.unwrap_internal_mut().color = Color::Black;
                            node.color = Color::Red;
                            return InsertionResult::Inserted;
                        }
                        {
                            let child = &mut node.left;
                            if child.unwrap_internal_mut().right.color() == Color::Red {

                                left_rotate(child);
                            }
                        }
                        {

                            let child = &mut node.left;
                            node.color = Color::Red;
                            child.unwrap_internal_mut().color = Color::Black;
                        }
                        is_left_rotation = false;
                    }
                }
            }
            Ordering::Greater => {
                match insert(&mut node.right, value) {
                    InsertionResult::NoProblem => return InsertionResult::NoProblem,
                    InsertionResult::Inserted => {
                        if node.color == Color::Red {

                            return InsertionResult::RequireRebalance;
                        } else {
                            return InsertionResult::NoProblem;
                        }
                    }
                    InsertionResult::RequireRebalance => {

                        if node.left.color() == Color::Red {
                            node.right.unwrap_internal_mut().color = Color::Black;
                            node.left.unwrap_internal_mut().color = Color::Black;
                            node.color = Color::Red;
                            return InsertionResult::Inserted;
                        }
                        {
                            let child = &mut node.right;
                            if child.unwrap_internal_mut().left.color() == Color::Red {

                                right_rotate(child);
                            }
                        }
                        {

                            let child = &mut node.right;
                            node.color = Color::Red;
                            child.unwrap_internal_mut().color = Color::Black;
                        }
                        is_left_rotation = true;
                    }
                }
            }
        }
    } else {
        replace(this, Node::internal(value));
        return InsertionResult::Inserted;
    }


    if is_left_rotation {
        left_rotate(this);
    } else {
        right_rotate(this);
    }
    InsertionResult::NoProblem
}

fn left_rotate<T: Ord>(this: &mut Link<T>) {
    let child = {
        let child = &mut this.unwrap_internal_mut().right;
        let grand_child = replace(&mut child.unwrap_internal_mut().left, Node::leaf());
        &mut replace(child, grand_child)
    };
    swap(this, &mut child.unwrap_internal_mut().left);
    swap(this, child);
}

fn right_rotate<T: Ord>(this: &mut Link<T>) {
    let child = {
        let child = &mut this.unwrap_internal_mut().left;
        let grand_child = replace(&mut child.unwrap_internal_mut().right, Node::leaf());
        &mut replace(child, grand_child)
    };
    swap(this, &mut child.unwrap_internal_mut().right);
    swap(this, child);
}
