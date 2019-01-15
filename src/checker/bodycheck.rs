use data::ast;
use data::ast::Attribute;
use data::types::*;
use std::collections::HashMap;
use std;
use super::{CheckerEnv, ToVarData};

pub fn check_map_program(mut ast: ast::Program, env: &mut HashMap<String, InternalType>) -> FrontResult<ast::AttrProgram> {
    fn map_stmt(stmt_box: ast::Statement) -> ast::AttrStmt {
        use self::ast::StmtG::*;
        let stmt = *stmt_box;
        let mut attrs = ast::StmtAttr::new();
        let res = match stmt.item {
            Block(stmts) => { 
                let mapped: Vec<ast::AttrStmt> = stmts.into_iter().map(|s|map_stmt(s)).collect();
                attrs.returns = mapped.iter().any(|s| (**s).attributes.returns);
                Block(mapped)
            },
            Decl(t, items) => {
                use self::ast::DeclItemG::*;
                let mapped_items = items.into_iter().map(|item|match item {
                    NoInit(id) => NoInit(id),
                    WithInit(id, expr) => {
                        let mapped_expr = map_expr(expr);
                        WithInit(id, mapped_expr)
                    }
                }).collect(); 
                Decl(t, mapped_items)
            },
            Ass(id, e) => Ass(id, map_expr(e)),
            Inc(id) => Inc(id),
            Dec(id) => Dec(id),
            Ret(v) => {
                attrs.returns = true;
                Ret(v.map(|e|map_expr(e)))
            },
            If(cond, then, else_opt) => {
                let mapped_cond = map_expr(cond);
                let then_mapped = map_stmt(then);
                let else_mapped_opt = else_opt.map(|els|map_stmt(els));
                attrs.returns = (*then_mapped).attributes.returns;
                attrs.returns &= match &else_mapped_opt {
                    Some(v) => (**v).attributes.returns,
                    None => false
                };
                If(mapped_cond, then_mapped, else_mapped_opt)
            },
            While(cond, stmt) => {
                let mapped_cond = map_expr(cond);
                let stmt_mapped = map_stmt(stmt);
                attrs.returns = match as_bool(&mapped_cond) {
                    Some(true) => true,
                    _ => (*stmt_mapped).attributes.returns
                };
                While(mapped_cond, stmt_mapped)
            },
            Expr(e) => Expr(map_expr(e)),
            Error => Error
        };

        fn map_expr(expr_box: ast::Expression) -> ast::AttrExpr {
            // use self::ast::ExprG::*;
            // let expr = *expr_box;
            // let mut attrs = ast::ExprAttr::new();
            // let res = match expr.item {
                
            // };
            // unimplemented!()
            expr_box
        }


        let mut with_pos = ast::WithPosition::new_pos(stmt.position, res);
        with_pos.attributes = attrs;
        Box::new(with_pos)
    }

    let env_val = std::mem::replace(env, HashMap::new());
    check_optimize_stmts(&mut ast, env_val)?;
    *env = super::topdefs::collect_top_defs(&ast)?;
    Ok(ast.into_iter().map(|td_box| {
        use self::ast::TopDefG::*;
        let td = *td_box;
        match td.item {
            Func(id, args, res, stmt_box) => Box::new(ast::WithPosition::new_pos(td.position, Func(id, args, res, map_stmt(stmt_box))))
        }
    }).collect())
}

pub fn check_optimize_stmts(ast: &mut ast::Program, env: HashMap<String, InternalType>) -> FrontResult<()> {
    let mut errs = vec![];

    let mut new_env = CheckerEnv::new();
    for (k,v) in env {
        new_env.insert(k, v.to_var_data(true, false));
    }
    
    for stmt in ast {
        check_top_def(&mut stmt.as_mut().item, new_env.clone()).move_err_to(&mut errs);
    }

    FrontResult::mk_err(errs)
}

fn check_top_def(top_def: &mut ast::TopDef, mut env: CheckerEnv) -> FrontResult<()> {
    use self::ast::TopDefG::*;

    fn update_env(env: &mut CheckerEnv, args: &Vec<ast::Argument>) -> Result<(), FrontError> {
        let mut names = vec![];
        for arg in args {
            let ref name = (**arg).item.name;
            if names.contains(name) {
                return Err(FrontError {
                    position: (**arg).position.clone(),
                    message: redefinition(name)
                })
            }
            names.push(name.clone());
            env.insert(name.clone(), (**arg).item.ltype.clone().to_internal_type().to_vd());
        }
        Ok(())
    }

    match top_def {
        Func(id, args, res, stmt) => {
            update_env(&mut env, args).map_err(|e|vec![e])?;
            let t: ast::Type = *res.clone();
            let mut checker = StmtChecker::new(env, t);
            let status = checker.check(stmt)?;

            normalize_name(id, args.iter().map(|arg| &(*(**arg).item.ltype)).collect());

            if **res == ast::Type::Void {
                return Ok(());
            }

            // {
            //     let args_types = args.iter().map(|arg| (**arg).item.ltype.as_ref().stringify()).collect::<Vec<String>>().join(".")
            //     *id = format!("{}..{}", id, args_types);
            // }

            match status.non_ret_position {
                None => Ok(()),
                Some(pos) => Err(vec![FrontError {
                    position: pos,
                    message: "No return path detected".to_string()
                }])
            }
        }
    }
}

