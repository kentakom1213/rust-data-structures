use std::borrow::Borrow;
use std::cell::UnsafeCell;
use std::cmp::Ordering::{Less, Equal, Greater};
use std::default::Default;
use std::iter::{FromIterator, IntoIterator};
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    pub fn new(k: K, v: V,
               l: Option<Box<Node<K, V>>>,
               r: Option<Box<Node<K, V>>>) -> Box<Node<K, V>> {
        Box::new(Node{ key: k, value: v, left: l, right: r })
    }

    #[inline(always)]
    pub fn pop_left(&mut self) -> Option<Box<Node<K, V>>> {
        mem::replace(&mut self.left, None)
    }

    #[inline(always)]
    pub fn pop_right(&mut self) -> Option<Box<Node<K, V>>> {
        mem::replace(&mut self.right, None)
    }
}

/// The implementation of this splay tree is largely based on the c code at:
///     ftp://ftp.cs.cmu.edu/usr/ftp/usr/sleator/splaying/top-down-splay.c
/// This version of splaying is a top-down splay operation.
pub struct SplayMap<K: Ord, V> {
    root: UnsafeCell<Option<Box<Node<K, V>>>>,
    size: usize,
}

pub struct IntoIter<K, V> {
    cur: Option<Box<Node<K, V>>>,
    remaining: usize,
}

/// Performs a top-down splay operation on a tree rooted at `node`. This will
/// modify the pointer to contain the new root of the tree once the splay
/// operation is done. When finished, if `key` is in the tree, it will be at the
/// root. Otherwise the closest key to the specified key will be at the root.
fn splay<K, V, Q: ?Sized>(key: &Q, node: &mut Box<Node<K, V>>)
    where K: Ord + Borrow<Q>, Q: Ord
{
    let mut newleft = None;
    let mut newright = None;

    // Eplicitly grab a new scope so the loans on newleft/newright are
    // terminated before we move out of them.
    {
        // Yes, these are backwards, that's intentional.
        let mut l = &mut newright;
        let mut r = &mut newleft;

        loop {
            match key.cmp(node.key.borrow()) {
                // Found it, yay!
                Equal => { break }

                Less => {
                    let mut left = match node.pop_left() {
                        Some(left) => left, None => break
                    };
                    // rotate this node right if necessary
                    if key.cmp(left.key.borrow()) == Less {
                        // A bit odd, but avoids drop glue
                        mem::swap(&mut node.left, &mut left.right);
                        mem::swap(&mut left, node);
                        let none = mem::replace(&mut node.right, Some(left));
                        match mem::replace(&mut node.left, none) {
                            Some(l) => { left = l; }
                            None    => { break }
                        }
                    }

                    *r = Some(mem::replace(node, left));
                    let tmp = r;
                    r = &mut tmp.as_mut().unwrap().left;
                }

                // If you look closely, you may have seen some similar code
                // before
                Greater => {
                    match node.pop_right() {
                        None => { break }
                        // rotate left if necessary
                        Some(mut right) => {
                            if key.cmp(right.key.borrow()) == Greater {
                                mem::swap(&mut node.right, &mut right.left);
                                mem::swap(&mut right, node);
                                let none = mem::replace(&mut node.left,
                                                         Some(right));
                                match mem::replace(&mut node.right, none) {
                                    Some(r) => { right = r; }
                                    None    => { break }
                                }
                            }
                            *l = Some(mem::replace(node, right));
                            let tmp = l;
                            l = &mut tmp.as_mut().unwrap().right;
                        }
                    }
                }
            }
        }

        mem::swap(l, &mut node.left);
        mem::swap(r, &mut node.right);
    }

    node.left = newright;
    node.right = newleft;
}