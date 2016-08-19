use std::borrow::BorrowMut;

struct SequencedList<T> {
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

struct Iter<'a, T: 'a> {
    length: usize,
    head: &'a Link<T>,
    tail: Option<*mut Node<T>>,
}

impl<T> SequencedList<T> {
    fn new() -> SequencedList<T> {
        SequencedList::<T> {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, val: T) {
        self.length += 1;
        if self.head.is_none() {
            let mut node = Box::new(Node::new(val));
            self.tail = Some(node.borrow_mut() as *mut Node<T>);
            self.head = Some(node);
        } else {
            let mut node = Box::new(Node::new(val));
            std::mem::swap(&mut node.next, &mut self.head);
            std::mem::swap(&mut self.head, &mut Some(node));
        }
    }

    fn push_back(&mut self, val: T) {
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

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            None
        } else {
            Some(&self.head.as_ref().unwrap().nth(index).value)
        }
    }

    fn iter(&self) -> Iter<T> {
        Iter::<T>::new(self.length, &self.head, self.tail)
    }

    fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.length = 0;
    }

    fn len(&self) -> usize {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn append(&mut self, other: &mut Self) {
        if let Some(tail) = self.tail {
            self.length += other.length;
            other.length = 0;
            unsafe {
                std::mem::swap(&mut (*tail).next, &mut other.head);
            }
            self.tail = other.tail;
            other.tail = None;
        } else {
            std::mem::swap(&mut self.head, &mut other.head);
            std::mem::swap(&mut self.tail, &mut other.tail);
            std::mem::swap(&mut self.length, &mut other.length);
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
            let reference = std::mem::replace(&mut self.head,
                                              &(**self.head.as_ref().unwrap()).next);
            Some(&(*reference.as_ref().unwrap()).value)
        }
    }
}

impl<'a, T: std::fmt::Debug> DoubleEndedIterator for Iter<'a, T> {
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



fn main() {
    let mut list = SequencedList::<i32>::new();
    let mut list2 = SequencedList::<i32>::new();
    list.push_back(3);
    list.push_front(2);
    list.push_back(4);
    list.push_back(5);
    list.push_back(6);

    println!("{:?} {:?} {:?} {:?} {:?}",
             list.get(0),
             list.get(1),
             list.get(2),
             list.get(3),
             list.get(4));
    println!("length = {}", list.len());
    list.clear();
    println!("cleared. is_empty() = {}", list.is_empty());

    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list2.push_front(2);
    list2.push_front(1);

    list2.append(&mut list);

    println!("{} {}", list.len(), list2.len());

    for v in list2.iter() {
        println!("{}", v);
    }

    let mut iter = list2.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&5), iter.next_back());
    assert_eq!(Some(&4), iter.next_back());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
}
