use std::vec::Vec;
use std::iter::Iterator;
use std::iter::DoubleEndedIterator;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;

pub struct Btree<T: Ord> {
    root: Node<T>,
    count: usize,
}

struct NodeData<T: Ord> {
    order: usize,
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

type Node<T> = Box<NodeData<T>>;

enum InsertionResult<T: Ord> {
    NotFull,
    /// .0 -> median
    /// .1 -> less child
    /// .2 -> greater child
    Full(T, Node<T>, Node<T>),
}

struct DebugNode<'a, T: Ord + 'a> {
    node: &'a Node<T>,
    level: usize,
}

impl<T: Ord + Debug> Debug for Btree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Btree({})", self.count).unwrap();
        write!(f,
               "{:?}",
               DebugNode {
                   node: &self.root,
                   level: 0,
               })
    }
}

impl<'a, T: Ord + Debug + 'a> Debug for DebugNode<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let indent = "+-".chars().cycle().take(self.level * 2).collect::<String>();
        write!(f, "{}{:?}", indent, self.node.keys).unwrap();
        let mut result = Ok(());
        for child in self.node.children.iter() {
            result = write!(f,
                            "\n{:?}",
                            DebugNode {
                                node: &child,
                                level: self.level + 1,
                            });
        }
        result
    }
}

pub struct Iter<'a, T: Ord + 'a> {
    tree: &'a Btree<T>,
    refs: Vec<*const T>,
    index: usize,
    index_back: usize,
}

impl<'a, T: Ord + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.index_back {
            None
        } else {
            let ret = unsafe { &*self.refs[self.index] };
            self.index += 1;
            Some(ret)
        }
    }
}

impl<'a, T: Ord + 'a> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.index_back {
            None
        } else {
            self.index_back -= 1;
            let ret = unsafe { &*self.refs[self.index_back] };
            Some(ret)
        }
    }
}

impl<T: Ord> Btree<T> {
    pub fn new() -> Self {
        Btree {
            root: NodeData::new(3),
            count: 0,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        let mut refs = vec![];
        self.root.iterate(&mut |key| refs.push(key as *const T));
        Iter {
            tree: self,
            refs: refs,
            index: 0,
            index_back: self.count,
        }
    }

    pub fn insert(&mut self, value: T) {
        self.count += 1;
        match self.root.insert(value) {
            InsertionResult::NotFull => return,
            InsertionResult::Full(median, left, right) => {
                self.root = NodeData::with_children(self.root.order,
                                                    vec![median],
                                                    vec![left, right]);
            }
        }
    }
}

impl<T: Ord> NodeData<T> {
    fn new(order: usize) -> Node<T> {
        NodeData::with_keys(order, vec![])
    }

    fn with_keys(order: usize, keys: Vec<T>) -> Node<T> {
        NodeData::with_children(order, keys, vec![])
    }

    fn with_children(order: usize, keys: Vec<T>, children: Vec<Node<T>>) -> Node<T> {
        Box::new(NodeData {
            order: order,
            keys: keys,
            children: children,
        })
    }

    fn insert(&mut self, value: T) -> InsertionResult<T> {
        if self.is_leaf() {
            if !self.is_full() {
                self.keys.push(value);
                self.keys.sort();
                InsertionResult::NotFull
            } else {
                self.keys.push(value);
                self.keys.sort();
                self.split()
            }
        } else {
            for i in 0..self.keys.len() {
                match value.cmp(&self.keys[i]) {
                    Ordering::Equal => panic!("same key disallowed"),
                    Ordering::Less => {
                        match self.children[i].insert(value) {
                            InsertionResult::NotFull => return InsertionResult::NotFull,
                            InsertionResult::Full(median, left, right) => {
                                self.keys.insert(i, median);
                                self.children[i] = left;
                                self.children.insert(i + 1, right);
                                if self.keys.len() <= self.order * 2 {
                                    return InsertionResult::NotFull;
                                } else {
                                    return self.split();
                                }
                            }
                        }
                    }
                    Ordering::Greater => {}
                }
            }
            match self.children[self.keys.len()].insert(value) {
                InsertionResult::NotFull => InsertionResult::NotFull,
                InsertionResult::Full(median, left, right) => {
                    self.keys.push(median);
                    self.children[self.keys.len() - 1] = left;
                    self.children.push(right);
                    if self.keys.len() <= self.order * 2 {
                        InsertionResult::NotFull
                    } else {
                        self.split()
                    }
                }
            }
        }
    }

    fn split(&mut self) -> InsertionResult<T> {
        let right = self.keys.split_off(self.order + 1);
        let median = self.keys.pop().unwrap();
        let left = self.keys.split_off(0);
        let right_children;
        let left_children;
        if self.is_leaf() {
            right_children = vec![];
            left_children = vec![];
        } else {
            right_children = self.children.split_off(self.order + 1);
            left_children = self.children.split_off(0);
            assert!(left_children.len() == self.order + 1);
            assert!(right_children.len() == self.order + 1);
        }
        assert!(left.len() == self.order);
        assert!(right.len() == self.order);
        assert!(self.keys.is_empty());
        assert!(self.children.is_empty());
        InsertionResult::Full(median,
                              NodeData::with_children(self.order, left, left_children),
                              NodeData::with_children(self.order, right, right_children))
    }

    fn is_full(&self) -> bool {
        self.keys.len() == self.order * 2
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn iterate<F: FnMut(&T)>(&self, f: &mut F) {
        for i in 0..self.keys.len() {
            if !self.is_leaf() {
                self.children[i].iterate(f);
            }
            f(&self.keys[i]);
        }
        if !self.is_leaf() {
            self.children[self.keys.len()].iterate(f);
        }
    }
}
