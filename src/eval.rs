use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::attr_set::Bindings;
use crate::env::{Env, EnvInner, StaticEnv};
use crate::nix_expr::{Expr, ExprExt, ExprVar};
use crate::pos::Pos;
use crate::symbol_table::{Symbol, SymbolTable};
use crate::{NixError, NixResult, Value};

pub type FileParseCache<'arena> = HashMap<PathBuf, Box<Expr<'arena>>>;
pub type FileEvalCache<'arena> = HashMap<PathBuf, Value<'arena>>;
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
    pub empty_set: Value<'arena>,
    // If set, force copying files to the Nix store even if they
    // already exist there.
    // RepairFlag
    // Store
    // SrcToStore
    /// A cache from path names to parse trees.
    file_parse_cache: FileParseCache<'arena>,

    /// A cache from path names to values.
    file_eval_cache: FileEvalCache<'arena>,

    search_path: SearchPath,
    search_path_resolved: HashMap<String, (bool, String)>,
    /// Cache used by checkSourcePath().
    resolved_paths: HashMap<PathBuf, PathBuf>,
    /// Cache used by prim_match().
    regex_cache: HashMap<String, ()>,

    /// The base environment, containing the builtin functions and
    /// values.
    pub base_env: Box<Env<'arena>>,

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

#[derive(Copy, Clone, PartialEq)]
pub enum ShouldEval {
    Yes,
    No,
}

impl<'arena> EvalState<'arena> {
    pub fn eval_attrs(
        &self,
        env: &Env<'arena>,
        expr: &'arena impl ExprExt,
    ) -> NixResult<Bindings<'arena>> {
        let ret = expr.eval(&self, env)?;
        if let Value::Attrs(attrs) = ret {
            Ok(*attrs)
        } else {
            Err(NixError::Type(format!(
                // TODO pretty-print the value
                "value is {:?} while a set was expected",
                ret
            )))
        }
    }

    pub fn lookup_var(
        &self,
        env: &Env<'arena>,
        var: &ExprVar<'arena>,
        should_eval: ShouldEval,
    ) -> NixResult<Value<'arena>> {
        let env = env
            .into_iter()
            .find(|env_level| env_level.level == var.level)
            .unwrap()
            .env;

        if !var.from_with {
            match env.values {
                EnvInner::Plain(values) => return Ok(values[var.displ.0]),
                EnvInner::HasWithAttrs(_) | EnvInner::HasWithExpr(_) => unreachable!(),
            }
        }

        loop {
            if let EnvInner::HasWithExpr(expr) = env.values {
                if should_eval == ShouldEval::No {
                    return Err(NixError::VarLookupUnevaluated(var.name.into()));
                }
                let value = self.eval_attrs(&env.up.unwrap(), &*expr)?;
                env.values = EnvInner::HasWithAttrs(value);
            }

            let first_value = match env.values {
                EnvInner::Plain(values) => match values[0] {
                    Value::Attrs(bindings) => bindings,
                    _ => unreachable!(),
                },
                EnvInner::HasWithAttrs(bindings) => bindings,
                EnvInner::HasWithExpr(_) => unreachable!(),
            };
            if let Some((name, attr)) = first_value
                .0
                .into_iter()
                .find(|(name, _)| name == &var.name)
            {
                return Ok(attr.value);
            }
        }

        // Ok(Value::Blackhole)
    }
}
