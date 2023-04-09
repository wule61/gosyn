//! Define all golang syntax node

use crate::token::Keyword;
use crate::token::LitKind;
use crate::token::Operator;

use std::fmt::Debug;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct Comment {
    pub pos: usize,
    pub text: String,
}

#[derive(Default, Debug)]
pub struct Ident {
    pub pos: usize,
    pub name: String,
}

// ================ Type Definition ================

#[derive(Debug)]
pub struct PointerType {
    pub pos: usize,
    pub typ: Box<Expression>,
}

#[derive(Debug)]
pub struct ArrayType {
    pub pos: (usize, usize),
    pub len: Box<Expression>,
    pub typ: Box<Expression>,
}

#[derive(Debug)]
pub struct SliceType {
    pub pos: (usize, usize),
    pub typ: Box<Expression>,
}

#[derive(Debug)]
pub struct MapType {
    pub pos: (usize, usize),
    pub key: Box<Expression>,
    pub val: Box<Expression>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Vec<Ident>,
    pub typ: Expression,
    pub tag: Option<StringLit>,
}

#[derive(Default, Debug)]
pub struct FieldList {
    pub pos: Option<(usize, usize)>,
    pub list: Vec<Field>,
}

#[derive(Debug)]
pub struct StructType {
    pub pos: (usize, usize),
    pub fields: Vec<Field>,
}

