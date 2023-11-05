use splay_tree_multiset::multiset::SplayTreeMultiSet;

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
}

fn main() {
    let mut multiset = SplayTreeMultiSet::<u8>::new();

    loop {
        let x = get!(u8);
        multiset.insert(x);

        println!("--- insert {} ---", x);
        println!("{:#?}", multiset);
    }
}
