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
}

impl<T> SequencedList<T> {
    fn new() -> SequencedList<T> {
        SequencedList::<T> {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, val: T) {
        self.length += 1;
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(val)));
            self.tail = Some(self.head.as_mut().unwrap().borrow_mut() as *mut Node<T>);
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
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node::<T> {
            value: val,
            next: None,
        }
    }

    fn insert_next(&mut self, next_val: T) -> Option<*mut Self> {
        let next = Some(Box::new(Self::new(next_val)));
        self.next = next;
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

fn main() {
    let mut list = SequencedList::<i32>::new();
    list.push(3);
    list.push(2);
    list.push(4);

    println!("{:?} {:?} {:?} {:?}",
             list.get(0),
             list.get(1),
             list.get(2),
             list.get(3));
}
