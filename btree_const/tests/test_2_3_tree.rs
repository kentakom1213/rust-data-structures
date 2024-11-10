//! 2-3木のテスト
//! `DEGREE = 3` 以外の場合にはエラー

use btree_const::{btree, debug_util::dbg_inner, node::Node};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
