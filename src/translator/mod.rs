use data::{ast, types};
use data::types::Typeable;
use std::fmt::{Write, Display, Formatter, Error};
use std::collections::HashMap;

mod strings_collect;

pub fn translate(ast: &ast::AttrProgram, funcs: HashMap<String, types::InternalType>) -> String {
    let mut translator = Translator::new();
    translator.translate(ast, funcs);
    translator.output
}

struct Translator {
    output: String,
    reg_cnt: usize,
    current_label: String,
    label_cnt: usize,
    strings: HashMap<String, String>,
    idents: Identifiers,
    func_types: HashMap<String, types::SpecType>
}

impl Translator {
    pub fn new() -> Translator {
        Translator {
            output: "".to_string(),
            reg_cnt: 0,
            current_label: "0".to_string(),
            label_cnt: 0,
            strings: HashMap::new(),
            idents: Identifiers::new(),
            func_types: HashMap::new()
        }
    }

    fn translate(&mut self, ast: &ast::AttrProgram, funcs: HashMap<String, types::InternalType>) {
        self.generate_stdlib();

        self.strings = strings_collect::collect_strings(ast);
        self.func_types = funcs.into_iter().map(|(k,v)| (k, v.spec_type().unwrap())).collect();

        for (val, id) in self.strings.iter() {
            let size = val.len();
            writeln!(self.output, "@{} = private constant [{} x i8] c\"{}\\00\"", id, size+1, val);
        }
        writeln!(self.output, "");

        for top_def in ast {
            self.translate_top_def(top_def);
            writeln!(self.output, "");
        }
    }

    fn translate_top_def(&mut self, top_def: &Box<ast::WithPosition<ast::AttrTopDef, ()>>) {
        use self::ast::TopDefG::*;
        match &(**top_def).item {
            Func(id, args, res, stmt) => {
                let args_repr = args.iter().map(|arg| {
                    let a = &(*arg).item;
                    let t = (*a.ltype).to_llvm_type();
                    format!("{} %{}", t, a.name)
                }).collect::<Vec<String>>().join(", "); //"".to_string();
                
                writeln!(self.output, "define {} @{} ({}) {{", res.to_llvm_type(), id, args_repr);
                
                let start_label = self.new_label();
                self.push_label(&start_label);

                self.push_args(args);
                self.translate_stmt(stmt);
                if !(*stmt).attributes.returns {
                    writeln!(self.output, "ret void");
                }
                writeln!(self.output, "}}");
            }
        }
    }

    fn push_args(&mut self, args: &Vec<ast::Argument>) {
        for arg in args {
            let ref a = (*arg).item;
            let t = (*a.ltype).to_llvm_type();
            let reg_name = {
                let r = self.as_reg_mut(&a.name);
                r.reg_type = t.clone().ptr();
                r.name.clone()
            };

            writeln!(self.output, "{} = alloca {}", reg_name, t);
            writeln!(self.output, "store {} %{}, {}* {}", t, a.name, t, reg_name);
        }
    }

