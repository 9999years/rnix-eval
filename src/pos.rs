use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::symbol_table::Symbol;

#[derive(Debug, PartialEq)]
pub struct KnownPos<'arena> {
    pub file: Symbol<'arena>,
    pub line: usize,
    pub column: usize,
}

impl Display for KnownPos<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl<'arena> KnownPos<'arena> {
    pub fn to_owned(&self) -> OwnedKnownPos {
        OwnedKnownPos {
            file: self.file.to_owned(),
            line: self.line,
            column: self.column,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OwnedKnownPos {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Display for OwnedKnownPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, PartialEq)]
pub enum Pos<'arena> {
    Undefined,
    Known(KnownPos<'arena>),
}

impl<'arena> Pos<'arena> {
    pub fn to_owned(&self) -> OwnedPos {
        match self {
            Pos::Undefined => OwnedPos::Undefined,
            Pos::Known(pos) => OwnedPos::Known(pos.to_owned()),
        }
    }
}

impl Display for Pos<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pos::Undefined => write!(f, "undefined position"),
            Pos::Known(pos) => write!(f, "{}", pos),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OwnedPos {
    Undefined,
    Known(OwnedKnownPos),
}

impl Display for OwnedPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OwnedPos::Undefined => write!(f, "undefined position"),
            OwnedPos::Known(pos) => write!(f, "{}", pos),
        }
    }
}
