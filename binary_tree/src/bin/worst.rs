use binary_tree::binary_tree::*;

fn main() {
    let mut tree = BinaryTree::<i32>::new();

    for x in 0..20 {
        println!("------------------");
        println!("Insert: {}", x);
        tree.insert(x);
        tree.pretty_print();
    }
}