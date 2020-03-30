pub mod context;
pub mod fetch_git;
pub mod fetch_mercurial;
pub mod from_toml;

type PrimOpFun = ();

#[derive(Debug, PartialEq)]
pub struct PrimOp {
    name: String,
    arity: usize,
    fun: PrimOpFun,
}

pub struct RegisterPrimOp {
    prim_ops: Vec<PrimOp>,
}

impl RegisterPrimOp {
    fn register(&mut self, prim_op: PrimOp) {
        self.prim_ops.push(prim_op)
    }
}
