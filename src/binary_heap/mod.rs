use std::vec::Vec;

pub struct BinaryHeap<T: Ord> {
    values: Vec<T>,
}

#[derive(Copy,Clone)]
enum TraverseFrom {
    Parent,
    Left,
    Right,
}

pub struct Iter<'a, T: 'a + Ord> {
    heap: &'a BinaryHeap<T>,
    index: Option<usize>,
    flag: TraverseFrom,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.index {
            let ret = Some(&self.heap.values[index]);
            let (index, flag) = next_index(self.heap, index, self.flag);
            self.index = index;
            self.flag = flag;
            ret
        } else {
            None
        }
    }
}

fn is_root_node(index: usize) -> bool {
    index == 0
}

fn is_left_node(index: usize) -> bool {
    index % 2 == 1
}

fn next_index<T: Ord>(heap: &BinaryHeap<T>,
                      index: usize,
                      flag: TraverseFrom)
                      -> (Option<usize>, TraverseFrom) {
    if is_root_node(index) {
        match flag {
            TraverseFrom::Parent => {
                // first next()
                if heap.has_left_child(index) {
                    (Some(left_child_index(index)), TraverseFrom::Parent)
                } else {
                    (None, TraverseFrom::Parent)
                }
            }
            TraverseFrom::Left => {
                if heap.has_right_child(index) {
                    (Some(right_child_index(index)), TraverseFrom::Parent)
                } else {
                    (None, TraverseFrom::Parent)
                }
            }
            TraverseFrom::Right => (None, TraverseFrom::Parent),
        }
    } else {
        match flag {
            TraverseFrom::Parent => {
                if heap.has_left_child(index) {
                    (Some(left_child_index(index)), TraverseFrom::Parent)
                } else if is_left_node(index) {
                    next_index(heap, parent_index(index), TraverseFrom::Left)
                } else {
                    // right node
                    next_index(heap, parent_index(index), TraverseFrom::Right)
                }
            }
            TraverseFrom::Left => {
                if heap.has_right_child(index) {
                    (Some(right_child_index(index)), TraverseFrom::Parent)
                } else if is_left_node(index) {
                    next_index(heap, parent_index(index), TraverseFrom::Left)
                } else {
                    // right node
                    next_index(heap, parent_index(index), TraverseFrom::Right)
                }
            }
            TraverseFrom::Right => {
                if is_left_node(index) {
                    next_index(heap, parent_index(index), TraverseFrom::Left)
                } else {
                    // right node
                    next_index(heap, parent_index(index), TraverseFrom::Right)
                }
            }
        }
    }
}

fn parent_index(index: usize) -> usize {
    if index == 0 {
        panic!("root doesn't have parent")
    } else {
        (index - 1) / 2
    }
}

fn left_child_index(index: usize) -> usize {
    index * 2 + 1
}

fn right_child_index(index: usize) -> usize {
    index * 2 + 2
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> BinaryHeap<T> {
        BinaryHeap::<T> { values: vec![] }
    }

    fn insertion_index(&self) -> usize {
        self.values.len()
    }

    fn has_left_child(&self, index: usize) -> bool {
        left_child_index(index) < self.values.len()
    }

    fn has_right_child(&self, index: usize) -> bool {
        right_child_index(index) < self.values.len()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::<T> {
            heap: self,
            index: if self.values.is_empty() {
                None
            } else {
                Some(0)
            },
            flag: TraverseFrom::Parent,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.values.get(0)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn push(&mut self, value: T) {
        let mut index = self.insertion_index();
        self.values.push(value);
        while index != 0 {
            let parent = parent_index(index);
            if self.values[index] > self.values[parent] {
                self.values.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.values.is_empty() {
            None
        } else {
            let ret = self.values.swap_remove(0);
            let mut index = 0;
            while self.has_left_child(index) {
                // loops until reaches leaf
                let left = left_child_index(index);
                let right = right_child_index(index);
                if self.values[index] < self.values[left] {
                    if self.has_right_child(index) && self.values[left] < self.values[right] {
                        self.values.swap(index, right);
                        index = right;
                    } else {
                        self.values.swap(index, left);
                        index = left;
                    }
                } else if self.has_right_child(index) && self.values[index] < self.values[right] {
                    self.values.swap(index, right);
                    index = right;
                } else {
                    break;
                }
            }
            Some(ret)
        }
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn append(&mut self, other: &mut BinaryHeap<T>) {
        for v in other.values.drain(..) {
            self.push(v);
        }
    }
}
