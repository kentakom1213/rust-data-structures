use double_linked_list::double_linked_list::*;

fn main() {
    let mut root = &mut Node::new(5) as *mut Node<_>;
    insert_next(root, 1);

    println!("{:?}", unsafe { root.as_ref().unwrap() }.data);

    root = to_next(root).unwrap();
    println!("{:?}", unsafe { root.as_ref().unwrap() }.data);
}
