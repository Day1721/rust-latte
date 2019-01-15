use data::ast;
use std;
use std::collections::HashMap;

pub fn optimize(ast: &mut ast::AttrProgram) {
    inline_constants(ast);
}

//TODO: after add mid-step to translator, i.e. using simular to LLVM IR, make more advanced optimizations on it 
fn inline_constants(ast: &mut ast::AttrProgram) {
    fn inline_stmt(stmt: &mut ast::AttrStmt, env: &mut InlinerEnv) {
        use self::ast::StmtG::*;
        let item = &mut (*stmt).item;
        match item {
            Block(stmts) => {
                env.up_depth(); 

                let mut continues = true;
                stmts.retain(|stmt| {
                    let ret = continues;
                    continues = continues && !(*stmt).attributes.returns;
                    ret
                });

                for stmt in stmts {
                    inline_stmt(stmt, env)
                }

                env.down_depth();
            },
            Decl(t, items) => {
                use self::ast::DeclItemG::*;
                for decl in items {
                    match decl {
                        NoInit(id) => { 
                            env.new_var(id.clone(), Some((*t).default()));
                        },
                        WithInit(id, expr) => {
                            inline_expr(expr, env);
                            match (*expr).item.as_value() {
                                Some(v) => { env.new_var(id.clone(), Some(v)) },
                                None => ()
                            };
                        }
                    }
                }
            },
            _ => ()
        }
    }

    fn inline_expr(expr: &mut ast::AttrExpr, env: &mut InlinerEnv) {
        use self::ast::ExprG::*;
        use self::std::mem::*;
        let mut replace = Error;
        let item = &mut (*expr).item;
        match item {
            Id(ref id) => match env.get(id) {
                Some(val) => replace = val.to_expr(),
                None => ()
            },
            App(_, args) => {
                for arg in args {
                    inline_expr(arg, env);
                }
            },
            UnaryOper(expr, _) => {
                use self::Value::*;
                inline_expr(expr, env);
                match (*expr).item.as_value() {
                    Some(Int(i)) => replace = ast::ExprG::Int(-i),
                    Some(Bool(b)) => replace = ast::ExprG::Bool(!b),
                    _ => ()
                };
            },
            BinOper(l, oper, r) => {
                use self::Value::*;
                use self::ast::BinOper::*;
                inline_expr(l, env);
                inline_expr(r, env);
                match ((*l).item.as_value(), oper, (*r).item.as_value()) {
                    (Some(Str(ls)), Add, Some(Str(rs))) => replace = ast::ExprG::Str(format!("{}{}", ls, rs)),
                    (Some(Int(li)), Add, Some(Int(ri))) => replace = ast::ExprG::Int(li + ri),
                    (Some(Int(li)), Sub, Some(Int(ri))) => replace = ast::ExprG::Int(li - ri),
                    (Some(Int(li)), Mul, Some(Int(ri))) => replace = ast::ExprG::Int(li * ri),
                    (Some(Int(li)), Div, Some(Int(ri))) => replace = ast::ExprG::Int(li / ri),
                    (Some(Int(li)), Mod, Some(Int(ri))) => replace = ast::ExprG::Int(li % ri),
                    (Some(Int(li)), Eq, Some(Int(ri))) => replace = ast::ExprG::Bool(li == ri),
                    (Some(Str(ls)), Eq, Some(Str(rs))) => replace = ast::ExprG::Bool(ls == rs),
                    (Some(Bool(lb)), Eq, Some(Bool(rb))) => replace = ast::ExprG::Bool(lb == rb),
                    (Some(Int(li)), NE, Some(Int(ri))) => replace = ast::ExprG::Bool(li != ri),
                    (Some(Str(ls)), NE, Some(Str(rs))) => replace = ast::ExprG::Bool(ls != rs),
                    (Some(Bool(lb)), NE, Some(Bool(rb))) => replace = ast::ExprG::Bool(lb != rb),
                    (Some(Int(li)), GT, Some(Int(ri))) => replace = ast::ExprG::Bool(li > ri),
                    (Some(Int(li)), LT, Some(Int(ri))) => replace = ast::ExprG::Bool(li < ri),
                    (Some(Int(li)), GE, Some(Int(ri))) => replace = ast::ExprG::Bool(li >= ri),
                    (Some(Int(li)), LE, Some(Int(ri))) => replace = ast::ExprG::Bool(li <= ri),
                    (Some(Bool(lb)), And, Some(Bool(rb))) => replace = ast::ExprG::Bool(lb && rb),
                    (Some(Bool(lb)), Or, Some(Bool(rb))) => replace = ast::ExprG::Bool(lb || rb),
                    _ => ()
                }
            },
            _ => ()
        }

        match replace {
            Error => (),
            _ => swap(item, &mut replace)
        };
    }

    for td in ast.iter_mut() {
        use self::ast::TopDefG::*;
        let item = &mut (*td).item;
        match item {
            Func(_, _, _, ref mut stmt) => inline_stmt(stmt, &mut InlinerEnv::new())
        };
    }

    struct InlinerEnv {
        envs: Vec<HashMap<String, Option<Value>>>,
        depth: usize
    }

    impl InlinerEnv {
        fn new() -> InlinerEnv {
            InlinerEnv {
                envs: vec![],
                depth: 0
            }
        }

        pub fn get(&self, id: &String) -> Option<Value> {
            self.envs[self.depth-1].get(id).unwrap_or(&None).clone()
        }

        pub fn new_var(&mut self, id: String, val: Option<Value>) {
            if self.envs.len() < self.depth {
                self.envs.push(HashMap::new());
            }
            
            self.envs[self.depth-1].insert(id, val);
        }

        pub fn up_depth(&mut self) {
            self.depth += 1;
        }
        
        pub fn down_depth(&mut self) {
            self.depth -= 1;
            if self.envs.len() >= self.depth {
                self.envs.pop();
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Value {
    Int(i32),
    Str(String),
    Bool(bool)
    // Variable(String)
}

impl Value {
    fn to_expr<A>(self) -> ast::ExprG<A> {
        use self::Value::*;
        match self {
            Int(i) => ast::ExprG::Int(i),
            Str(s) => ast::ExprG::Str(s),
            Bool(b) => ast::ExprG::Bool(b)
        }
    }
}

impl ast::Type {
    fn default(&self) -> Value {
        use self::ast::Type::*;
        match self {
            Int => Value::Int(0),
            Str => Value::Str("".to_string()),
            Bool => Value::Bool(false),
            _ => unreachable!()
            // Arr(_) => 
        }
    }
}

impl<A> ast::ExprG<A> {
    fn as_value(&self) -> Option<Value> {
        use self::ast::ExprG::*;
        match self {
            Int(i) => Some(Value::Int(*i)),
            Str(s) => Some(Value::Str(s.clone())),
            Bool(b) => Some(Value::Bool(*b)),
            _ => None
        }
    }
}