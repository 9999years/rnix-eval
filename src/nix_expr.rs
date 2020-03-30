use std::collections::HashMap;
use std::path::PathBuf;

use crate::symbol_table::{Symbol, SymbolTable};
use crate::value::{NixFloat, NixInt};

#[derive(Debug, PartialEq)]
pub struct Pos<'arena> {
    pub file: Symbol<'arena>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub struct AttrName<'arena> {
    pub symbol: Symbol<'arena>,
    pub expr: Box<Expr<'arena>>,
}

pub type AttrPath<'arena> = Vec<AttrName<'arena>>;

#[derive(Debug, PartialEq)]
pub struct AttrDef<'arena> {
    inherited: bool,
    expr: Box<Expr<'arena>>,
    pos: Pos<'arena>,
    /// Displacement
    displ: usize,
}

#[derive(Debug, PartialEq)]
pub struct DynamicAttrDef<'arena> {
    name_expr: Box<Expr<'arena>>,
    value_expr: Box<Expr<'arena>>,
    pos: Pos<'arena>,
}

#[derive(Debug, PartialEq)]
pub struct Formal<'arena> {
    name: Symbol<'arena>,
    def: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct Formals<'arena> {
    formals: Vec<Formal<'arena>>,
    ellipsis: bool,
}

#[derive(Debug, PartialEq)]
pub struct BinOp<'arena> {
    pos: Pos<'arena>,
    e1: Box<Expr<'arena>>,
    e2: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprVar<'arena> {
    pos: Pos<'arena>,
    name: Symbol<'arena>,
    from_with: bool,
    level: usize,
    /// Displacement
    displ: usize,
}

#[derive(Debug, PartialEq)]
pub struct ExprSelect<'arena> {
    pos: Pos<'arena>,
    expr: Box<Expr<'arena>>,
    def: Box<Expr<'arena>>,
    attr_path: AttrPath<'arena>,
}

#[derive(Debug, PartialEq)]
pub struct ExprOpHasAttr<'arena> {
    expr: Box<Expr<'arena>>,
    attr_path: AttrPath<'arena>,
}

#[derive(Debug, PartialEq)]
pub struct ExprAttrs<'arena> {
    recursive: bool,
    attrs: HashMap<Symbol<'arena>, AttrDef<'arena>>,
    dynamic_attrs: Vec<DynamicAttrDef<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprLambda<'arena> {
    pos: Pos<'arena>,
    name: Symbol<'arena>,
    arg: Symbol<'arena>,
    match_attrs: bool,
    formals: Formals<'arena>,
    body: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprLet<'arena> {
    attrs: Box<ExprAttrs<'arena>>,
    body: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprWith<'arena> {
    pos: Pos<'arena>,
    attrs: Box<Expr<'arena>>,
    body: Box<Expr<'arena>>,
    prev_with: usize,
}

#[derive(Debug, PartialEq)]
pub struct ExprIf<'arena> {
    cond: Box<Expr<'arena>>,
    then: Box<Expr<'arena>>,
    else_: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprAssert<'arena> {
    pos: Pos<'arena>,
    cond: Box<Expr<'arena>>,
    body: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprConcatStrings<'arena> {
    pos: Pos<'arena>,
    force_string: bool,
    exprs: Vec<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'arena> {
    Int(NixInt),
    Float(NixFloat),
    String(Symbol<'arena>),
    Path(PathBuf),
    Var(ExprVar<'arena>),
    Select(ExprSelect<'arena>),
    OpHasAttr(ExprOpHasAttr<'arena>),
    Attrs(ExprAttrs<'arena>),
    List(Vec<Expr<'arena>>),
    Lambda(ExprLambda<'arena>),
    Let(ExprLet<'arena>),
    With(ExprWith<'arena>),
    If(ExprIf<'arena>),
    Assert(ExprAssert<'arena>),
    OpNot(Box<Expr<'arena>>),
    App(BinOp<'arena>),
    OpEq(BinOp<'arena>),
    OpNEq(BinOp<'arena>),
    OpAnd(BinOp<'arena>),
    OpOr(BinOp<'arena>),
    OpImpl(BinOp<'arena>),
    OpUpdate(BinOp<'arena>),
    OpConcatLists(BinOp<'arena>),
    ConcatStrings(ExprConcatStrings<'arena>),
    Pos(Pos<'arena>),
}

pub type Vars<'arena> = HashMap<Symbol<'arena>, usize>;
pub struct StaticEnv<'arena> {
    pub is_with: bool,
    pub up: Box<StaticEnv<'arena>>,
    pub vars: Vars<'arena>,
}
