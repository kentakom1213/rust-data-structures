/// ## splay_rev
/// - 比較関数`compare`を引数にとり、条件を満たす最小のノードを返す
/// - splayの逆向き
fn splay_rev<T, C>(mut root: Option<Box<Node<T>>>, key: &T, compare: C) -> (Option<Box<Node<T>>>, bool)
where
    T: Ord + Debug,
    C: Fn(&T, &T) -> bool,
{
    if root.is_none() {
        return (root, false);
    }
    if compare(key, &root.as_ref().unwrap().key) {
        let right = &mut root.as_mut().unwrap().right;
        if right.is_none() {
            return (root, true);
        }
        if compare(key, &right.as_ref().unwrap().key) {
            let rightright = right.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = splay_rev(rightright, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().right, &mut tmp);
            // 親を左に回転
            let tmp_right = rotate_left(root);
            if !is_found {
                return (tmp_right, true);
            }
            // さらに左回転
            (rotate_left(tmp_right), true)
        } else {
            let rightleft = right.as_mut().unwrap().left.take();
            let (mut new_rightleft, is_found) = splay_rev(rightleft, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().left, &mut new_rightleft);
            // root->right->leftがNoneでないとき
            if !is_found {
                return (root, true);
            }
            // 右の子を右回転
            let right = root.as_mut().unwrap().right.take();
            let mut tmp_child = rotate_right(right);
            swap(&mut root.as_mut().unwrap().right, &mut tmp_child);
            // 親を左回転
            (rotate_left(root), true)
        }
    } else {
        let left = &mut root.as_mut().unwrap().left;
        if left.is_none() {
            return (root, false);
        }
        if compare(key, &left.as_ref().unwrap().key) {
            let leftright = left.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = splay_rev(leftright, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().right, &mut tmp);
            if is_found {
                // 左の子を左回転
                let left = root.as_mut().unwrap().left.take();
                let mut tmp_child = rotate_left(left);
                swap(&mut root.as_mut().unwrap().left, &mut tmp_child);
            }
            // 親を右回転
            (rotate_right(root), true)
        } else {
            let leftleft = left.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = splay_rev(leftleft, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().left, &mut tmp);
            // 親を右回転
            let tmp_child = rotate_right(root);
            // さらに右回転
            (rotate_right(tmp_child), is_found)
        }
    }
}