#[derive(Clone)]
struct StmtChecker {
    env: CheckerEnv,
    ret_type: ast::Type,
    local_env: Vec<String>
}

fn invalid_type(expected: &InternalType, got: &InternalType) -> String {
    format!("Invalid type:\nexpected: {},\n got:      {}", expected, got)
}

fn redefinition(id: &str) -> String {
    format!("Redefinition of variable {}", id)
}

fn as_bool<A>(expr: &ast::ExpressionG<A>) -> Option<bool> {
    use self::ast::ExprG::*;
    match &(*expr).item {
        Bool(b) => Some(*b),
        _ => None
    }
}

impl StmtChecker {
    pub fn new(env: CheckerEnv, ret_type: ast::Type) -> StmtChecker {
        StmtChecker {
            env: env,
            ret_type: ret_type,
            local_env: vec![]
        }
    }

    pub fn check(&mut self, stmt: &mut ast::Statement) -> FrontResult<CheckStatus> {
        use self::ast::StmtG::*;
        let pos = (*stmt).position.clone();
        let mut res = CheckStatus::empty();
        res.non_ret_position = Some(pos.clone());
        let ref mut item = (*stmt).item;

        macro_rules! inc_dec_handle {
            ($id:ident) => {{
                let t = self.get_type($id, &pos, true).map_err(|e|vec![e])?;
                let int_t = ast::Type::Int.to_internal_type();
                if t != int_t {
                    return Err(vec![FrontError {
                        position: pos,
                        message: invalid_type(&int_t, &t)
                    }])
                } else {
                    Ok(())
                }
            };
        }}

        let mut skip = ast::Stmt::skip();
        let mut replace = None;

        match item {
            Block(stmts) => {
                let old_local = std::mem::replace(&mut self.local_env, vec![]);
                for stmt in stmts {
                    let status = self.check(stmt)?;
                    if res.non_ret_position.is_some() {
                        res.non_ret_position = status.non_ret_position;
                    }
                }
                self.local_env = old_local;
            },
            Decl(t, decls) => {
                use self::ast::DeclItemG::*;
                for decl in decls {
                    match decl {
                        NoInit(id) => self.env.insert(id.clone(), t.clone().to_internal_type().to_var_data(false, true)),
                        WithInit(id, e) => { 
                            let e_t = self.expr_type(e)?;
                            let t_intern = (**t).clone().to_internal_type();
                            if !e_t.assignable_to(&t_intern) {
                                return Err(vec![FrontError{
                                    position: (*e).position.clone(),
                                    message: invalid_type(&t_intern, &e_t)
                                }])
                            }
                            self.env.insert(id.clone(), t.clone().to_internal_type().to_var_data(true, true)) 
                        }
                    };
                    let id = decl.get_ident();
                    if self.local_env.contains(&id) {
                        return Err(vec![FrontError {
                            position: pos,
                            message: redefinition(id)
                        }]);
                    }
                    self.local_env.push(id.clone());
                }
            },
            Ass(id, e) => {
                let expr_t = self.expr_type(e)?;
                let mut id_data = match self.env.get_mut(id) {
                    Some(t) => t,
                    None => return Err(vec![FrontError {
                        position: pos,
                        message: "Use of undefined value".to_string()
                    }])
                };
                id_data.initialized = true;
                let id_t = id_data.var_type.clone();
                if !expr_t.assignable_to(&id_t) {
                    return Err(vec![FrontError {
                        position: pos,
                        message: format!("Assignment of incompatible types: \nlvalue of type {},\nrvalue of type {}", id_t, expr_t)
                    }])
                };
            },
            Inc(id) => {
                let r: Result<(),FrontError> = inc_dec_handle!(id);
                r.map_err(|err|vec![err])?;
            },
            Dec(id) => {
                let r: Result<(),FrontError> = inc_dec_handle!(id);
                r.map_err(|err|vec![err])?;
            },
            Ret(opt) => {
                use self::ast::Type::*;
                res.non_ret_position = None;
                match (opt, &self.ret_type) {
                    (None, Void) => (),
                    (Some(_), Void) => return Err(vec![FrontError {
                        position: pos,
                        message: "Return value from void function".to_string()
                    }]),
                    (Some(got_expr), expect) => {
                        let got_t = self.expr_type(got_expr)?;
                        let expect_t = expect.clone().to_internal_type();
                        if !got_t.assignable_to(&expect_t) {
                            return Err(vec![FrontError {
                                position: pos,
                                message: invalid_type(&expect_t, &got_t)
                            }])
                        }
                    },
                    (None, _) => return Err(vec![FrontError {
                        position: pos,
                        message: "Value expected".to_string()
                    }]),
                };
            },
            If(cond, then, else_opt) => {
                let cond_t = self.expr_type(cond)?;
                let bool_t = ast::Type::Bool.to_internal_type();
                if !cond_t.assignable_to(&bool_t) {
                    return Err(vec![FrontError {
                        position: pos,
                        message: invalid_type(&bool_t, &cond_t)
                    }])
                }

                let mut then_checker = self.clone();
                let then_status = then_checker.check(then)?;
                let mut else_checker = self.clone();
                let else_status = lift(else_opt.as_mut().map(|else_br| else_checker.check(else_br)))?;

                res.non_ret_position = match as_bool(&cond) {
                    Some(true) => then_status.non_ret_position,
                    Some(false) => match else_status {
                        Some(status) => status.non_ret_position,
                        None => Some(pos)
                    },
                    None => match else_status {
                        Some(status) => then_status.non_ret_position.or(status.non_ret_position),
                        None => then_status.non_ret_position.or(Some(pos))
                    }
                };

                let mut tmp = ast::StmtG::Error;
                match as_bool(&cond) {
                    Some(true) => std::mem::swap(&mut (*then).item, &mut tmp),
                    Some(false) => match else_opt {
                        Some(stmt) => std::mem::swap(&mut (*stmt).item, &mut tmp),
                        None => std::mem::swap(&mut skip, &mut tmp)
                    },
                    None => ()
                };

                replace = match tmp {
                    ast::StmtG::Error => None,
                    tmp => Some(tmp)
                };

                self.merge_env(vec![then_checker,else_checker]);
            },
            While(cond, stmt) => {
                let cond_t = self.expr_type(cond)?;
                let bool_t = ast::Type::Bool.to_internal_type();
                if !cond_t.assignable_to(&bool_t) {
                    return Err(vec![FrontError {
                        position: pos,
                        message: invalid_type(&bool_t, &cond_t)
                    }])
                }

                let status = self.check(stmt)?;
                res.non_ret_position = match as_bool(&cond) {
                    Some(false) => Some(pos),
                    Some(true) => None,
                    None => status.non_ret_position 
                };
            },
            Expr(expr) => {
                self.expr_type(expr)?;
            }
            _ => ()
        };

        match replace {
            Some(mut stmt) => std::mem::swap(item, &mut stmt),
            None => ()
        };

        Ok(res)
    }