    fn translate_stmt(&mut self, stmt: &ast::AttrStmt) {
        use self::ast::StmtG::*;
        match &(**stmt).item {
            Block(stmts) => {
                self.idents.depth = self.idents.depth + 1;
                for stmt in stmts {
                    self.translate_stmt(stmt);
                }
                self.idents.depth = self.idents.depth - 1;
                self.idents.clean();
            },
            Decl(t, items) => for item in items {
                use self::ast::DeclItemG::*;
                let llvm_t = t.to_llvm_type();

                macro_rules! alloc {
                    ($id:ident) => {{
                        let reg_name = {
                            let r = self.as_reg_mut($id);
                            r.reg_type = (**t).to_llvm_type().ptr();
                            r.name.clone()
                        };
                        writeln!(self.output, "{} = alloca {}", reg_name, llvm_t);
                        reg_name.clone()
                    };
                }}

                match item {
                    NoInit(id) => {
                        let id_name = alloc!(id);
                        writeln!(self.output, "store {} {}, {}* {}", llvm_t, llvm_t.default(), llvm_t, id_name);
                        id_name
                    },
                    WithInit(id, val) => {
                        let reg = self.translate_expr(val);
                        let id_name = alloc!(id);
                        writeln!(self.output, "store {} {}, {}* {}", llvm_t, reg, llvm_t, id_name);
                        id_name
                    }
                };
            },
            Ass(id, expr) => {
                let id_reg = self.clone_reg(id);
                let under_t = id_reg.reg_type.deref();
                let reg = self.translate_expr(expr);
                writeln!(self.output, "store {} {}, {}* {}", under_t, reg, under_t, id_reg.name);
            },
            Inc(id) => {
                let reg = self.load(id);
                let id_reg = self.clone_reg(id);
                let incr_reg = self.new_reg(LlvmType::I32);
                writeln!(self.output, "{} = add i32 {}, 1", incr_reg.name, reg.name);
                writeln!(self.output, "store {} {}, {}* {}", reg.reg_type, incr_reg.name, reg.reg_type, id_reg.name);
            },
            Dec(id) => {
                let reg = self.load(id);
                let id_reg = self.clone_reg(id);
                let incr_reg = self.new_reg(LlvmType::I32);
                writeln!(self.output, "{} = sub i32 {}, 1", incr_reg.name, reg.name);
                writeln!(self.output, "store {} {}, {}* {}", reg.reg_type, incr_reg.name, reg.reg_type, id_reg.name);
            },
            Ret(Some(expr)) => {
                let val = self.translate_expr(expr);
                writeln!(self.output, "ret {} {}", val.get_type(), val);
            },
            Ret(None) => {
                writeln!(self.output, "ret void");
            },
            If(cond, then, else_opt) => {
                let cond_reg = self.translate_expr(cond);
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();

                let then_returns = (*then).attributes.returns;
                let else_returns = else_opt.as_ref().map(|els|(*els).attributes.returns);

                writeln!(self.output, "br i1 {}, label %{}, label %{}", cond_reg, then_label, if else_opt.is_some() { &else_label } else { &end_label });
                self.push_label(&then_label);
                self.translate_stmt(then);
                if !then_returns {
                    writeln!(self.output, "br label %{}", end_label);
                }

                match else_opt {
                    Some(else_br) => {
                        self.push_label(&else_label);
                        self.translate_stmt(else_br);
                        if !else_returns.unwrap() {
                            writeln!(self.output, "br label %{}", end_label);
                        }
                    },
                    None => ()
                };

                match (then_returns, else_returns) {
                    (true, Some(true)) => (),
                    _ => self.push_label(&end_label)
                };
            },
            While(cond, stmt) => {
                let cond_label = self.new_label();
                writeln!(self.output, "br label %{}", cond_label);
                let start_label = self.new_label();
                self.push_label(&start_label);
                self.translate_stmt(stmt);
                writeln!(self.output, "br label %{}", cond_label);
                self.push_label(&cond_label);
                let cond_reg = self.translate_expr(cond);
                let end_label = self.new_label();
                writeln!(self.output, "br i1 {}, label %{}, label %{}", cond_reg, start_label, end_label);
                self.push_label(&end_label);
            },
            Expr(e) => {
                self.translate_expr(e);
            },
            Error => unreachable!()
        }
    }

