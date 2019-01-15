use std::collections::HashMap;
use std::mem;
use super::ast;

pub fn collect_strings(ast: &ast::AttrProgram) -> HashMap<String, String> {
    let mut collector = StringsCollector::new();
    collector.collect(ast);
    mem::replace(&mut collector.strings, HashMap::new())
}

struct StringsCollector {
    current: usize,
    strings: HashMap<String, String>
}

impl StringsCollector {
    fn new() -> Self {
        StringsCollector {
            current: 0,
            strings: HashMap::new()
        }
    }

    fn collect(&mut self, ast: &ast::AttrProgram) {
        for top_def in ast {
            use self::ast::TopDefG::*;
            match &(**top_def).item {
                Func(_, _, _, stmt) => self.collect_in_stmt(stmt)
            }
        }
    }

    fn collect_in_stmt(&mut self, stmt: &ast::AttrStmt) {
        use self::ast::StmtG::*;
        match &(**stmt).item {
            Block(stmts) => for stmt in stmts {
                self.collect_in_stmt(stmt);
            },
            Decl(_, items) => for item in items {
                use self::ast::DeclItemG::*;
                match item {
                    WithInit(_, expr) => self.collect_in_expr(expr),
                    _ => ()
                }
            },
            Ass(_, expr) => self.collect_in_expr(expr),
            Ret(Some(expr)) => self.collect_in_expr(expr),
            If(cond, then, else_opt) => {
                self.collect_in_expr(cond);
                self.collect_in_stmt(then);
                match else_opt {
                    Some(else_br) => self.collect_in_stmt(else_br),
                    None => ()
                }
            },
            While(cond, stmt) => {
                self.collect_in_expr(cond);
                self.collect_in_stmt(stmt);
            },
            Expr(expr) => self.collect_in_expr(expr),
            _ => ()
        }
    }

    fn collect_in_expr(&mut self, expr: &ast::AttrExpr) {
        use self::ast::ExprG::*;
        match &(**expr).item {
            Str(val) => if !self.strings.contains_key(val) {
                let new_id = format!("__STR__{}", self.current);
                self.current = self.current + 1;
                self.strings.insert(val.clone(), new_id);
            },
            App(_, exprs) => for expr in exprs {
                self.collect_in_expr(expr);
            },
            UnaryOper(expr, _) => self.collect_in_expr(expr),
            BinOper(l, _, r) => {
                self.collect_in_expr(l);
                self.collect_in_expr(r);
            },
            _ => ()
        }
    }
}