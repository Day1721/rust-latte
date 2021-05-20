use data::ast::{Expr, ExprG, Expression, BinOper, UnaryOper};
use data::types::{FrontError};

// #[allow(dead_code)]
//
// prop stałych i zmiennych
// wspólne podwyrażenia
// coś z pętlami (czyli np. zmienne indukcyjne)
//
pub fn make_oper(l: Expression, op: BinOper, r: Expression) -> Result<Expr, FrontError> {
    use self::ExprG::*;
    use self::BinOper::*;

    {
        let ref l_item = (*l).item;
        let ref r_item = (*r).item;

        match (l_item, op, r_item) {
            (Str(l), Add, Str(r)) => return Ok(Str(format!("{}{}", l, r))),
            (Int(i), Add, Int(j)) => return Ok(Int(i+j)),
            (Int(i), Sub, Int(j)) => return Ok(Int(i-j)),
            (Int(i), Mul, Int(j)) => return Ok(Int(i*j)),
            (Int(i), Div, Int(j)) => { 
                return if j.clone() == 0 {
                    Err(FrontError {
                        position: (*r).position.clone(),
                        message: "Division by zero detected".to_string()
                    })
                } else {
                    Ok(Int(i/j))
                }
            },
            (Int(i), Mod, Int(j)) => {
                return if j.clone() == 0 {
                    Err(FrontError {
                        position: (*r).position.clone(),
                        message: "Division by zero detected".to_string()
                    })
                } else {
                    Ok(Int(i%j))
                }
            },
            (Int(i), LE, Int(j)) => return Ok(Bool(i <= j)),
            (Int(i), GE, Int(j)) => return Ok(Bool(i >= j)),
            (Int(i), LT, Int(j)) => return Ok(Bool(i < j)),
            (Int(i), GT, Int(j)) => return Ok(Bool(i > j)),
            (Int(i), Eq, Int(j)) => return Ok(Bool(i == j)),
            (Int(i), NE, Int(j)) => return Ok(Bool(i != j)),
            (Bool(b), Eq, Bool(c)) => return Ok(Bool(b == c)),
            (Bool(b), NE, Bool(c)) => return Ok(Bool(b != c)),
            (_, _, _) => ()
        };
    }

    Ok(BinOper(l, op, r))
}

pub fn make_unary(op: UnaryOper, e: Expression) -> Expr {
    use self::ExprG::*;
    use self::UnaryOper::*;
    {
        let ref e_item = (*e).item;
        match (op, e_item) {
            (Neg, Int(i)) => return Int(-i),
            (Not, Bool(b)) => return Bool(!b),
            (_, _) => ()
        }
    }
    
    UnaryOper(e, op)
}

pub fn as_str(s: &str) -> String {
    let mut res = s.to_string();
    res.remove(0);
    res.pop();
    res
}

//TODO move it to return checker because of undefined variable in dead branch
// pub fn parse_if(cond: Box<Expr>, then: Box<Stmt>, opt_else: Option<Box<Stmt>>) -> Box<Stmt> {
//     use self::Expr::*;
//     match *cond {
//         Bool(true) => then,
//         Bool(false) => match opt_else {
//             Some(els) => els,
//             None => Box::new(Stmt::Skip)
//         },
//         _ => Box::new(Stmt::If(cond, then, opt_else))
//     }
// }
