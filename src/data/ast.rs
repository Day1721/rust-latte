use std::fmt::{Debug, Formatter, Error, Display};
use data::types::Position;

pub struct WithPosition<T, TAttr> {
    pub item: T,
    pub position: Position,
    pub attributes: TAttr
}

pub type ProgramG<TD> = Vec<Box<WithPosition<TD, ()>>>;

pub type Program = ProgramG<TopDef>;

pub enum TopDefG<S> {
    Func(Ident, Vec<Argument>, Box<Type>, S)
}

pub type TopDef = TopDefG<Statement>;

pub type Argument = Box<WithPosition<Arg,()>>;

#[derive(new)]
pub struct Arg {
    pub ltype: Box<Type>,
    pub name: Ident
}


pub type StatementG<S, A> = Box<WithPosition<S,A>>;
pub type Statement = StatementG<Stmt, ()>;

pub enum StmtG<A, E> {
    Block(Vec<StatementG<StmtG<A, E>, A>>),
    Decl(Box<Type>, Vec<DeclItemG<E>>),
    Ass(Ident, E),
    // Skip,
    Inc(Ident),
    Dec(Ident),
    Ret(Option<E>),
    If(E, StatementG<StmtG<A, E>, A>, Option<StatementG<StmtG<A, E>, A>>),
    While(E, StatementG<StmtG<A, E>, A>),
    For(SimpleType, Ident, Ident, StatementG<StmtG<A,E>, A>),
    Expr(E),
    Error
}

pub type Stmt = StmtG<(), Expression>;

pub enum DeclItemG<E> {
    NoInit(Ident),
    WithInit(Ident, E)
}
pub type DeclItem = DeclItemG<Expression>;

pub type ExpressionG<A> = Box<WithPosition<ExprG<A>,A>>;
pub type Expression = ExpressionG<()>;

pub enum ExprG<A> {
    Id(Ident),
    Int(i32),
    Bool(bool),
    Str(String),
    App(Ident, Vec<ExpressionG<A>>),
    UnaryOper(ExpressionG<A>, UnaryOper),
    BinOper(ExpressionG<A>, BinOper, ExpressionG<A>),
    ArrayAccess(Ident, ExpressionG<A>),
    Null(Box<Type>),
    New(Box<Type>),
    Error
}
pub type Expr = ExprG<()>;

#[derive(Copy, Clone)]
pub enum BinOper {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NE,
    LT,
    GT,
    LE,
    GE,
    And,
    Or
}

#[derive(Copy, Clone)]
pub enum UnaryOper {
    Neg,
    Not
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Simple(SimpleType),
    Arr(SimpleType)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum SimpleType {
    Int,
    Str,
    Bool,
    Void
}


impl Stmt {
    pub fn skip<E,A>() -> StmtG<A,E> {
        StmtG::Block(vec![])
    }
}


pub struct StmtAttr {
    pub returns: bool
}

pub struct ExprAttr {
    pub expr_t: Type
}

pub type AttrProgram = ProgramG<AttrTopDef>;
pub type AttrTopDef = TopDefG<AttrStmt>;
pub type AttrStmt = StatementG<StmtG<StmtAttr, AttrExpr>, StmtAttr>;
pub type AttrExpr = ExpressionG<()>;



impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        write!(f, "{:?}", self)
    }
}


pub type Ident = String;

impl <T, A:Attribute> WithPosition<T, A> {
    pub fn new(l: usize, r: usize, item: T) -> WithPosition<T, A> {
        WithPosition::new_pos(Position::new(l, r), item)
    }

    pub fn new_pos(pos: Position, item: T) -> WithPosition<T, A> {
        WithPosition {
            item: item,
            position: pos,
            attributes: A::new()
        }
    }
}

pub trait Attribute {
    fn new() -> Self;
}

impl Attribute for () {
    fn new() -> Self {
        ()
    }
}

impl Attribute for StmtAttr {
    fn new() -> Self {
        StmtAttr {
            returns: false
        }
    }
}

impl Attribute for ExprAttr {
    fn new() -> Self {
        ExprAttr {
            expr_t: Type::Simple(SimpleType::Void)
        }
    }
}

impl <T:Debug, A> Debug for WithPosition<T, A> {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        write!(f, "{:?}", self.item)
    }
}

