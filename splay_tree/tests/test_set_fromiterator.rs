#[cfg(test)]
use splay_tree::splay_tree_set::*;

#[test]
fn test_from_range() {
    let mut set: SplayTreeSet<usize> = (0..20).collect();

    println!("--- collect from (0..20) ---");
    println!("{:?}", &set);

    assert_eq!(set.len(), 20);

    println!("--- get 0 ---");
    set.get(&0);
    println!("{:?}", &set);
}