    fn expr_type(&self, expr: &mut ast::Expression) -> FrontResult<InternalType> {
        use self::ast::ExprG::*;
        let pos = (*expr).position.clone();
        // let mut errs = vec![]; 

        match &mut (**expr).item {
            Id(id) => self.get_type(id, &pos, false).map_err(|e|vec![e]),
            Int(_) => Ok(ast::Type::Int.to_internal_type()),
            Bool(_) => Ok(ast::Type::Bool.to_internal_type()),
            Str(_) => Ok(ast::Type::Str.to_internal_type()),
            App(id, args) => {
                let func_t = self.get_type(id, &pos, true).map_err(|e|vec![e])?;
                let args_res_t: Vec<FrontResult<InternalType>> = args.iter_mut().map(|arg|self.expr_type(arg)).collect();
                let args_t = aggregate_err(args_res_t)?;
                
                let mut candidates: Vec<SpecType> = vec![];
                for simple_t in func_t.vec() {
                    use self::SpecType::*;
                    match simple_t {
                        Func(args, _) => {
                            if args.len() != args_t.len() {
                                continue;
                            }
                            let mut it = args.iter().zip(args_t.iter());
                            let all_ok = it.all(|(expected, given)| {
                                let expect = (*expected).clone().to_internal_type();
                                given.assignable_to(&expect)
                            });
                            if all_ok {
                                candidates.push(simple_t.clone());
                            };
                        },
                        Type(_) => ()
                    };
                }
                match candidates.len() {
                    0 => Err(vec![FrontError {
                        position: pos,
                        message: format!("no matching type found for {}", id)
                    }]),
                    1 => { 
                        let (args, ret) = match &candidates[0] {
                            SpecType::Func(args, ret) => (args, ret),
                            SpecType::Type(_) => unreachable!()
                        };
                        normalize_name(id, args.iter().map(|boxed| boxed.as_ref()).collect());
                        Ok(ret.clone().to_internal_type()) 
                    }
                    _ => Err(vec![FrontError {
                        position: pos,
                        message: format!("ambigous call for {}", id)
                    }])
                }
            },
            UnaryOper(expr, oper) => {
                let oper_t = oper.get_arg_type();
                let expr_t = self.expr_type(expr)?;
                if expr_t.assignable_to(&oper_t) {
                    Ok(oper.get_ret_type())
                } else {
                    Err(vec![FrontError {
                        position: pos,
                        message: invalid_type(&oper_t, &expr_t)
                    }])
                }
            },
            BinOper(l, oper, r) => {
                let l_t = self.expr_type(l)?;
                let r_t = self.expr_type(r)?;
                let oper_t = oper.get_arg_type();
                let mut errs = vec![];

                if !l_t.assignable_to(&oper_t) {
                    errs.push(FrontError {
                        position: (**l).position.clone(),
                        message: invalid_type(&oper_t, &l_t)
                    });
                }
                // if !r_t.assignable_to(&oper_t) {
                if !l_t.assignable_to(&r_t) {
                    errs.push(FrontError {
                        position: (**r).position.clone(),
                        message: invalid_type(&oper_t, &r_t)
                    });
                }

                if errs.len() > 0 {
                    Err(errs)
                } else {
                    Ok(oper.get_ret_type())
                }
            }
            Error => panic!("Unreachable"),
        }
    }

