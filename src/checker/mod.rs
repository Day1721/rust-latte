use std::collections::HashMap;
use data::types::InternalType;

mod topdefs;
mod bodycheck;
mod optimizer;

use super::data::{ast, types::FrontResult};

pub fn check(ast: ast::Program) -> FrontResult<(ast::AttrProgram, HashMap<String, InternalType>)> {
    let mut defs = topdefs::collect_top_defs(&ast)?;
    let mut attr_ast = bodycheck::check_map_program(ast, &mut defs)?;
    optimizer::optimize(&mut attr_ast);
    Ok((attr_ast, defs))
}

static STDLIB: &'static [&'static str] = &["printString", "printInt", "readInt", "readString", "main"];

type CheckerEnv = HashMap<String, VarData>;

#[derive(Clone)]
struct VarData {
    var_type: InternalType,
    initialized: bool,
    mutable: bool
}

trait ToVarData {
    fn to_vd(self) -> VarData;
    fn to_var_data(self, init: bool, mutable: bool) -> VarData;
}

impl ToVarData for InternalType {
    fn to_vd(self) -> VarData {
        VarData {
            var_type: self,
            initialized: true,
            mutable: true
        }
    }

    fn to_var_data(self, init: bool, mutable: bool) -> VarData {
        let mut r = self.to_vd();
        r.initialized = init;
        r.mutable = mutable;
        r
    }
}