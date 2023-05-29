/// # trie
/// - トライ木の実装
pub mod trie {

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
        pub fn new() -> Self {
            Self { data: None, children: vec![NodePointer::None; KINDS] }
        }
    }

    #[derive(Debug)]
    pub struct Trie<T> {
        root: NodePointer<T>,
    }

    impl<T> Trie<T>
    where
        T: Clone,
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

        pub fn insert(&mut self, s: &str, data: T) {
            let mut node = &mut self.root;
            let mut idx = 0;
            for c in s.chars().map(ord) {
                if node.as_ref().is_none() {
                    *node = Some(Box::new(TrieNode::new()));
                }
                node = node.as_mut().unwrap().children.get_mut(c).unwrap();
                if idx + 1 == s.len() {
                    *node = Some(Box::new(
                        TrieNode { data: Some(data), children: vec![None; KINDS] }
                    ));
                    break;
                }
                idx += 1;
            }
        }

        pub fn traverse(&self) -> Vec<String> {
            let mut res = vec![];
            let mut cur = String::new();
            traverse_inner(&self.root, &mut cur, &mut res);
            res
        }
    }

    /// trieを順に探索する
    fn traverse_inner<T>(node: &NodePointer<T>, cur: &mut String, list: &mut Vec<String>) {
        if node.as_ref().unwrap().data.is_some() {
            list.push(cur.clone());
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
        trie.insert("powell", 1);

        // デバッグ
        println!("{:#?}", trie);

        // 一覧表示
        let dict = trie.traverse();
        println!("{:#?}", dict);
    }
}
