mod sequenced;
use sequenced::SequencedList;

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

    assert_eq!(Some(&1), list2.front());
    assert_eq!(Some(&5), list2.back());
    assert_eq!(None, list.front());
    assert_eq!(None, list.back());

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
