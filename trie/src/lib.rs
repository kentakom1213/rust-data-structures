/// # trie
/// - トライ木の実装
pub mod trie {
    use std::fmt::Debug;

    // 定数
    const ORIGIN: char = 'a'; // 基準となる文字
    const ORIGIN_ID: usize = ORIGIN as u32 as usize; // 基準となる文字のID
    const KINDS: usize = 26; // 文字の種類数
    type NodePointer<T> = Option<Box<TrieNode<T>>>;

    /// 何番目の文字かを判定する
    fn ord(c: char) -> usize {
        let num = c as u32 as usize;
        num - ORIGIN_ID
    }

    /// i番目の文字を返す
    fn chr(i: usize) -> char {
        (ORIGIN_ID + i) as u8 as char
    }

    #[derive(Debug, Clone)]
    struct TrieNode<T> {
        data: Option<T>,
        children: Vec<NodePointer<T>>,
    }

    impl<T> TrieNode<T>
    where
        T: Clone,
    {
        pub fn new(data: Option<T>) -> Self {
            Self {
                data,
                children: vec![NodePointer::None; KINDS],
            }
        }
    }

    #[derive(Debug)]
    pub struct Trie<T> {
        root: NodePointer<T>,
    }

    impl<T> Trie<T>
    where
        T: Clone + Debug,
    {
        // self.originを基準にした文字の番号を返す
        // fn ord()

        pub fn new() -> Self {
            Trie {
                root: Some(Box::new(TrieNode {
                    data: None,
                    children: vec![NodePointer::None; KINDS],
                })),
            }
        }

        pub fn insert(&mut self, key: &str, data: T) {
            *self.get_or_insert_mut(key) = Some(data);
        }

        pub fn get(&self, key: &str) -> Option<&T> {
            let mut node = &self.root;
            for c in key.chars().map(ord) {
                node = &node.as_ref()?.children[c];
            }
            node.as_deref()?.data.as_ref()
        }

        pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
            let mut node = &mut self.root;
            for c in key.chars().map(ord) {
                node = node.as_mut()?.children.get_mut(c).unwrap();
            }
            node.as_deref_mut()?.data.as_mut()
        }

        pub fn get_or_insert_mut(&mut self, key: &str) -> &mut Option<T> {
            let mut node = &mut self.root;
            for c in key.chars().map(ord).chain(KINDS..=KINDS) {
                // データの挿入
                if c == KINDS {
                    if node.as_ref().is_none() {
                        *node = Some(Box::new(TrieNode::new(None)));
                    }
                    break;
                }
                if node.as_ref().is_none() {
                    *node = Some(Box::new(TrieNode::new(None)));
                }
                node = node.as_mut().unwrap().children.get_mut(c).unwrap();
            }
            &mut node.as_deref_mut().unwrap().data
        }

        pub fn traverse(&self) -> Vec<(String, &T)> {
            let mut res = vec![];
            let mut cur = String::new();
            traverse_inner(&self.root, &mut cur, &mut res);
            res
        }
    }

    /// trieを順に探索する
    fn traverse_inner<'a, T>(
        node: &'a NodePointer<T>,
        cur: &mut String,
        list: &mut Vec<(String, &'a T)>,
    ) {
        if let Some(value) = node.as_ref().unwrap().data.as_ref() {
            let key = cur.clone();
            list.push((key, value));
        }
        if let Some(node) = node.as_deref() {
            for (i, child) in node.children.iter().enumerate() {
                if child.as_ref().is_some() {
                    cur.push(chr(i));
                    traverse_inner(child, cur, list);
                    cur.pop();
                }
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
        // trie.insert("powell", 5);
        // trie.insert("kentakomoto", 11);
        // trie.insert("kenta", 5);
        // trie.insert("pow", 3);
        // trie.insert("", 0);
        *trie.get_or_insert_mut("powell") = Some(6);
        *trie.get_or_insert_mut("kenta") = Some(8);

        // デバッグ
        println!("{:#?}", trie);

        // 一覧表示
        let dict = trie.traverse();
        println!("{:?}", dict);

        // 検索
        println!("{:?}", trie.get_mut("pow"));

        *trie.get_mut("powell").unwrap() += 1;

        // 一覧表示
        let dict = trie.traverse();
        println!("{:?}", dict);
    }
}
