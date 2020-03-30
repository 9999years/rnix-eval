use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use ordered_float::OrderedFloat;

pub use crate::attr_set::Bindings;
pub use crate::env::Env;
pub use crate::nix_expr::{Expr, ExprLambda};
pub use crate::primops::PrimOp;

pub type NixInt = i64;
pub type NixFloat = OrderedFloat<f64>;

#[derive(Debug, PartialEq)]
pub struct Thunk<'arena> {
    pub env: Env<'arena>,
    pub expr: Expr<'arena>,
}

#[derive(Debug, PartialEq)]
pub struct App<'arena> {
    pub left: Value<'arena>,
    pub right: Value<'arena>,
}

#[derive(Debug, PartialEq)]
pub struct Lambda<'arena> {
    pub env: Env<'arena>,
    pub fun: ExprLambda<'arena>,
}

#[derive(Debug, Hash, PartialEq)]
pub struct NixString {
    pub s: String,
    pub context: Vec<String>,
}

impl NixString {
    pub fn new(s: String) -> Self {
        Self {
            s,
            context: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value<'arena> {
    Int(NixInt),
    Bool(bool),
    String(NixString),
    Path(PathBuf),
    Null,
    Attrs(Bindings<'arena>),
    /// `List` represents `tList1`, `tList2` and `tListN`.
    List(Vec<Value<'arena>>),
    Thunk(Thunk<'arena>),
    App(Box<App<'arena>>),
    Lambda,
    Blackhole,
    PrimOp(PrimOp),
    PrimOpApp(Box<App<'arena>>),
    External,
    Float(NixFloat),
}

struct ValueDisplay<'arena> {
    value: Value<'arena>,
    active: HashSet<Value<'arena>>,
}