impl<S:Debug> Debug for TopDefG<S> {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::TopDefG::*;
        match *self {
            Func(ref name, ref args, ref res, ref body) => {
                write!(f, "{:?} {}(", res, name);
                match args.len() {
                    0 => (),
                    _ => {
                        write!(f, "{:?}", args[0]);
                        for arg in args.iter().skip(1) {
                            write!(f, ", {:?}", arg);
                        }
                    }
                };
                write!(f, ") {:?}", body)
            }
        }
    }
}

impl Debug for Arg {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        write!(f, "{:?} {:?}", self.ltype, self.name)
    }
}

impl<A,E:Debug> Debug for StmtG<A,E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::StmtG::*;
        match *self {
            Block(ref stmts) => {
                write!(f, "{{ ");
                for stmt in stmts {
                    write!(f, "{:?} ", stmt);
                }
                write!(f, "}}")
            },
            Decl(ref t, ref decls) => {
                write!(f, "{:?} {:?}", t, decls[0]);
                for decl in decls.iter().skip(1) {
                    write!(f, ", {:?}", decl);
                }
                write!(f, ";")
            },
            // Skip => write!(f, "; "),
            Ass(ref id, ref val) => write!(f, "{} = {:?};", id, val),
            Inc(ref id) => write!(f, "{}++;", id),
            Dec(ref id) => write!(f, "{}--;", id),
            Ret(ref val) => match val {
                None => write!(f, "return;"),
                Some(v) => write!(f, "return {:?};", v)
            },
            If(ref cond, ref then, ref els) => match (&*then,els) {
                (v, None) => write!(f, "if ({:?}) {{{:?}}}", cond, Box::new(v)),
                (v, Some(els)) => write!(f, "if ({:?}) {{{:?}}} else {:?}", cond, Box::new(v), els)
            },
            For(ref t, ref iter, ref arr, ref stmt) => write!(f, "for ({:?} {} : {}) {{{:?}}}", t, iter, arr, stmt),
            While(ref cond, ref stmt) => write!(f, "while ({:?}) {{{:?}}}", cond, stmt),
            Expr(ref e) => write!(f, "{:?};", e),
            Error => write!(f, "#ERROR#")
        }
    }
}

impl<E:Debug> Debug for DeclItemG<E> {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::DeclItemG::*;
        match *self {
            NoInit(ref id) => write!(f, "{}", id),
            WithInit(ref id, ref val) => write!(f, "{} = {:?}", id, val)
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::ExprG::*;
        match *self {
            Id(ref id) => write!(f, "{}", id),
            Int(i) => write!(f, "{}", i),
            Bool(b) => write!(f, "{}", b),
            Str(ref s) => write!(f, r#""{}""#, s),
            App(ref id, ref args) => {
                f.write_str(&id)?;
                f.write_str("(")?;
                match args.len() {
                    0 => (),
                    _ => {
                        write!(f, "{:?}", args[0]);

                        for arg in args.iter().skip(1) {
                            f.write_str(", ")?;
                            write!(f, "{:?}", arg);
                        }
                    }
                };
                f.write_str(")")
            },
            UnaryOper(ref expr, ref oper) => write!(f, "({:?}{:?})", oper, expr),
            BinOper(ref l, o, ref r) => write!(f, "({:?} {:?} {:?})", l, o, r),
            ArrayAccess(ref arr, ref idx) => write!(f, "{}[{:?}]", arr, idx),
            Null(ref t) => write!(f, "({:?})null", t),
            New(ref t) => write!(f, "new {:?}", t),
            Error => write!(f, "#ERROR#")
        }
    }
}

impl Debug for UnaryOper {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::UnaryOper::*;
        write!(f, "{}", match *self {
            Neg => "-",
            Not => "!"
        })
    }
}

impl Debug for BinOper {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::BinOper::*;
        write!(f, "{}", match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => "==",
            NE => "!=",
            LT => "<",
            GT => ">",
            LE => "<=",
            GE => ">=",
            And => "&&",
            Or => "||"
        })
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::Type::*;
        match self {
            Simple(t) => write!(f, "{:?}", t),
            Arr(t) => write!(f, "{:?}[]", t)
        }
    }
}

impl Debug for SimpleType {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        use self::SimpleType::*;
        match *self {
            Int => write!(f, "int"),
            Str => write!(f, "string"),
            Bool => write!(f, "bool"),
            Void => write!(f, "void"),
        }
    }
}

impl DeclItem {
    pub fn get_ident<'a>(&'a self) -> &'a String {
        use self::DeclItemG::*;
        match *self {
            NoInit(ref id) => id,
            WithInit(ref id, _) => id
        }
    }
}
