use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;

use crate::pos::Pos;
use crate::symbol_table::Symbol;
use crate::Value;

#[derive(Debug, PartialEq)]
pub struct Attr<'arena> {
    pub name: Symbol<'arena>,
    pub value: AttrValue<'arena>,
}

impl<'arena> Eq for Attr<'arena> {}

#[derive(Debug, PartialEq)]
pub struct AttrValue<'arena> {
    pub value: Value<'arena>,
    pub pos: Pos<'arena>,
}

impl<'arena> Eq for AttrValue<'arena> {}

impl PartialOrd for Attr<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Attr<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(other.name)
    }
}

#[derive(Debug, PartialEq)]
pub struct Bindings<'arena>(HashMap<Symbol<'arena>, AttrValue<'arena>>);

impl<'arena> Bindings<'arena> {
    fn sorted(&self) -> Vec<Attr<'arena>> {
        let mut ret: Vec<_> = self
            .0
            .iter()
            .map(|(name, value)| Attr {
                name,
                value: *value,
            })
            .collect();
        ret.sort();
        ret
    }
}