    fn translate_expr(&mut self, expr: &ast::AttrExpr) -> Value {
        use self::ast::ExprG::*;
        match &(**expr).item {
            Id(id) => self.load(id).as_value(),
            Int(i) => Value::Const(ConstValue::Int(*i)),
            Bool(b) => Value::Const(ConstValue::Bool(*b)),
            Str(lit) => {
                let reg = self.new_reg(LlvmType::str());
                let global_id = self.strings.get(lit).unwrap();
                let len = lit.len();
                writeln!(self.output, "{} = getelementptr [{} x i8], [{} x i8]* @{}, i32 0, i32 0", reg.name, len+1, len+1, global_id);
                reg.as_value()
            },
            App(id, args) => {
                use self::types::SpecType::*;
                let values = args.iter()
                    .map(|arg| self.translate_expr(arg))
                    .map(|val| format!("{} {}", val.get_type(), val))
                    .collect::<Vec<String>>()
                    .join(", ");
                let func_ret = match &self.func_types[id] {
                    Func(_, ret) => ret,
                    Type(_) => unreachable!()
                }.to_llvm_type();

                match func_ret {
                    LlvmType::Void => { 
                        writeln!(self.output, "call {} @{}({})", func_ret, id, values);
                        Value::Const(ConstValue::Void)
                    },
                    t => {
                        let reg = self.new_reg(t.clone());
                        writeln!(self.output, "{} = call {} @{}({})", reg.name, t, id, values);
                        reg.as_value()
                    }
                }
            },
            UnaryOper(expr, oper) => {
                let val = self.translate_expr(expr);
                let reg = self.new_reg(val.get_type());
                use self::ast::UnaryOper::*;
                match oper {
                    Not => { writeln!(self.output, "{} = sub i1 1, {}", reg.name, val); },
                    Neg => { writeln!(self.output, "{} = sub i32 0, {}", reg.name, val); },
                };
                reg.as_value()
            },
            BinOper(l, oper, r) => {
                let l_val = self.translate_expr(l);

                if oper.lazy() {
                    self.translate_lazy(l_val, oper, r)
                } else {
                    let r_val = self.translate_expr(r);
                    self.translate_strict(l_val, oper, r_val)
                }
            },
            Error => unreachable!()
        }
    }

    fn translate_lazy(&mut self, l: Value, oper: &ast::BinOper, r: &ast::AttrExpr) -> Value {
        let start_label = self.current_label.clone();
        let mut r_label = self.new_label();
        let end_label = self.new_label();
        let non_r_val = {
            use self::ast::BinOper::*;
            let (non_r_val, true_label, false_label) = match oper {
                And => ("false", &r_label, &end_label),
                Or => ("true", &end_label, &r_label),
                _ => unreachable!()
            };
            writeln!(self.output, "br i1 {}, label %{}, label %{}", l, true_label, false_label);
            non_r_val
        };

        self.push_label(&r_label);
        let r_reg = self.translate_expr(r);
        r_label = self.current_label.clone();
        writeln!(self.output, "br label %{}", end_label);

        self.push_label(&end_label);
        let res_reg = self.new_reg(LlvmType::I1);
        writeln!(self.output, "{} = phi {} [ {}, %{}], [ {}, %{}]", res_reg.name, res_reg.reg_type, non_r_val, start_label, r_reg, r_label);
        res_reg.as_value()
    }

