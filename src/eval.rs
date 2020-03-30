use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::nix_expr::{Expr, Pos, StaticEnv};
use crate::symbol_table::{Symbol, SymbolTable};
use crate::value::Value;

#[derive(Debug, Hash, PartialEq)]
pub enum EnvKind {
    Plain,
    HasWithExpr,
    HasWithAttrs,
}

#[derive(Debug, Hash, PartialEq)]
pub struct Env {
    pub up: Box<Env>,
    pub prev_with: u16,
    pub kind: EnvKind,
    pub values: Vec<Value>,
}

pub type FileParseCache = HashMap<PathBuf, Box<Expr>>;
pub type FileEvalCache = HashMap<PathBuf, Value>;
pub type SearchPathElem = (String, String); // ???
pub type SearchPath = Vec<SearchPathElem>;

type ExprLambda = (); // TODO

pub struct EvalState<'arena> {
    pub symbols: SymbolTable,
    pub sWith: Symbol<'arena>,
    pub sOutPath: Symbol<'arena>,
    pub sDrvPath: Symbol<'arena>,
    pub sType: Symbol<'arena>,
    pub sMeta: Symbol<'arena>,
    pub sName: Symbol<'arena>,
    pub sValue: Symbol<'arena>,
    pub sSystem: Symbol<'arena>,
    pub sOverrides: Symbol<'arena>,
    pub sOutputs: Symbol<'arena>,
    pub sOutputName: Symbol<'arena>,
    pub sIgnoreNulls: Symbol<'arena>,
    pub sFile: Symbol<'arena>,
    pub sLine: Symbol<'arena>,
    pub sColumn: Symbol<'arena>,
    pub sFunctor: Symbol<'arena>,
    pub sToString: Symbol<'arena>,
    pub sRight: Symbol<'arena>,
    pub sWrong: Symbol<'arena>,
    pub sStructuredAttrs: Symbol<'arena>,
    pub sBuilder: Symbol<'arena>,
    pub sArgs: Symbol<'arena>,
    pub sOutputHash: Symbol<'arena>,
    pub sOutputHashAlgo: Symbol<'arena>,
    pub sOutputHashMode: Symbol<'arena>,
    pub sDerivationNix: Symbol<'arena>,
    /// The allowed filesystem paths in restricted or pure evaluation
    /// mode.
    pub allowed_paths: Option<HashSet<PathBuf>>,
    pub empty_set: Value,
    // If set, force copying files to the Nix store even if they
    // already exist there.
    // RepairFlag
    // Store
    // SrcToStore
    /// A cache from path names to parse trees.
    file_parse_cache: FileParseCache,

    /// A cache from path names to values.
    file_eval_cache: FileEvalCache,

    search_path: SearchPath,
    search_path_resolved: HashMap<String, (bool, String)>,
    /// Cache used by checkSourcePath().
    resolved_paths: HashMap<PathBuf, PathBuf>,
    /// Cache used by prim_match().
    regex_cache: HashMap<String, ()>,

    /// The base environment, containing the builtin functions and
    /// values.
    pub base_env: Box<Env>,

    /// The same as `base_env`, but used during parsing to resolve variables.
    static_base_env: StaticEnv<'arena>,
    base_env_display: usize,
    // Statistics tracking...?
    // nr_envs: usize,
    // nr_values_in_envs: usize,
    // nr_values: usize,
    // nr_list_elems: usize,
    // nr_attrsets: usize,
    // nr_attrs_in_attrsets: usize,
    // nr_op_updates: usize,
    // nr_op_update_values_copied: usize,
    // nr_list_concats: usize,
    // nr_primop_calls: usize,
    // nr_function_calls: usize,
    // count_calls: bool,

    // primop_calls: HashMap<Symbol<'arena>, usize>,
    // function_calls: HashMap<ExprLambda, usize>,
    // attr_selects: HashMap<Pos<'arena>, usize>,
}