#[derive(Default, Debug)]
pub struct FuncType {
    pub pos: usize,
    pub typ_params: FieldList,
    pub params: FieldList,
    pub result: FieldList,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ChanMode {
    Send,
    Recv,
}

#[derive(Debug)]
pub struct ChannelType {
    pub pos: (usize, usize), // chan, <-
    pub dir: Option<ChanMode>,
    pub typ: Box<Expression>,
}

#[derive(Debug)]
pub struct InterfaceType {
    pub pos: usize,
    pub methods: FieldList,
}

// ================ Expression Definition ================

#[derive(Debug)]
pub struct BasicLit {
    pub pos: usize,
    pub kind: LitKind,
    pub value: String,
}

#[derive(Default, Debug)]
pub struct StringLit {
    pub pos: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct FuncLit {
    pub typ: FuncType,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub enum Element {
    Expr(Expression),
    LitValue(LiteralValue),
}

#[derive(Debug)]
pub struct KeyedElement {
    pub key: Option<Element>,
    pub val: Element,
}

#[derive(Debug)]
pub struct LiteralValue {
    pub pos: (usize, usize),
    pub values: Vec<KeyedElement>,
}

#[derive(Debug)]
pub struct CompositeLit {
    pub typ: Box<Expression>,
    pub val: LiteralValue,
}

#[derive(Debug)]
pub struct Selector {
    pub pos: usize,
    pub x: Box<Expression>,
    pub sel: Ident,
}

#[derive(Debug)]
pub struct TypeAssertion {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub right: Option<Box<Expression>>, // None for x.(type)
}

#[derive(Debug)]
pub struct Index {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug)]
pub struct IndexList {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub indices: Vec<Expression>,
}

#[derive(Debug)]
pub struct Slice {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub index: [Option<Box<Expression>>; 3],
}

#[derive(Debug)]
pub struct Call {
    pub pos: (usize, usize), // third pos > 0 means the ellipsis argument
    pub args: Vec<Expression>,
    pub func: Box<Expression>,
    pub dots: Option<usize>,
}

#[derive(Debug)]
pub struct ParenExpression {
    pub pos: (usize, usize),
    pub expr: Box<Expression>,
}

#[derive(Debug)]
pub struct StarExpression {
    pub pos: usize,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Ellipsis {
    pub pos: usize,
    pub elt: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct RangeExpr {
    pub pos: usize, // pos of 'range'
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Operation {
    pub pos: usize,
    pub op: Operator,
    pub x: Box<Expression>,
    pub y: Option<Box<Expression>>,
}

pub enum Expression {
    Call(Call),
    Index(Index),
    IndexList(IndexList),
    Slice(Slice),
    Ident(Ident),
    FuncLit(FuncLit),
    Ellipsis(Ellipsis),
    Selector(Selector),
    BasicLit(BasicLit),
    Range(RangeExpr),
    Star(StarExpression),
    Paren(ParenExpression),
    TypeAssert(TypeAssertion),
    CompositeLit(CompositeLit),
    List(Vec<Expression>),
    Operation(Operation),
    TypeMap(MapType),             // map[K]V
    TypeArray(ArrayType),         // [N]T
    TypeSlice(SliceType),         // []T
    TypeFunction(FuncType),       // func (...) ...
    TypeStruct(StructType),       // struct { ... }
    TypeChannel(ChannelType),     // <-chan T | chan<- T | chan T
    TypePointer(PointerType),     // *T
    TypeInterface(InterfaceType), // interface { ... }
}

// ================ Declaration Definition ================

#[derive(Debug)]
pub struct Decl<T> {
    pub docs: Vec<Rc<Comment>>,
    pub pos0: usize,                  // pos of var | const | type
    pub pos1: Option<(usize, usize)>, // pos of '(' and ')'
    pub specs: Vec<T>,
}

#[derive(Default, Debug)]
pub struct VarSpec {
    pub docs: Vec<Rc<Comment>>,
    pub name: Vec<Ident>,
    pub typ: Option<Expression>,
    pub values: Vec<Expression>,
}

#[derive(Default, Debug)]
pub struct ConstSpec {
    pub docs: Vec<Rc<Comment>>,
    pub name: Vec<Ident>,
    pub typ: Option<Expression>,
    pub values: Vec<Expression>,
}

#[derive(Debug)]
pub struct TypeSpec {
    pub docs: Vec<Rc<Comment>>,
    pub alias: bool,
    pub name: Ident,
    pub params: FieldList,
    pub typ: Expression,
}

#[derive(Debug)]
pub struct FuncDecl {
    pub docs: Vec<Rc<Comment>>,
    pub recv: Option<FieldList>,
    pub name: Ident,
    pub typ: FuncType,
    pub body: Option<BlockStmt>,
}

#[derive(Debug)]
pub enum Declaration {
    Function(FuncDecl),
    Type(Decl<TypeSpec>),
    Const(Decl<ConstSpec>),
    Variable(Decl<VarSpec>),
}

// ================ Statement Definition ================

#[derive(Debug)]
pub struct BlockStmt {
    pub pos: (usize, usize),
    pub list: Vec<Statement>,
}

#[derive(Debug)]
pub enum DeclStmt {
    Type(Decl<TypeSpec>),
    Const(Decl<ConstSpec>),
    Variable(Decl<VarSpec>),
}

#[derive(Debug)]
pub struct GoStmt {
    pub pos: usize,
    pub call: Call,
}

#[derive(Debug)]
pub struct DeferStmt {
    pub pos: usize,
    pub call: Call,
}

#[derive(Debug)]
pub struct ReturnStmt {
    pub pos: usize,
    pub ret: Vec<Expression>,
}

#[derive(Debug)]
pub struct BranchStmt {
    pub pos: usize,
    pub key: Keyword,
    pub ident: Option<Ident>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub cond: Expression,
    pub body: BlockStmt,
    pub else_: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct AssignStmt {
    // position of assign operator like = | += | &=
    pub pos: usize,
    pub op: Operator,
    pub left: Vec<Expression>,
    pub right: Vec<Expression>,
}

#[derive(Debug)]
pub struct LabeledStmt {
    pub pos: usize,
    pub name: Ident,
    pub stmt: Box<Statement>,
}

#[derive(Debug)]
pub struct SendStmt {
    pub pos: usize,
    pub chan: Expression,
    pub value: Expression,
}

#[derive(Debug)]
pub struct ExprStmt {
    pub expr: Expression,
}

#[derive(Debug)]
pub struct CaseClause {
    pub tok: Keyword,
    pub pos: (usize, usize),
    pub list: Vec<Expression>,
    pub body: Box<Vec<Statement>>,
}

#[derive(Debug)]
pub struct CaseBlock {
    pub pos: (usize, usize),
    pub body: Vec<CaseClause>,
}

#[derive(Debug)]
pub struct SwitchStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub tag: Option<Expression>,
    pub block: CaseBlock,
}

#[derive(Debug)]
pub struct TypeSwitchStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub tag: Option<Box<Statement>>,
    pub block: CaseBlock,
}

#[derive(Debug)]
pub struct IncDecStmt {
    pub pos: usize,
    pub op: Operator,
    pub expr: Expression,
}

#[derive(Debug)]
pub struct CommClause {
    pub pos: (usize, usize), // pos of (keyword, colon)
    pub tok: Keyword,
    pub comm: Option<Box<Statement>>,
    pub body: Box<Vec<Statement>>,
}

#[derive(Debug)]
pub struct CommBlock {
    pub pos: (usize, usize),
    pub body: Vec<CommClause>,
}

#[derive(Debug)]
pub struct SelectStmt {
    pub pos: usize,
    pub body: CommBlock,
}

#[derive(Debug)]
pub struct RangeStmt {
    pub pos: (usize, usize), // pos of (for, range)
    pub key: Option<Expression>,
    pub value: Option<Expression>,
    pub op: Option<(usize, Operator)>, // define or assign
    pub expr: Expression,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct ForStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub cond: Option<Box<Statement>>,
    pub post: Option<Box<Statement>>,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct EmptyStmt {
    pub pos: usize,
}

#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Go(GoStmt),
    If(IfStmt),
    For(ForStmt),
    Send(SendStmt),
    Expr(ExprStmt),
    Defer(DeferStmt),
    Block(BlockStmt),
    Range(RangeStmt),
    Empty(EmptyStmt),
    Label(LabeledStmt),
    IncDec(IncDecStmt),
    Assign(AssignStmt),
    Return(ReturnStmt),
    Branch(BranchStmt),
    Switch(SwitchStmt),
    Select(SelectStmt),
    TypeSwitch(TypeSwitchStmt),
    Declaration(DeclStmt),
}

#[derive(Default, Debug)]
pub struct Import {
    pub name: Option<Ident>,
    pub path: StringLit,
}

#[derive(Default, Debug)]
pub struct File {
    pub path: Option<PathBuf>,
    pub line_info: Vec<usize>,
    pub docs: Vec<Rc<Comment>>,
    pub pkg_name: Ident,
    pub imports: Vec<Import>,
    pub decl: Vec<Declaration>,
    pub comments: Vec<Rc<Comment>>,
}

#[derive(Debug)]
pub struct Package {
    pub path: PathBuf,
    pub files: Vec<File>,
}

// ================ Type Implemention ================

impl From<Ident> for Field {
    fn from(id: Ident) -> Self {
        Self {
            name: vec![],
            typ: Expression::Ident(id),
            tag: None,
        }
    }
}

impl From<BasicLit> for StringLit {
    fn from(lit: BasicLit) -> StringLit {
        assert_eq!(lit.kind, LitKind::String);
        StringLit { pos: lit.pos, value: lit.value }
    }
}

impl From<Expression> for Field {
    fn from(expr: Expression) -> Self {
        Self {
            name: vec![],
            typ: expr,
            tag: None,
        }
    }
}

impl AssignStmt {
    pub fn is_range(&self) -> bool {
        matches!(self.right.first(), Some(Expression::Range(_)))
    }
}

impl FieldList {
    pub fn pos(&self) -> usize {
        if let Some((pos, _)) = self.pos {
            return pos;
        }

        if let Some(field) = self.list.first() {
            if let Some(name) = field.name.first() {
                return name.pos;
            }

            return field.typ.pos();
        }

        panic!("call pos on empty FieldList");
    }
}

impl Expression {
    pub fn pos(&self) -> usize {
        match self {
            Expression::Call(x) => x.pos.0,
            Expression::Index(x) => x.pos.0,
            Expression::Slice(x) => x.pos.0,
            Expression::Ident(x) => x.pos,
            Expression::FuncLit(x) => x.typ.pos,
            Expression::Ellipsis(x) => x.pos,
            Expression::Selector(x) => x.x.pos(),
            Expression::BasicLit(x) => x.pos,
            Expression::Range(x) => x.pos,
            Expression::Star(x) => x.pos,
            Expression::Paren(x) => x.pos.0,
            Expression::TypeAssert(x) => x.left.pos(),
            Expression::CompositeLit(x) => x.typ.pos(),
            Expression::IndexList(x) => x.left.pos(),
            Expression::List(_) => unimplemented!("list may empty"),
            Expression::Operation(x) => x.x.pos(),
            Expression::TypeMap(x) => x.pos.0,
            Expression::TypeArray(x) => x.pos.0,
            Expression::TypeSlice(x) => x.pos.0,
            Expression::TypeFunction(f) => f.pos,
            Expression::TypeStruct(x) => x.pos.0,
            Expression::TypeChannel(x) => x.pos.0,
            Expression::TypePointer(x) => x.pos,
            Expression::TypeInterface(x) => x.pos,
        }
    }
}

pub(crate) trait Spec {
    fn with_docs(self, docs: Vec<Rc<Comment>>) -> Self;
}

impl Spec for VarSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> VarSpec {
        self.docs = docs;
        self
    }
}

impl Spec for TypeSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> TypeSpec {
        self.docs = docs;
        self
    }
}

impl Spec for ConstSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> ConstSpec {
        self.docs = docs;
        self
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Call(arg0) => f.debug_tuple("Call").field(arg0).finish(),
            Self::Index(arg0) => f.debug_tuple("Index").field(arg0).finish(),
            Self::IndexList(arg0) => f.debug_tuple("IndexList").field(arg0).finish(),
            Self::Slice(arg0) => f.debug_tuple("Slice").field(arg0).finish(),
            Self::Ident(arg0) => f.debug_tuple("Ident").field(arg0).finish(),
            Self::FuncLit(arg0) => f.debug_tuple("FuncLit").field(arg0).finish(),
            Self::Ellipsis(arg0) => f.debug_tuple("Ellipsis").field(arg0).finish(),
            Self::Selector(arg0) => f.debug_tuple("Selector").field(arg0).finish(),
            Self::BasicLit(arg0) => f.debug_tuple("BasicLit").field(arg0).finish(),
            Self::Range(arg0) => f.debug_tuple("Range").field(arg0).finish(),
            Self::Star(arg0) => f.debug_tuple("Star").field(arg0).finish(),
            Self::Paren(arg0) => f.debug_tuple("Paren").field(arg0).finish(),
            Self::TypeAssert(arg0) => f.debug_tuple("TypeAssert").field(arg0).finish(),
            Self::CompositeLit(arg0) => f.debug_tuple("CompositeLit").field(arg0).finish(),
            Self::List(arg0) => f.debug_tuple("List").field(arg0).finish(),
            Self::Operation(arg0) => f.debug_tuple("Operation").field(arg0).finish(),
            Self::TypeMap(arg0) => f.debug_tuple("TypeMap").field(arg0).finish(),
            Self::TypeArray(arg0) => f.debug_tuple("TypeArray").field(arg0).finish(),
            Self::TypeSlice(arg0) => f.debug_tuple("TypeSlice").field(arg0).finish(),
            Self::TypeFunction(arg0) => f.debug_tuple("TypeFunction").field(arg0).finish(),
            Self::TypeStruct(arg0) => f.debug_tuple("TypeStruct").field(arg0).finish(),
            Self::TypeChannel(arg0) => f.debug_tuple("TypeChannel").field(arg0).finish(),
            Self::TypePointer(arg0) => f.debug_tuple("TypePointer").field(arg0).finish(),
            Self::TypeInterface(arg0) => f.debug_tuple("TypeInterface").field(arg0).finish(),
        }
    }
}