    fn translate_strict(&mut self, l: Value, oper: &ast::BinOper, r: Value) -> Value {
        use self::ast::BinOper::*;
        let ret_type = match oper.get_ret_type().simple_type() {
            Some(t) => t.to_llvm_type(),
            None => LlvmType::Void
        };
        let mut reg = self.new_reg(ret_type);
        match oper {
            Add => match l.get_type() {
                LlvmType::I32 => { 
                    writeln!(self.output, "{} = add i32 {}, {}", reg.name, l, r);
                    reg.reg_type = LlvmType::I32;
                },
                t => if t.is_str() {
                    writeln!(self.output, "{} = call i8* @concat({} {}, {} {})", reg.name, l.get_type(), l, r.get_type(), r);
                    reg.reg_type = LlvmType::str();
                } else {
                    unreachable!();
                }
            },
            Sub => writeln!(self.output, "{} = sub i32 {}, {}", reg.name, l, r).ignore(),
            Mul => writeln!(self.output, "{} = mul i32 {}, {}", reg.name, l, r).ignore(),
            Div => writeln!(self.output, "{} = sdiv i32 {}, {}", reg.name, l, r).ignore(),
            Mod => writeln!(self.output, "{} = srem i32 {}, {}", reg.name, l, r).ignore(), // TODO calculate mod instead of rem
            Eq => match l.get_type() {
                LlvmType::I1 => writeln!(self.output, "{} = icmp eq i1 {}, {}", reg.name, l, r).ignore(),
                LlvmType::I32 => writeln!(self.output, "{} = icmp eq i32 {}, {}", reg.name, l, r).ignore(),
                t => if t.is_str() {
                    writeln!(self.output, "{} = call i8* @strcmp({},{})", reg.name, l, r).ignore()
                } else {
                    unreachable!()
                }
            },
            NE => match l.get_type() {
                LlvmType::I1 => writeln!(self.output, "{} = icmp ne i1 {}, {}", reg.name, l, r).ignore(),
                LlvmType::I32 => writeln!(self.output, "{} = icmp ne i32 {}, {}", reg.name, l, r).ignore(),
                t => if t.is_str() {
                    let tmp = self.new_reg(LlvmType::I1);
                    writeln!(self.output, "{} = call i8* @strcmp({},{})", tmp.name, l, r);
                    writeln!(self.output, "{} = sub i1 1, {}", reg.name, tmp.name);
                } else {
                    unreachable!()
                }
            },
            LT => self.translate_cmp(&reg, l, "slt", r),
            GT => self.translate_cmp(&reg, l, "sgt", r),
            LE => self.translate_cmp(&reg, l, "sle", r),
            GE => self.translate_cmp(&reg, l, "sge", r),
            _ => unreachable!()
        };
        reg.as_value()
    }

    fn translate_cmp(&mut self, ret: &Register, l: Value, oper: &str, r: Value) {
        writeln!(self.output, "{} = icmp {} i32 {}, {}", ret.name, oper, l, r);
    }

    fn generate_stdlib(&mut self) {
        writeln!(self.output, r#"
declare i32 @readInt()
declare i8* @readString()
declare void @printInt(i32)
declare void @printString(i8*)
declare i8* @concat(i8*, i8*)
        "#);
    //     writeln!(self.output, r#"
    //         declare i32 @printf(i8*, ...)
    //         declare i32 @scanf(i8*, ...)
    //         declare void* @malloc
    //         declare void @free(i8*)

    //         @formatStringInt = private constant [4 x i8] c"%d\0A\OO"
    //         @formatStringStr = private constant [4 x i8] c"%s\0A\OO"
            
    //         define void @printInt(i32 %v) {{
    //             call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStringInt, i32 0, i32 0), i32 %v) 
    //         }}
            
    //         define void @printString(i8* %str) {{
    //             call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStringStr, i32 0, i32 0), i8* %str) 
    //         }}

    //         define i32 readInt() {{
    //             %v = alloca i32
    //             call i32 (i8*, ...) @scanf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStringInt, i32 0, i32 0), i32* %v)
    //             %ret = load i32, i32* %v
    //             ret i32 %ret
    //         }}

    //         define i8* readString() {{
    //             %v = alloca i8*
    //             call i32 (i8*, ...) @scanf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStringStr, i32 0, i32 0), i8** %v)
    //             %ret = load i8*, i8** %v
    //             ret i8* %ret
    //         }}
    //     "#);
    }

    fn load(&mut self, id: &String) -> Register {
        let id_reg = self.clone_reg(id);
        let t = id_reg.reg_type.deref();
        let reg = self.new_reg(t.clone());
        writeln!(self.output, "{} = load {}, {}* {}", reg.name, t, t, id_reg.name);
        reg
    }

    fn new_reg(&mut self, t: LlvmType) -> Register {
        let reg = format!("%r{}", self.reg_cnt);
        self.reg_cnt = self.reg_cnt + 1;
        Register::new(reg, t)
    }

    fn new_label(&mut self) -> String {
        let ret = format!("LABEL_{}", self.label_cnt);
        self.label_cnt = self.label_cnt + 1;
        ret
    }

    fn push_label(&mut self, label: &String) {
        writeln!(self.output, "{}:", label);
        self.current_label = label.clone();
    }

    fn clone_reg(&mut self, id: &String) -> Register {
        // if not found => error
        // if !self.idents.contains_key(id) {
        //     let reg = Register::new(format!("%IDENT_{}", id), LlvmType::Void);
        //     self.idents.insert(id.clone(), reg);
        // }
        self.idents.get(id)
    }

    // fn as_reg(&mut self, id: &String) -> &Register {
    //     if !self.idents.contains_key(id) {
    //         let reg = Register::new(format!("%IDENT_{}", id), LlvmType::Void);
    //         self.idents.insert(id.clone(), reg);
    //     }
    //     self.idents.get(id)
    // }
    


    fn as_reg_mut(&mut self, id: &String) -> &mut Register {
        if !self.idents.contains_key(id) {
            self.idents.new_reg(id);
        }
        self.idents.get_mut(id)
    }
}

struct Identifiers {
    depth: usize,
    counters: HashMap<String, usize>,
    map: HashMap<(String, usize), Register>
}

impl Identifiers {
    fn new() -> Identifiers {
        Identifiers {
            depth: 0,
            counters: HashMap::new(),
            map: HashMap::new()
        }
    }

