#[macro_use] extern crate derive_new;
#[macro_use] extern crate derive_getters;
#[macro_use] extern crate lalrpop_util;
extern crate colored;

mod data;
mod parser;
mod checker;
mod translator;
mod err_print;

use std::env;
use std::fs;
use std::path::{Path};

static ERR_EXIT_CODE: i32 = 42;

fn print_errs(code: &String, errs: Vec<data::types::FrontError>) {
    eprintln!("ERROR");
    let printer = err_print::ErrorPrinter::new(code);
    for err in errs {
        printer.print(err.position, err.message);
    }
    std::process::exit(ERR_EXIT_CODE);
}

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("Path argument required");
            return;
        }
    };
    let mut code = fs::read_to_string(&path).unwrap();
    code = parser::lift(&code);

    macro_rules! unpack {
        ($res:expr) => {
            match $res {
                Ok(v) => v,
                Err(errs) => {
                    print_errs(&code, errs);
                    return;
                }
            }
        };
    }

    let ast = unpack!(parser::parse(&code));
    let (attr_ast, funcs) = unpack!(checker::check(ast));
    let generated_code = translator::translate(&attr_ast, funcs);

    let path = Path::new(&path);
    let out = path.with_extension("ll");
    fs::write(out, generated_code).unwrap();
    eprintln!("OK");
}