impl Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Go(arg0) => f.debug_tuple("Go").field(arg0).finish(),
            Self::If(arg0) => f.debug_tuple("If").field(arg0).finish(),
            Self::For(arg0) => f.debug_tuple("For").field(arg0).finish(),
            Self::Send(arg0) => f.debug_tuple("Send").field(arg0).finish(),
            Self::Expr(arg0) => f.debug_tuple("Expr").field(arg0).finish(),
            Self::Defer(arg0) => f.debug_tuple("Defer").field(arg0).finish(),
            Self::Block(arg0) => f.debug_tuple("Block").field(arg0).finish(),
            Self::Range(arg0) => f.debug_tuple("Range").field(arg0).finish(),
            Self::Empty(arg0) => f.debug_tuple("Empty").field(arg0).finish(),
            Self::Label(arg0) => f.debug_tuple("Label").field(arg0).finish(),
            Self::IncDec(arg0) => f.debug_tuple("IncDec").field(arg0).finish(),
            Self::Assign(arg0) => f.debug_tuple("Assign").field(arg0).finish(),
            Self::Return(arg0) => f.debug_tuple("Return").field(arg0).finish(),
            Self::Branch(arg0) => f.debug_tuple("Branch").field(arg0).finish(),
            Self::Switch(arg0) => f.debug_tuple("Switch").field(arg0).finish(),
            Self::Select(arg0) => f.debug_tuple("Select").field(arg0).finish(),
            Self::TypeSwitch(arg0) => f.debug_tuple("TypeSwitch").field(arg0).finish(),
            Self::Declaration(arg0) => f.debug_tuple("Declaration").field(arg0).finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_source;

    #[test]
    fn show_debug() {
        let source = include_str!("../tests/testdata/helloworld.go");
        let s = parse_source(source).unwrap();
        println!("{:#?}", s);
    }
}
