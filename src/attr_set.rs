use std::cmp::{Ord, Ordering};

use crate::nix_expr::Pos;
use crate::symbol_table::Symbol;
use crate::value::Value;

#[derive(Debug)]
pub struct Attr<'arena> {
    pub name: Symbol<'arena>,
    pub value: Value,
    pub pos: Pos,
}

impl Ord for Attr<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}

pub struct Bindings {}
