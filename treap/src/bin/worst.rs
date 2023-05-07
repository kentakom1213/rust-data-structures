use treap::treap::*;

fn main() {
    let mut tree = Treap::<i32>::new();

    for x in 0..20 {
        println!("------------------");
        println!("Insert: {}", x);
        tree.insert(x);
        tree.pretty_print();
    }
}