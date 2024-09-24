use splay_tree::{
    multiset::Multiset,
    node::{find::find, pointer::NodeOps},
};

#[test]
fn test_insert() {
    let mut mset = Multiset::new();
    eprintln!("{mset:?}");

    for i in 0..10 {
        mset.insert(i);
    }

    mset.print_as_tree();
    eprintln!("{mset:?}");

    for i in 0..10 {
        mset.insert(i);
        mset.print_as_tree();
    }

    mset.print_as_tree();

    eprintln!("{mset:?}");
}

#[test]
fn test() {
    let mut mset = Multiset::new();

    mset.insert(1);
    mset.insert(2);
    mset.insert(3);

    let node = find(&mset.root, &4);
    println!("4 state: {:?}", node.get_state());

    mset.insert(4);

    let node = find(&mset.root, &4);
    println!("4 state: {:?}", node.get_state());

    mset.insert(1);

    let node = find(&mset.root, &4);
    println!("4 state: {:?}", node.get_state());

    mset.insert(2);

    let node = find(&mset.root, &4);
    println!("4 state: {:?}", node.get_state());

    mset.insert(3);

    let node = find(&mset.root, &4);
    println!("4 state: {:?}", node.get_state());

    mset.insert(4);
}