    fn get(&self, id: &String) -> Register {
        let mut i = self.depth;
        loop {
            if self.map.contains_key(&(id.clone(), i)) {
                break;
            } else {
                i = i - 1;
            }
        }
        self.map.get(&(id.clone(), i)).unwrap().clone()
    }

    fn get_mut(&mut self, id: &String) -> &mut Register {
        let mut i = self.depth;
        loop {
            if self.map.contains_key(&(id.clone(), i)) {
                break;
            } else {
                i = i - 1;
            }
        }
        self.map.get_mut(&(id.clone(), i)).unwrap()
    }

    fn insert(&mut self, id: &String, reg: Register) {
        self.map.insert((id.clone(), self.depth), reg);
    }

    fn contains_key(&self, id: &String) -> bool {
        self.map.contains_key(&(id.clone(), self.depth))
    }

    fn new_reg(&mut self, id: &String) {
        let cnt = match self.counters.get(id) {
            Some(i) => *i,
            None => 0
        };
        if cnt == 0 {
            self.counters.insert(id.clone(), 0);
        }
        match self.counters.get_mut(id) {
            Some(i) => *i = cnt + 1,
            None => unreachable!()
        }

        let ident = format!("%IDENT_{}_{}", cnt, id);
        let reg = Register::new(ident, LlvmType::Void);
        self.insert(id, reg);
    }

    fn clean(&mut self) {
        let depth = self.depth;
        self.map.retain(|(_, d), _| *d <= depth);
    }
}

enum Value {
    Register(Register),
    Const(ConstValue)
}

impl Value {
    fn get_type(&self) -> LlvmType {
        match self {
            Value::Register(reg) => reg.reg_type.clone(),
            Value::Const(val) => val.val_type()
        }
    }
}

enum ConstValue {
    Int(i32),
    Bool(bool),
    Void,
    Null
}

impl Display for ConstValue {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            ConstValue::Int(i) => write!(f, "{}", i),
            ConstValue::Bool(b) => write!(f, "{}", if *b {"true"} else {"false"}),
            ConstValue::Null => write!(f, "null"),
            ConstValue::Void => unreachable!()
        }
    }
}

impl ConstValue {
    fn val_type(&self) -> LlvmType {
        match self {
            ConstValue::Int(_) => LlvmType::I32,
            ConstValue::Bool(_) => LlvmType::I1,
            ConstValue::Void => LlvmType::Void,
            ConstValue::Null => LlvmType::str()
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Value::Register(reg) => write!(f, "{}", reg.name),
            Value::Const(i) => write!(f, "{}", i)
        }
    }
}

