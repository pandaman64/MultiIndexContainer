use std::borrow::BorrowMut;
use std::mem::swap;
use std::mem::replace;

pub struct SequencedList<T> {
    length: usize,
    head: Link<T>,
    tail: Option<*mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Option<*mut Node<T>>,
}

pub struct Iter<'a, T: 'a> {
    length: usize,
    head: &'a Link<T>,
    tail: Option<*mut Node<T>>,
}

impl<T> SequencedList<T> {
    pub fn new() -> SequencedList<T> {
        SequencedList::<T> {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, val: T) {
        self.length += 1;
        if self.head.is_none() {
            let mut node = Box::new(Node::new(val));
            self.tail = Some(node.borrow_mut() as *mut Node<T>);
            self.head = Some(node);
        } else {
            let mut node = Box::new(Node::new(val));
            swap(&mut node.next, &mut self.head);
            swap(&mut self.head, &mut Some(node));
        }
    }

    pub fn push_back(&mut self, val: T) {
        self.length += 1;
        if self.head.is_none() {
            let mut node = Box::new(Node::new(val));
            self.tail = Some(node.borrow_mut() as *mut Node<T>);
            self.head = Some(node);
        } else {
            unsafe {
                self.tail = (**self.tail.as_ref().unwrap()).insert_next(val);
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            None
        } else {
            Some(&self.head.as_ref().unwrap().nth(index).value)
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::<T>::new(self.length, &self.head, self.tail)
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|b| &(**b).value)
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|&ptr| &(*ptr).value) }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn append(&mut self, other: &mut Self) {
        if let Some(tail) = self.tail {
            self.length += other.length;
            other.length = 0;
            unsafe {
                swap(&mut (*tail).next, &mut other.head);
            }
            self.tail = other.tail;
            other.tail = None;
        } else {
            swap(&mut self.head, &mut other.head);
            swap(&mut self.tail, &mut other.tail);
            swap(&mut self.length, &mut other.length);
        }
    }
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node::<T> {
            value: val,
            next: None,
            prev: None,
        }
    }

    fn insert_next(&mut self, next_val: T) -> Option<*mut Self> {
        let mut next = Box::new(Self::new(next_val));
        next.prev = Some(self as *mut Self);
        self.next = Some(next);
        Some(self.next.as_mut().unwrap().borrow_mut() as *mut Self)
    }

    fn nth(&self, index: usize) -> &Self {
        if index == 0 {
            self
        } else {
            self.next.as_ref().unwrap().nth(index - 1)
        }
    }
}

impl<'a, T> Iter<'a, T> {
    fn new(length: usize, head: &'a Link<T>, tail: Option<*mut Node<T>>) -> Self {
        Iter::<T> {
            length: length,
            head: head,
            tail: tail,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            let reference = replace(&mut self.head, &(**self.head.as_ref().unwrap()).next);
            Some(&(*reference.as_ref().unwrap()).value)
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            let ret = unsafe { Some(&(*(*self.tail.as_ref().unwrap())).value) };
            unsafe {
                self.tail = (**self.tail.as_ref().unwrap()).prev;
            }
            ret
        }
    }
}
