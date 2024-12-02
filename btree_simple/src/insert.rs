use crate::{
    node::{BTreeNode, NodePtr},
    node_util::NodeUtil,
};

/// B木に値を挿入する
/// - `root`：挿入対象の木のルート
/// - `key`：挿入するキー
/// - `value`：挿入する値
pub fn insert<const D: usize, K, V>(
    root: Option<NodePtr<D, K, V>>,
    key: K,
    value: V,
) -> Option<NodePtr<D, K, V>>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    let mut root = root.unwrap_or_else(BTreeNode::alloc_leaf);

    if root.is_full() {
        // 新しい葉ノードを作成
        let mut s = BTreeNode::new_internal();

        s.children.as_mut().unwrap()[0] = Some(root);

        // 分割
        split_child(&mut s, 0);

        // sにデータを挿入
        insert_non_full(&mut s, key, value);

        Some(Box::new(s))
    } else {
        insert_non_full::<D, _, _>(root.as_mut(), key, value);

        Some(root)
    }
}

/// 空きのある葉ノードにデータを挿入する
fn insert_non_full<const D: usize, K, V>(x: &mut BTreeNode<D, K, V>, key: K, value: V)
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    // if x.size == 0 {
    //     // 新しいノードを確保
    //     let mut new_node = BTreeNode::new_leaf();

    //     new_node.keys[0] = Some(key);
    //     new_node.vals[0] = Some(value);

    //     return;
    // }

    // 後ろにデータを移動し，挿入する位置を見つける
    // insert([1, 3, -], 2)
    // ---
    // 1. [1, 3, -]: idx=2
    // 2. [1, -, 3]: idx=1
    // 3. [1, 2, 3]: idx=1に2を挿入して終了

    // 挿入する位置
    let mut i = x.size;

    if x.is_leaf() {
        // keyより大きい要素を1つずつ後ろに移動
        while i > 0 && &key < x.keys[i - 1].as_ref().unwrap() {
            x.keys[i] = x.keys[i - 1].take();
            x.vals[i] = x.vals[i - 1].take();
            i -= 1;
        }

        // キー，値を挿入
        x.keys[i] = Some(key);
        x.vals[i] = Some(value);

        x.size += 1;
    } else {
        // 挿入対象の子ノードを見つける
        while i > 0 && &key < x.keys[i - 1].as_ref().unwrap() {
            i -= 1;
        }
        i += 1;

        // 子ノードに空きがない場合，分割
        if x.nth_child(i - 1).is_some_and(|ch| ch.is_full()) {
            // 左の子を分割
            split_child(x, i - 1);

            // 右の子に挿入するか判定
            if &key > x.keys[i - 1].as_ref().unwrap() {
                i += 1;
            }
        }

        // 子ノードに挿入
        let mut ith_child = x.children.as_mut().unwrap()[i - 1].take().unwrap();
        insert_non_full(&mut *ith_child.as_mut(), key, value);
        x.children.as_mut().unwrap()[i - 1] = Some(ith_child);
    }
}

/// ノード`x`の`i`番目の子`y`が飽和しているとき，頂点を分割する
///
/// **引数**
/// - `x`：分割する親ノード
/// - `i`：分割する子ノードのインデックス
fn split_child<const D: usize, K, V>(x: &mut BTreeNode<D, K, V>, i: usize)
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    debug_assert!(!x.is_leaf());
    debug_assert!(!x.is_full());
    debug_assert!(x.children.as_ref().unwrap()[i].is_some());

    let x_children = x.children.as_mut().unwrap();

    let mut y = x_children[i].take().unwrap();

    let mut z = if y.is_leaf() {
        BTreeNode::new_leaf()
    } else {
        BTreeNode::new_internal()
    };

    // キー，値を付け替える
    for j in 0..D - 1 {
        z.keys[j] = y.keys_mut()[j + D].take();
        z.vals[j] = y.vals_mut()[j + D].take();
    }

    z.size = D - 1;

    // 子を付け替える
    if let Some((y_children, z_children)) = y.children_mut().as_mut().zip(z.children.as_mut()) {
        for j in 0..D {
            z_children[j] = y_children[j + D].take();
        }
    }

    *y.size_mut() = D - 1;

    // xのi番目より右の子を1つづつ右にずらす
    for j in (i + 1..x.size + 1).rev() {
        x_children[j + 1] = x_children[j].take();
    }

    // zをxのi+1番目の子にする
    x_children[i + 1] = Some(Box::new(z));

    // xのi番目より右のキー，値を1つづつ右にずらす
    for j in (i..x.size).rev() {
        x.keys[j + 1] = x.keys[j].take();
        x.vals[j + 1] = x.vals[j].take();
    }

    x.keys[i] = y.keys_mut()[D - 1].take();
    x.vals[i] = y.vals_mut()[D - 1].take();

    x.size += 1;

    // yをxのi番目の子にする
    x_children[i] = Some(y);
}
