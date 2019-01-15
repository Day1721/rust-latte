lalrpop_mod!(latte, "/parser/latte.rs");

mod test;
mod latte_lalrpop;
use lalrpop_util;
use super::data::{ast, types::{FrontResult, FrontError, Position}};
use std::fmt::Write;

//1. \t -> ""*4
//2. replace comments by spaces (but \n by \n)
//3. @L & @R for error messages (Vec<{String, Span}>) 
//4. forbid method hiding/redefition
//5. move ast to separate mod
//6. enum X { Y { z }}
//7. Use ItemWithSpan<Expr<T>> instead of Expr<T>

static TAB_SIZE: usize = 4;

enum Comment {
    Line,
    Block,
    None
}

// return FrontendResult<String>
fn remove_comments(code: &str) -> FrontResult<String> {
    use self::Comment::*;
    let mut res = "".to_string();
    let mut prev_char = '\0';
    let mut com_type = None;
    let mut reminding = false;
    let mut in_str = false;
    for c in code.chars() {
        if reminding {
            reminding = false;
        } else {
            match (in_str, prev_char, c, com_type) {
                (false, '/', '/', None) => {
                    com_type = Line;
                    write!(&mut res, " ");
                },
                (false, '/', '*', None) => {
                    com_type = Block;
                    write!(&mut res, " ");
                }
                (false, '\0', _, _) => {
                    com_type = None;
                },
                (true, '\\', '"', None) => {
                    com_type = None;
                    write!(&mut res, "{}", prev_char);
                },
                (_, prev_char, '"', None) => {
                    com_type = None;
                    in_str = !in_str;
                    write!(&mut res, "{}", prev_char);
                },
                (_, prev_char, _, None) => {
                    com_type = None;
                    write!(&mut res, "{}", prev_char);
                },
                (_, '*', '/', Block) => {
                    reminding = true;
                    com_type = None;
                    write!(&mut res, "  ");
                }
                (_, '\n', _, Line) => {
                    com_type = None;
                    write!(&mut res, "\n");
                }
                (_, '\n', _, Block) => {
                    com_type = Block;
                    write!(&mut res, "\n");
                },
                (_, _, _, ct) => {
                    write!(&mut res, " ");
                    com_type = ct;
                }
            }
        }
        prev_char = c;
    }
    match com_type {
        Block => Err(vec![FrontError::new(res.len() - 1, res.len() - 1, "No closing block comment detected")]),
        Line => {
            write!(&mut res, " ");
            Ok(res.to_string())
        },
        None => {
            write!(&mut res, "{}", prev_char);
            Ok(res.to_string())
        }
    }
}

pub fn lift(code: &str) -> String {
    code.replace("\t", &" ".repeat(TAB_SIZE))
} 

pub fn parse(contents: &str) -> FrontResult<ast::Program> {
    let no_coments = remove_comments(contents)?; // necesssart cause Lalrpop doesn't supports comments yet

    let parser = latte::ProgramParser::new();
    let mut errors = Vec::new();
    let res = parser.parse(&mut errors, &no_coments);
    match res {
        Ok(ast) => {
            if errors.len() > 0 {
                Err(errors)
            } else {
                Ok(ast)
            }
        },
        Err(err) => match err {
            lalrpop_util::ParseError::InvalidToken { location } => Err(vec![FrontError{
                position: Position::new(location, location),
                message: "Invalid token".to_string()
            }]),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => match token {
                Some((b, t, e)) => Err(vec![FrontError {
                    position: Position::new(b, e),
                    message: format!("Unrecognized token:\nexpected: {},\ngot:      {}", to_str(expected), t)
                }]),
                None => panic!("(maybe) unreachable (no unrecognized token?)") // TODO inspect this
            },
            lalrpop_util::ParseError::ExtraToken { token: (l, t, r) } => Err(vec![FrontError {
                position: Position::new(l, r),
                message: format!("Unexpected token detected: {}", t)
            }]),
            lalrpop_util::ParseError::User { error: _ } => panic!("Unreachable (no user errors)")
        }// println!("{}", err)
    }
}

fn to_str(vec: Vec<String>) -> String {
    let mut res = "".to_string();
    if vec.len() == 0 {
        return res;
    }
    let mut iter = vec.iter();
    write!(&mut res, "{}", iter.next().unwrap());
    loop {
        let v = match iter.next() {
            Some(x) => x,
            None => break
        };
        write!(&mut res, ", {}", v);
    };
    res
}