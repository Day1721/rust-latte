#[cfg(test)] use super::latte;

#[cfg(test)]
macro_rules! test {
    ($parser: ident |- $input: expr => $output: expr) => {
        let mut errors = Vec::new();
        let res = $parser::new().parse(&mut errors, $input).unwrap();
        let formatted = format!("{:?}", res);
        assert_eq!(formatted, $output);
    };
    ($parser: ident |- $input: expr => _|_) => {
        let mut errors = Vec::new();
        let res = $parser::new().parse(&mut errors, $input);
        assert!(res.is_err());
    }
}

#[test]
fn program_tests() {
    use super::latte::*;
    test!(ProgramParser |- "" => _|_);
    test!(ProgramParser |- "void foo() { }" => "[void foo() { }]");
    test!(ProgramParser |- "void foo() { } void bar() { }" => "[void foo() { }, void bar() { }]");
    // test("2", "[2]")
}
#[test]
fn str_tests() {
    use self::latte::*;
    test!(ExprParser |- r#" "a" "# => r#""a""#);
    test!(ExprParser |- r#""a"+"b""# => r#""ab""#);
}

#[test]
fn int_tests() {
    use self::latte::*;
    test!(ExprParser |- "0" => "0");
    test!(ExprParser |- "2" => "2");
    test!(ExprParser |- "42 + 24 + 55" => "121");
    test!(ExprParser |- "5 + 3 - 2" => "6");
    test!(ExprParser |- "-5" => "-5");
}

#[test]
fn decl_tests() {
    use self::latte::*;
    test!(StmtParser |- "int i;" => "int i;");
    test!(StmtParser |- "int i = 42;" => "int i = 42;");
    test!(StmtParser |- "int i,j = 42;" => "int i, j = 42;");
    test!(StmtParser |- "int ;" => _|_);
}

#[test]
fn if_tests() {
    use self::latte::*;
    test!(StmtParser |- "if (5 == 42) { int i; }" => "if (false) {{ int i; }}");
    test!(StmtParser |- "if (f()) if (g()) f();" => "if (f()) {if (g()) {f();}}");
}

#[test]
fn comments_tests() {
    let res = super::remove_comments("int /* COMMENT */ foo(int j) { // COMMENT\n\treturn 42;\n}").unwrap();
    assert_eq!(res, "int               foo(int j) {           \n\treturn 42;\n}");
}