use rand::prelude::*;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct TreapNode<T> {
    value: T,
    priority: u64,
    left: Option<Box<TreapNode<T>>>,
    right: Option<Box<TreapNode<T>>>,
}

#[derive(Debug)]
pub struct Treap<T> {
    root: Option<Box<TreapNode<T>>>,
}
