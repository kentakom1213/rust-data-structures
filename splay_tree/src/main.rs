use splay_tree::collections::multiset::Multiset;

fn main() {
    let mut mset = Multiset::new();

    for i in 0..20 {
        mset.insert(i);
    }

    mset.count(&0);

    mset.print_as_tree();
}