    fn get_type(&self, id: &str, pos: &Position, should_init: bool) -> Result<InternalType, FrontError> {
        match self.env.get(id) {
            Some(data) => if should_init && !data.initialized {
                Err(FrontError {
                    position: pos.clone(),
                    message: format!("Use of non-initialized variable: {}", id)
                })
            } else {
                Ok(data.var_type.clone())
            },
            None => Err(FrontError {
                position: pos.clone(),
                message: format!("use of undefined variable: {}", id)
            })
        }
    }
    
    fn merge_env(&mut self, other: Vec<StmtChecker>) {
        for (k, v) in self.env.iter_mut() {
            v.initialized = other.iter().all(|checker|checker.env.get(k).map(|v2|v2.initialized).unwrap_or(false))
        }
    }
}

struct CheckStatus {
    non_ret_position: Option<Position>
}

impl CheckStatus {
    fn empty() -> CheckStatus {
        CheckStatus {
            non_ret_position: None
        }
    }
}


trait AssingCheckable {
    fn assignable_to(&self, to: &Self) -> bool;
}

impl AssingCheckable for InternalType {
    fn assignable_to(&self, to: &InternalType) -> bool {
        self.vec().iter().any(|from_spec|to.vec().iter().any(|to_spec|from_spec.assignable_to(to_spec)))
        // self.vec().iter().zip(to.vec().iter()).all(|(from, to)|from.assignable_to(to))
    }
}

impl AssingCheckable for SpecType {
    fn assignable_to(&self, to: &SpecType) -> bool {
        use self::SpecType::*;
        match (self, to) {
            (Type(from), Type(to)) => from.assignable_to(to),
            (Func(_from_args, _from_res), Func(_to_args, _to_res)) => true, //TODO add checking
            _ => false
        }
    }
}

impl AssingCheckable for ast::Type {
    fn assignable_to(&self, to: &ast::Type) -> bool {
        self == to
    }
}

fn aggregate_err<V,E>(res: Vec<Result<V,Vec<E>>>) -> Result<Vec<V>, Vec<E>> {
    let mut succs = vec![];
    let mut errs = vec![];
    for result in res {
        match result {
            Ok(v) => succs.push(v),
            Err(mut e) => errs.append(&mut e)
        };
    };
    if errs.len() > 0 { 
        Err(errs) 
    } else {
        Ok(succs)
    }
}


fn lift<R,E>(opt: Option<Result<R,E>>) -> Result<Option<R>,E> {
    match opt {
        Some(res) => match res {
            Ok(v) => Ok(Some(v)),
            Err(err) => Err(err)
        },
        None => Ok(None)
    }
}

fn normalize_name(id: &mut String, args: Vec<&ast::Type>) {
    if super::STDLIB.iter().all(|f| f.to_string() != *id) {
        let args_types = args.iter().map(|arg| arg.stringify()).collect::<Vec<String>>().join(".");
        *id = format!("{}..{}", id, args_types);
    }
}

impl ast::Type {
    fn stringify(&self) -> String {
        use self::ast::Type::*;
        match self {
            Int => "I".to_string(),
            Str => "S".to_string(),
            Bool => "B".to_string(),
            Void => "V".to_string(),
            Arr(t) => format!("A{}", t)
        }
    }
}