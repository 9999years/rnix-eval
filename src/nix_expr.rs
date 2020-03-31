use std::borrow::{Borrow, ToOwned};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::env::{Displ, Env, Level, StaticEnv, StaticEnvLevel};
use crate::err::{NixError, NixResult};
use crate::eval::EvalState;
use crate::pos::Pos;
use crate::symbol_table::{Symbol, SymbolTable};
use crate::value::{NixFloat, NixInt, Value};

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
    displ: Displ,
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

pub trait ExprExt {
    fn bind_vars<'env>(&mut self, env: &StaticEnv<'env>) -> NixResult<()> {
        Ok(())
    }
    fn eval<'a>(&'a self, state: &EvalState, env: &Env) -> NixResult<&'a Value>;
    fn maybe_thunk<'a>(&'a self, state: &EvalState, env: &Env) -> &'a Value;
    /// Storing function names.
    fn set_name<'env>(name: Symbol<'env>) {}
}

#[derive(Debug, PartialEq)]
pub enum OpKind {
    App,
    Eq,
    NEq,
    And,
    Or,
    Impl,
    Update,
    ConcatLists,
}

#[derive(Debug, PartialEq)]
pub struct BinOp<'arena> {
    kind: OpKind,
    pos: Pos<'arena>,
    e1: Box<Expr<'arena>>,
    e2: Box<Expr<'arena>>,
}

#[derive(Debug, PartialEq)]
pub struct ExprVar<'arena> {
    pub pos: Pos<'arena>,
    pub name: Symbol<'arena>,
    pub from_with: bool,
    pub level: Level,
    /// Displacement
    pub displ: Displ,
}

impl<'arena> ExprExt for ExprVar<'arena> {
    fn bind_vars<'env>(&mut self, env: &StaticEnv<'env>) -> NixResult<()> {
        // Check whether the variable appears in the environment. If so,
        // set its level and displacement.
        let with_level = None;
        for env_level in env.into_iter() {
            with_level = env_level.with_level;
            if let Some(displ) = env_level.env.vars.get(&self.name) {
                self.from_with = false;
                self.level = env_level.level;
                self.displ = *displ;
                return Ok(());
            }
        }

        match with_level {
            None => Err(NixError::UndefinedVar(
                self.name.into(),
                self.pos.to_owned(),
            )),
            Some(with_level) => {
                self.from_with = true;
                self.level = with_level;
                Ok(())
            }
        }
    }

    fn eval<'a>(&'a self, state: &EvalState, env: &Env) -> NixResult<&'a Value> {
        unimplemented!()
    }

    fn maybe_thunk<'a>(&'a self, state: &EvalState, env: &Env) -> &'a Value {
        unimplemented!()
    }
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

impl<'arena> ExprExt for ExprAttrs<'arena> {
    fn bind_vars<'env>(&mut self, env: &StaticEnv<'env>) -> NixResult<()> {
        unimplemented!()
    }

    fn eval<'a>(&'a self, state: &EvalState, env: &Env) -> NixResult<&'a Value> {
        unimplemented!()
    }

    fn maybe_thunk<'a>(&'a self, state: &EvalState, env: &Env) -> &'a Value {
        unimplemented!()
    }
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
    prev_with: Level,
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
    BinOp(BinOp<'arena>),
    ConcatStrings(ExprConcatStrings<'arena>),
    Pos(Pos<'arena>),
}

impl<'arena> ExprExt for Expr<'arena> {
    fn bind_vars<'env>(&mut self, env: &StaticEnv<'env>) -> NixResult<()> {
        unimplemented!()
    }

    fn eval<'a>(&'a self, state: &EvalState, env: &Env) -> NixResult<&'a Value> {
        unimplemented!()
    }

    fn maybe_thunk<'a>(&'a self, state: &EvalState, env: &Env) -> &'a Value {
        unimplemented!()
    }
}
