use rand::random;
use splay_tree::multiset::MultiSet;

#[test]
fn test_multiset() {
    let mut mset = MultiSet::default();

    for _ in 0..100 {
        let x: i8 = random();

        mset.insert(x);
    }
}
