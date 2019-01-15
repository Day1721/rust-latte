use data::{ast::{Program, TopDefG, Ident, Type}, 
           types::{InternalType, SpecType, FrontResult, FrontError}};
use std::collections::HashMap;

//TODO add stdlib functions to checker
pub fn collect_top_defs(ast: &Program) -> FrontResult<HashMap<Ident, InternalType>> {
    use self::SpecType::*;
    use self::Type::*;
    let mut res: HashMap<Ident, InternalType> = HashMap::new();
    //TODO (opt) use DSL macro for type making :D
    {
        let mut ins = |name: &'static str, t| {
            res.insert(name.to_string(), InternalType::single(t));
        };

        ins("printString", Func(vec![Box::new(Str)], Void ));
        ins("printInt", Func(vec![Box::new(Int)], Void ));
        ins("readInt", Func(vec![], Int));
        ins("readString", Func(vec![], Str));
    }

    collect_top_defs_exact(ast, &mut res)?;
    Ok(res)
}


fn collect_top_defs_exact(ast: &Program, stdlib: &mut HashMap<Ident, InternalType>) -> FrontResult<()> {
    let mut errs = vec![];

    for pos_def in ast {
        let def = &(*pos_def).item;
        let pos = &(*pos_def).position;
        match def {
            TopDefG::Func(name, args, ret_type, _) => {
                let func_t = SpecType::func_type(&args, &ret_type);

                if stdlib.contains_key(name) {
                    let mut t = stdlib.get_mut(name).unwrap(); //None unreachable
                    if t.vec().contains(&func_t) {
                        errs.push(FrontError {
                            position: pos.clone(),
                            message: format!("Redefinition of function {}", name)
                        });
                    } else {
                        t.push(func_t);
                    }
                } else {
                    stdlib.insert(name.clone(), InternalType::single(func_t));
                };
            }
        };
    }

    if errs.len() > 0 {
        Err(errs)
    } else {
        Ok(())
    }
}