#[derive(new, Clone)]
struct Register {
    name: String,
    reg_type: LlvmType
}

impl Register {
    fn as_value(self) -> Value {
        Value::Register(self)
    }
}

#[derive(Clone)]
enum LlvmType {
    Void,
    I32,
    I1,
    I8,
    Ptr(Box<LlvmType>),
    Func(Vec<Box<LlvmType>>, Box<LlvmType>)
}

impl LlvmType {
    fn str() -> LlvmType {
        LlvmType::Ptr(Box::new(LlvmType::I8))
    }

    fn ptr(self) -> LlvmType {
        LlvmType::Ptr(Box::new(self))
    }

    fn deref(self) -> LlvmType {
        match self {
            LlvmType::Ptr(v) => *v,
            _ => { 
                panic!("DEREF OF NON-PTR TYPE: {}", self)
            }
        }
    }

    fn is_str(&self) -> bool {
        match self {
            LlvmType::Ptr(b) => match **b {
                LlvmType::I8 => true,
                _ => false
            },
            _ => false
        }
    }

    fn default(&self) -> Value {
        Value::Const(match self {
            LlvmType::I1 => ConstValue::Bool(false),
            LlvmType::I32 => ConstValue::Int(0),
            LlvmType::Ptr(_) => ConstValue::Null,
            LlvmType::Void => ConstValue::Void,
            LlvmType::I8 => ConstValue::Int(0),
            LlvmType::Func(_, _) => unreachable!()
        })
    }
}

impl Display for LlvmType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::LlvmType::*;
        match self {
            Void => write!(f, "void"),
            I32 => write!(f, "i32"),
            I1 => write!(f, "i1"),
            I8 => write!(f, "i8"),
            Ptr(t) => write!(f, "{}*", t),
            Func(args, res) => {
                let args_types = match args.len() {
                    0 => "".to_string(),
                    _ => {
                        let mut res = format!("{}", args[0]);
                        for arg in args.iter().skip(1) {
                            write!(&mut res, ", {}", arg);
                        }
                        res
                    }
                };
                write!(f, "{} ({})", res, args_types)
            }
        }
    }
}

// impl Register {
//     fn llvm_type(&self) -> String {
//         self.reg_type.to_llvm_type()
//     }
// }

trait TypeConvertable<To> {
    fn to_llvm_type(&self) -> To;
}

impl TypeConvertable<LlvmType> for ast::Type {
    fn to_llvm_type(&self) -> LlvmType {
        use self::ast::Type::*;
        use self::LlvmType::*;
        match self {
            Int => I32,
            ast::Type::Void => LlvmType::Void,
            Bool => I1,
            Str => Ptr(Box::new(I8)),
            Arr(t) => Ptr(Box::new(t.to_llvm_type()))
        }
    }
}

impl TypeConvertable<LlvmType> for types::SpecType {
    fn to_llvm_type(&self) -> LlvmType {
        use self::types::SpecType::*;
        match self {
            Type(t) => t.to_llvm_type(),
            Func(args, res) => { 
                let llvm_args = args.iter().map(|t| Box::new((*t).to_llvm_type())).collect();
                let llvm_res = Box::new((*res).to_llvm_type());
                LlvmType::Func(llvm_args, llvm_res)
            }
        }
    }
}

impl ast::BinOper {
    fn lazy(&self) -> bool {
        use self::ast::BinOper::*;
        match self {
            And => true,
            Or => true,
            _ => false
        }
    }
}

// impl types::InternalType {
//     fn first(self) -> types::SpecType {
//         let types::InternalType(v) = self;
//         v[0]
//     }
// }

trait Ignore {
    fn ignore(self) -> ();
}

impl<T> Ignore for T {
    fn ignore(self) -> () {
        ()
    }
}