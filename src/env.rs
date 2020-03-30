use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use derive_more::{Add, AddAssign, From, Into};

use crate::symbol_table::Symbol;
use crate::value::Value;

/// A variable's environment level.
#[derive(Debug, PartialEq, Copy, Clone, From, Into, Add, AddAssign)]
pub struct Level(pub usize);

/// A variable's displacement within an environment level.
#[derive(Debug, PartialEq, Copy, Clone, From, Into, Add, AddAssign)]
pub struct Displ(pub usize);

#[derive(Debug, PartialEq)]
pub enum EnvKind {
    Plain,
    HasWithExpr,
    HasWithAttrs,
}

#[derive(Debug, PartialEq)]
pub struct Env<'arena> {
    pub up: Option<Box<Env<'arena>>>,
    pub prev_with: Level,
    pub kind: EnvKind,
    pub values: Vec<Value<'arena>>,
}

impl<'arena> IntoIterator for Env<'arena> {
    type Item = EnvLevel<'arena>;
    type IntoIter = EnvIter<'arena>;
    fn into_iter(self) -> Self::IntoIter {
        EnvIter::new(Box::new(self))
    }
}

pub struct EnvLevel<'arena> {
    pub env: Box<Env<'arena>>,
    pub level: Level,
}

pub struct EnvIter<'arena> {
    cur_env: Option<Box<Env<'arena>>>,
    level: Level,
}

impl<'arena> EnvIter<'arena> {
    fn new(env: Box<Env<'arena>>) -> Self {
        EnvIter {
            cur_env: Some(env),
            level: Level(0),
        }
    }
}

impl<'arena> Iterator for EnvIter<'arena> {
    type Item = EnvLevel<'arena>;
    fn next(&mut self) -> Option<Self::Item> {
        // If the current env is None, return early.
        let env = self.cur_env?;
        let ret = Some(EnvLevel {
            env,
            level: self.level,
        });

        // Increment for next iteration.
        self.cur_env = env.up;
        self.level += Level(1);
        ret
    }
}

pub type Vars<'arena> = HashMap<Symbol<'arena>, Displ>;

pub struct StaticEnv<'arena> {
    pub is_with: bool,
    pub up: Option<Box<StaticEnv<'arena>>>,
    pub vars: Vars<'arena>,
}

impl<'arena> IntoIterator for StaticEnv<'arena> {
    type Item = StaticEnvLevel<'arena>;
    type IntoIter = StaticEnvIter<'arena>;
    fn into_iter(self) -> Self::IntoIter {
        StaticEnvIter::new(Box::new(self))
    }
}

pub struct StaticEnvLevel<'arena> {
    pub env: Box<StaticEnv<'arena>>,
    pub level: Level,
    pub with_level: Option<Level>,
}

pub struct StaticEnvIter<'arena> {
    cur_env: Option<Box<StaticEnv<'arena>>>,
    level: Level,
    with_level: Option<Level>,
}

impl<'arena> StaticEnvIter<'arena> {
    fn new(env: Box<StaticEnv<'arena>>) -> Self {
        StaticEnvIter {
            cur_env: Some(env),
            level: Level(0),
            with_level: None,
        }
    }
}

impl<'arena> Iterator for StaticEnvIter<'arena> {
    type Item = StaticEnvLevel<'arena>;

    fn next(&mut self) -> Option<Self::Item> {
        // If the current env is None, return early.
        let env = self.cur_env?;
        // If the current env is a `with`, set the `with_level`.
        if env.is_with && self.with_level.is_none() {
            self.with_level = Some(self.level);
        }
        let ret = Some(StaticEnvLevel {
            env,
            level: self.level,
            with_level: self.with_level,
        });

        // Increment for next iteration.
        self.cur_env = env.up;
        self.level += Level(1);
        ret
    }
}
