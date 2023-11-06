#[macro_export]
macro_rules! define_splay {
    ($name:ident, $u:ident, $v:ident, $rotate_u:ident, $rotate_v:ident) => {
        fn $name<T, C>(mut root: Option<Box<Node<T>>>, key: &T, compare: C) -> (Option<Box<Node<T>>>, bool)
        where
            T: Ord + Debug,
            C: Fn(&T, &T) -> bool,
        {
            if root.is_none() {
                return (root, false);
            }
            if compare(key, &root.as_ref().unwrap().key) {
                let $u = &mut root.as_mut().unwrap().$u;
                if $u.is_none() {
                    return (root, true);
                }
                if compare(key, &$u.as_ref().unwrap().key) {
                    let child = $u.as_mut().unwrap().$u.take();
                    let (mut tmp, is_found) = splay(child, key, compare);
                    // 戻す
                    swap(&mut $u.as_mut().unwrap().$u, &mut tmp);
                    // 親を右に回転
                    let tmp_root = $rotate_v(root);
                    if !is_found {
                        return (tmp_root, true);
                    }
                    // さらに右回転
                    ($rotate_v(tmp_root), true)
                } else {
                    let child = $u.as_mut().unwrap().$v.take();
                    let (mut new_child, is_found) = splay(child, key, compare);
                    // 戻す
                    swap(&mut $u.as_mut().unwrap().$v, &mut new_child);
                    // root->$u->$vがNoneでないとき
                    if !is_found {
                        return (root, true);
                    }
                    // 左の子を左回転
                    let $u = root.as_mut().unwrap().$u.take();
                    let mut tmp_child = $rotate_u($u);
                    swap(&mut root.as_mut().unwrap().$u, &mut tmp_child);
                    // 親を右回転
                    ($rotate_v(root), true)
                }
            } else {
                let $v = &mut root.as_mut().unwrap().$v;
                if $v.is_none() {
                    return (root, false);
                }
                if compare(key, &$v.as_ref().unwrap().key) {
                    let child = $v.as_mut().unwrap().$u.take();
                    let (mut tmp, is_found) = splay(child, key, compare);
                    // 戻す
                    swap(&mut $v.as_mut().unwrap().$u, &mut tmp);
                    if is_found {
                        // 右の子を右回転
                        let $v = root.as_mut().unwrap().$v.take();
                        let mut tmp_child = $rotate_v($v);
                        swap(&mut root.as_mut().unwrap().$v, &mut tmp_child);
                    }
                    // 親を左回転
                    ($rotate_u(root), true)
                } else {
                    let child = $v.as_mut().unwrap().$v.take();
                    let (mut tmp, is_found) = splay(child, key, compare);
                    // 戻す
                    swap(&mut $v.as_mut().unwrap().$v, &mut tmp);
                    // 親を左回転
                    let tmp_child = $rotate_u(root);
                    // さらに左回転
                    ($rotate_u(tmp_child), is_found)
                }
            }
        }
    };
}
