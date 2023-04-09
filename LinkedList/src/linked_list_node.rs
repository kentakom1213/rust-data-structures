#[allow(dead_code)]

/// # LinkedList
/// 連結リストのノード
#[derive(Debug)]
pub enum LinkedList<T> {
    Null,
    Node {
        val: T,
        next: Box<LinkedList<T>>,
    }   
}

impl<T> LinkedList<T>
where
    T: std::cmp::Eq
{
    /// ## replace
    /// `self`を引数で置き換える。
    pub fn replace(&mut self, node: Self) {
        *self = node;
    }

    /// ## remove
    /// ノードを削除する。
    pub fn remove(&mut self) {
        *self = LinkedList::Null;
    }

    /// ## get_next
    /// 次のノードを返す
    pub fn get_next(&mut self) -> Option<&mut LinkedList<T>> {
        match self {
            LinkedList::Null => None,
            LinkedList::Node { val: _, next } => {
                Some( &mut **next )
            },
        }
    }

    /// ## nth
    /// 与えられたノードから数えてn番目のノードを返す
    pub fn nth(&mut self, n: usize) -> Option<&mut LinkedList<T>> {
        let mut res = self;

        for _ in 0..n {
            match res {
                LinkedList::Null => {
                    return None;
                },
                LinkedList::Node { val: _, next } => {
                    res = &mut **next;
                },
            }
        }

        Some( res )
    }

    /// ## find
    /// 与えられたノードの子要素から、要素`x`を探索する
    pub fn find(&self, x: T) -> Option<&LinkedList<T>> {
        let mut res = self;

        while let LinkedList::Node { val, next } = res {
            if *val == x {
                return Some( res )
            }
            res = &**next;
        }

        None
    }

    /// ## insert
    /// 与えられたノードの後ろに新しい要素を追加する
    pub fn insert(&mut self, node: Self) {
        
    }
}
