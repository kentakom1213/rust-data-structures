use std::rc::{Rc, Weak};

use skiplist::node::{insert_next, next, ListNode};

fn main() {
    let mut list = Some(ListNode::new(0));
    println!("{:?}", list);

    list = insert_next(list, 5);
    println!("{:?}", list);

    list = insert_next(list, 4);
    println!("{:?}", list);

    list = next(list);
    println!("{:?}", list);
}
