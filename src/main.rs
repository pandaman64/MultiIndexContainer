mod binary_heap;
use binary_heap::BinaryHeap;

fn main() {
    let mut list = BinaryHeap::<i32>::new();
    let mut list2 = BinaryHeap::<i32>::new();
    list.push(3);
    list.push(2);
    list.push(4);
    list.push(5);
    list.push(6);

    println!("length = {}", list.len());
    assert_eq!(Some(&6), list.peek());
    assert_eq!(Some(6), list.pop());
    assert_eq!(Some(5), list.pop());
    assert_eq!(Some(4), list.pop());
    assert_eq!(Some(3), list.pop());
    assert_eq!(Some(2), list.pop());
    assert_eq!(None, list.peek());
    println!("length = {}", list.len());
    println!("is_empty() = {}", list.is_empty());

    list.push(2);
    list.push(4);
    list.push(3);
    list2.push(5);
    list2.push(1);

    list2.append(&mut list);

    println!("{} {}", list.len(), list2.len());

    assert_eq!(Some(5), list2.pop());
    assert_eq!(Some(4), list2.pop());
    assert_eq!(Some(3), list2.pop());
    assert_eq!(Some(2), list2.pop());
    assert_eq!(Some(1), list2.pop());
    assert_eq!(None, list2.pop());
    assert_eq!(None, list2.pop());

    list.push(3);
    list.push(2);
    list.push(4);
    list.push(5);
    list.push(6);

    for v in list.iter() {
        println!("{}", v);
    }

    list.clear();

    for v in list.iter() {
        println!("{}", v);
    }
}
