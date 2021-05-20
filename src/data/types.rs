use std::fmt::{Formatter, Error, Display, Debug};
use super::ast::{Type, SimpleType, Argument, BinOper, UnaryOper};

#[derive(new, Debug, Clone, Getters)]
pub struct Position { 
    l: usize,
    r: usize,
}

pub type FrontResult<T> = Result<T,Vec<FrontError>>;

pub trait ErrMoving<E, ECol> {
    fn move_err_to(self, err: &mut ECol);
    fn mk_err(err: E) -> Self;
}

impl ErrMoving<Vec<FrontError>,Vec<FrontError>> for FrontResult<()> {
    fn move_err_to(self, err: &mut Vec<FrontError>) {
        match self {
            Err(mut e) => err.append(&mut e),
            _ => ()
        }
    }

    fn mk_err(errs: Vec<FrontError>) -> FrontResult<()> {
        if errs.len() > 0 {
            Err(errs)
        } else {
            Ok(())
        }
    }
}

impl ErrMoving<FrontError,Vec<FrontError>> for Result<(),FrontError> {
    fn move_err_to(self, err: &mut Vec<FrontError>) {
        match self {
            Err(e) => err.push(e),
            _ => ()
        }
    }

    fn mk_err(errs: FrontError) -> Result<(), FrontError> {
        Err(errs)
    }
}


pub trait ResultMoving<VCol, ECol> where {
    fn move_to(self, vals: &mut VCol, errs: &mut ECol);
}

impl<V,E> ResultMoving<Vec<V>, Vec<E>> for Result<V,E> {
    fn move_to(self, vals: &mut Vec<V>, errs: &mut Vec<E>) {
        match self {
            Ok(v) => vals.push(v),
            Err(e) => errs.push(e)
        }
    }
}

impl<V,E> ResultMoving<Vec<V>, Vec<E>> for Result<V,Vec<E>> {
    fn move_to(self, vals: &mut Vec<V>, errs: &mut Vec<E>) {
        match self {
            Ok(v) => vals.push(v),
            Err(mut e) => errs.append(&mut e)
        }
    }
}


#[derive(Debug)]
pub struct FrontError {
    pub position: Position,
    pub message: String
}

impl FrontError {
    pub fn new(l: usize, r: usize, message: &str) -> FrontError {
        FrontError {
            message: message.to_string(),
            position: Position::new(l, r)
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct InternalType(Vec<SpecType>);

impl InternalType {
    pub fn new(types: Vec<SpecType>) -> InternalType {
        InternalType(types)
    }

    pub fn push(&mut self, t: SpecType) {
        let v = self.vec_mut();
        v.push(t);
        v.sort()
    }

    pub fn single(t: SpecType) -> InternalType {
        InternalType(vec![t])
    }

    pub fn vec(&self) -> &Vec<SpecType> {
        let InternalType(v) = self;
        v
    }

    pub fn vec_mut(&mut self) -> &mut Vec<SpecType> {
        let InternalType(v) = self;
        v
    }

    pub fn simple_type(&self) -> Option<Type> {
        let v = self.vec();
        if v.len() != 1 {
            return None;
        };
        match &v[0] {
            SpecType::Type(t) => Some(t.clone()),
            _ => None
        }
    }

    pub fn spec_type(&self) -> Option<SpecType> {
        let v = self.vec();
        match &v[0..] {
            [elem] => Some(elem.clone()),
            _ => None
        }
    }
}

impl Display for InternalType {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        let v = self.vec();
        match v.len() {
            0 => (),
            _ => {
                write!(f, "{}", v[0]);
                for arg in v.iter().skip(1) {
                    write!(f, " /\\ {}", arg);
                }
            }
        };
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum SpecType {
    Func(Vec<Box<Type>>, Type),
    Type(Type),
}

impl Display for SpecType {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        match self {
            SpecType::Type(t) => write!(f, "{}", t),
            SpecType::Func(args, result) => {
                write!(f, "(");
                match args.len() {
                    0 => (),
                    _ => {
                        write!(f, "{:?}", args[0]);
                        for arg in args.iter().skip(1) {
                            write!(f, ", {:?}", arg);
                        }
                    }
                };
                write!(f, ") -> {}", result)
            },
        }
    }
}

impl Debug for SpecType {
    fn fmt(&self, f: &mut Formatter) -> Result<(),Error> {
        write!(f, "{}", self)
    }
}

impl SpecType {
    pub fn func_type(params: &Vec<Argument>, result: &Box<Type>) -> SpecType {
        SpecType::Func(
            params.into_iter().map(|a|(**a).item.ltype.clone()).collect(),
            (**result).clone()
        )
    }
}

pub trait ToInternalType {
    fn to_internal_type(self) -> InternalType;
}

impl ToInternalType for Type {
    fn to_internal_type(self) -> InternalType {
        SpecType::Type(self).to_internal_type()
    }
}

impl ToInternalType for SimpleType {
    fn to_internal_type(self) -> InternalType {
        Type::Simple(self).to_internal_type()
    }
}

impl ToInternalType for SpecType {
    fn to_internal_type(self) -> InternalType {
        InternalType::single(self)
    }
}

pub trait Typeable {
    fn get_arg_type(&self) -> InternalType;
    fn get_ret_type(&self) -> InternalType;
}

impl Typeable for UnaryOper {
    fn get_arg_type(&self) -> InternalType {
        use self::UnaryOper::*;
        use self::SimpleType::*;
        match self {
            Not => Bool,
            Neg => Int
        }.to_internal_type()
    }

    fn get_ret_type(&self) -> InternalType {
        self.get_arg_type()
    }
}

impl Typeable for BinOper {
    fn get_arg_type(&self) -> InternalType {
        use self::BinOper::*;
        use self::Type::*;
        use self::SimpleType::*;
        match self {
            Add => InternalType::new(vec![SpecType::Type(Simple(Int)), SpecType::Type(Simple(Str))]),
            Sub => Int.to_internal_type(),
            Mul => Int.to_internal_type(),
            Div => Int.to_internal_type(),
            Mod => Int.to_internal_type(),
            Eq => InternalType::new(vec![SpecType::Type(Simple(Int)), SpecType::Type(Simple(Str)), SpecType::Type(Simple(Bool))]),
            NE => InternalType::new(vec![SpecType::Type(Simple(Int)), SpecType::Type(Simple(Str)), SpecType::Type(Simple(Bool))]),
            LT => Int.to_internal_type(),
            GT => Int.to_internal_type(),
            LE => Int.to_internal_type(),
            GE => Int.to_internal_type(),
            And => Bool.to_internal_type(),
            Or => Bool.to_internal_type()
        }
    }

    fn get_ret_type(&self) -> InternalType {
        use self::BinOper::*;
        use self::SimpleType::*;
        match self {
            Eq => Bool.to_internal_type(),
            NE => Bool.to_internal_type(),
            LT => Bool.to_internal_type(),
            GT => Bool.to_internal_type(),
            LE => Bool.to_internal_type(),
            GE => Bool.to_internal_type(),
            _ => self.get_arg_type()
        }
    }
}