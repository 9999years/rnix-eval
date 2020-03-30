use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use ordered_float::OrderedFloat;

pub use crate::attr_set::Bindings;
pub use crate::eval::Env;
pub use crate::nix_expr::Expr;
pub use crate::primops::PrimOp;

pub type NixInt = i64;
pub type NixFloat = OrderedFloat<f64>;
pub type ExprLambda = ();

#[derive(Debug, Hash, PartialEq)]
pub struct Thunk<'arena> {
    pub env: Env,
    pub expr: Expr<'arena>,
}

#[derive(Debug, Hash, PartialEq)]
pub struct App {
    pub left: Value,
    pub right: Value,
}

#[derive(Debug, Hash, PartialEq)]
pub struct Lambda {
    pub env: Env,
    pub fun: Expr<'arena>::Lambda,
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

#[derive(Debug, Hash, PartialEq)]
pub enum Value {
    Int(NixInt),
    Bool(bool),
    String(NixString),
    Path(PathBuf),
    Null,
    Attrs(Bindings),
    /// `List` represents `tList1`, `tList2` and `tListN`.
    List(Vec<Value>),
    Thunk(Thunk),
    App(Box<App>),
    Lambda,
    Blackhole,
    PrimOp(PrimOp),
    PrimOpApp(Box<App>),
    External,
    Float(NixFloat),
}

struct ValueDisplay {
    value: Value,
    active: HashSet<Value>,
}
