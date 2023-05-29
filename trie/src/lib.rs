/// # trie
/// - トライ木の実装
pub mod trie {
    use std::mem::{
        replace,
        swap,
    };

    // 定数
    const ORIGIN: char = 'a'; // 基準となる文字
    const ORIGIN_ID: usize = ORIGIN as u32 as usize; // 基準となる文字のID
    const KINDS: usize = 26; // 文字の種類数
    type NodePointer<T> = Option<Box<TrieNode<T>>>;

    /// 何番目の文字かを判定する関数
    fn ord(c: char) -> usize {
        let num = c as u32 as usize;
        num - ORIGIN_ID
    }

    #[derive(Debug, Clone)]
    struct TrieNode<T> {
        data: T,
        children: Vec<NodePointer<T>>,
    }

    impl<T> TrieNode<T>
    where
        T: Default + Clone,
    {
        pub fn new() -> Self {
            Self { data: T::default(), children: vec![NodePointer::None; KINDS] }
        }
    }

    #[derive(Debug)]
    pub struct Trie<T> {
        root: NodePointer<T>,
    }

    impl<T> Trie<T>
    where
        T: Default + Clone,
    {
        // self.originを基準にした文字の番号を返す
        // fn ord()

        pub fn new() -> Self {
            Trie {
                root: Some(Box::new(TrieNode {
                    data: T::default(),
                    children: vec![NodePointer::None; KINDS],
                })),
            }
        }

        pub fn insert(&mut self, s: &str) {
            let mut node = &mut self.root;
            for c in s.chars().map(ord) {
                if node.as_ref().is_none() {
                    *node = Some(Box::new(TrieNode::new()));
                }
                node = node.as_mut().unwrap().children.get_mut(c).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::trie::Trie;

    use super::*;

    #[test]
    fn test_trie_node() {
        let mut trie: Trie<usize> = trie::Trie::new();

        // 文字列の挿入
        trie.insert("powell");
        trie.insert("powell");
        trie.insert("powell");

        println!("{:#?}", trie);
    }
}
