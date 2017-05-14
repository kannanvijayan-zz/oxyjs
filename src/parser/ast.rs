
use std::fmt;

use parser::ast_builder::FullToken;
use parser::tokenizer::Token;

#[derive(Debug, Clone, Copy)]
pub enum AstKind {
    Program,
    BlockStmt,
    VarStmt,
    EmptyStmt,
    IfStmt,
    ExprStmt,

    PropertyExpr,
    ConstructExpr,
    PostfixOpExpr,
    UnaryOpExpr,
    BinaryOpExpr,
    CondExpr,
    AssignExpr,
    CommaExpr,
    AtomicExpr
}
impl AstKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait AstNode where Self: fmt::Debug {
    fn kind(&self) -> AstKind;
    fn is_statement(&self) -> bool;
    fn is_expression(&self) -> bool;
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error>;

    fn tree_string(&self) -> String {
        let mut str = String::new();
        self.write_tree(&mut str).unwrap();
        str
    }
}

/*****************************************************************************
 **** ProgramNode ************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ProgramNode {
    source_elements: Vec<Box<AstNode>>
}
impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode { source_elements: Vec::with_capacity(3) }
    }

    pub fn source_elements(&self) -> &Vec<Box<AstNode>> {
        &self.source_elements
    }
    pub fn add_source_element(&mut self, source_element: Box<AstNode>) {
        self.source_elements.push(source_element);
    }
}
impl AstNode for ProgramNode {
    fn kind(&self) -> AstKind {
        AstKind::Program
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ProgramNode{")?;
        let mut first = true;
        for source_element in &self.source_elements {
            if ! first {
                w.write_str(", ")?;
            }
            first = false;
            source_element.write_tree(w)?;
        }
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** BlockStmtNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct BlockStmtNode {
}
impl BlockStmtNode {
    pub fn new() -> BlockStmtNode {
        BlockStmtNode {}
    }
}
impl AstNode for BlockStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::BlockStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Block{}")
    }
}

/*****************************************************************************
 **** VarStmtNode ************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct VarStmtNode {
    var_decls: Vec<VarDecl>
}
impl VarStmtNode {
    pub fn new() -> VarStmtNode {
        VarStmtNode { var_decls: Vec::with_capacity(1) }
    }

    pub fn var_decls(&self) -> &Vec<VarDecl> {
        &self.var_decls
    }

    pub fn add_var_decl(&mut self, name: FullToken) {
        self.var_decls.push(VarDecl::new(name, None));
    }
    pub fn add_var_decl_with_init(&mut self, name: FullToken, init: Box<AstNode>) {
        self.var_decls.push(VarDecl::new(name, Some(init)));
    }
}
impl AstNode for VarStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::VarStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Var{")?;
        let mut first = true;
        for var_decl in &self.var_decls {
            if ! first {
                w.write_str(", ")?;
            }
            first = false;
            var_decl.write_tree(w)?;
        }
        w.write_str("}")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct VarDecl {
    name: FullToken,
    init_expr: Option<Box<AstNode>>
}
impl VarDecl {
    pub fn new(name: FullToken, init_expr: Option<Box<AstNode>>) -> VarDecl {
        assert!(name.kind().is_identifier());
        assert!(init_expr.as_ref().map_or(true, |boxed_expr| boxed_expr.is_expression()));

        VarDecl { name, init_expr }
    }

    pub fn name(&self) -> &FullToken {
        &self.name
    }
    pub fn has_init_expr(&self) -> bool {
        self.init_expr.is_some()
    }
    pub fn init_expr(&self) -> Option<&AstNode> {
        if let Some(ref boxed_expr) = self.init_expr {
            Some(boxed_expr.as_ref())
        } else {
            None
        }
    }

    pub fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        self.name.write_token(w)?;
        if let Some(ref expr) = self.init_expr {
            w.write_str(" = ")?;
            expr.write_tree(w)?;
        }
        Ok(())
    }
}

/*****************************************************************************
 **** EmptyStmtNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct EmptyStmtNode {
}
impl EmptyStmtNode {
    pub fn new() -> EmptyStmtNode {
        EmptyStmtNode {}
    }
}
impl AstNode for EmptyStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::EmptyStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Empty{}")
    }
}

/*****************************************************************************
 **** IfStmtNode *************************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct IfStmtNode {
    cond_expr: Box<AstNode>,
    if_true_stmt: Box<AstNode>,
    if_false_stmt: Option<Box<AstNode>>
}
impl IfStmtNode {
    pub fn new_if(cond_expr: Box<AstNode>, if_true_stmt: Box<AstNode>)
        -> IfStmtNode
    {
        assert!(cond_expr.is_expression());
        assert!(if_true_stmt.is_statement());
        IfStmtNode { cond_expr, if_true_stmt, if_false_stmt: None }
    }
    pub fn new_if_else(cond_expr: Box<AstNode>,
                       if_true_stmt: Box<AstNode>,
                       if_false_stmt: Box<AstNode>)
        -> IfStmtNode
    {
        assert!(cond_expr.is_expression());
        assert!(if_true_stmt.is_statement());
        assert!(if_false_stmt.is_statement());
        IfStmtNode { cond_expr, if_true_stmt, if_false_stmt: Some(if_false_stmt) }
    }

    pub fn cond_expr(&self) -> &AstNode {
        self.cond_expr.as_ref()
    }
    pub fn if_true_stmt(&self) -> &AstNode {
        self.if_true_stmt.as_ref()
    }
    pub fn has_if_false_stmt(&self) -> bool {
        self.if_false_stmt.is_some()
    }
    pub fn if_false_stmt(&self) -> Option<&AstNode> {
        if let Some(ref boxed_stmt) = self.if_false_stmt {
            Some(boxed_stmt.as_ref())
        } else {
            None
        }
    }
}
impl AstNode for IfStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::IfStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }

    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("If(")?;
        self.cond_expr.write_tree(w)?;
        w.write_str("){")?;
        self.if_true_stmt.write_tree(w)?;
        w.write_str("}")?;
        if let Some(ref boxed_stmt) = self.if_false_stmt {
            w.write_str("Else{")?;
            boxed_stmt.write_tree(w)?;
            w.write_str("}")?;
        }
        Ok(())
    }
}

/*****************************************************************************
 **** ExprStmtNode ***********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ExprStmtNode {
    expr: Box<AstNode>
}
impl ExprStmtNode {
    pub fn new(expr: Box<AstNode>) -> ExprStmtNode {
        assert!(expr.is_expression());
        ExprStmtNode { expr }
    }

    pub fn expression(&self) -> &AstNode {
        self.expr.as_ref()
    }
}
impl AstNode for ExprStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::ExprStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ExprStmt{")?;
        self.expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** BinaryExprNode *********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct BinaryOpExprNode {
    binary_op: FullToken,
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl BinaryOpExprNode {
    pub fn new(binary_op: FullToken, left_expr: Box<AstNode>, right_expr: Box<AstNode>)
        -> BinaryOpExprNode
    {
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        BinaryOpExprNode { binary_op, left_expr, right_expr }
    }

    pub fn binary_op(&self) -> &FullToken {
        &self.binary_op
    }
    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for BinaryOpExprNode {
    fn kind(&self) -> AstKind {
        AstKind::BinaryOpExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("BinaryOpExpr(")?;
        self.binary_op.write_token(w)?;
        w.write_str("){")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** CondExprNode ***********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct CondExprNode {
    cond_expr: Box<AstNode>,
    if_expr: Box<AstNode>,
    else_expr: Box<AstNode>
}
impl CondExprNode {
    pub fn new(cond_expr: Box<AstNode>, if_expr: Box<AstNode>, else_expr: Box<AstNode>)
        -> CondExprNode
    {
        assert!(cond_expr.is_expression());
        assert!(if_expr.is_expression());
        assert!(else_expr.is_expression());
        CondExprNode { cond_expr, if_expr, else_expr }
    }

    pub fn cond_expr(&self) -> &AstNode {
        self.cond_expr.as_ref()
    }
    pub fn if_expr(&self) -> &AstNode {
        self.if_expr.as_ref()
    }
    pub fn else_expr(&self) -> &AstNode {
        self.else_expr.as_ref()
    }
}
impl AstNode for CondExprNode {
    fn kind(&self) -> AstKind {
        AstKind::CondExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("CondExpr{")?;
        self.cond_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.if_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.else_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** AssignExprNode *********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct AssignExprNode {
    assign_op: FullToken,
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl AssignExprNode {
    pub fn new(assign_op: FullToken, left_expr: Box<AstNode>, right_expr: Box<AstNode>)
        -> AssignExprNode
    {
        // FIXME: assert that left_expr is a valid lvalue expression.
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        assert!(assign_op.kind().is_assignment_op());
        AssignExprNode { assign_op, left_expr, right_expr }
    }

    pub fn assignment_op(&self) -> &FullToken {
        &self.assign_op
    }
    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for AssignExprNode {
    fn kind(&self) -> AstKind {
        AstKind::AssignExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("AssignExpr(")?;
        self.assign_op.write_token(w)?;
        w.write_str("){")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** CommaExprNode **********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct CommaExprNode {
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl CommaExprNode {
    pub fn new(left_expr: Box<AstNode>, right_expr: Box<AstNode>) -> CommaExprNode {
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        CommaExprNode { left_expr, right_expr }
    }

    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for CommaExprNode {
    fn kind(&self) -> AstKind {
        AstKind::CommaExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("CommaExpr{")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** PostfixOpExprNode ******************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct PostfixOpExprNode {
    postfix_op: FullToken,
    sub_expr: Box<AstNode>
}
impl PostfixOpExprNode {
    pub fn new(postfix_op: FullToken, sub_expr: Box<AstNode>) -> PostfixOpExprNode {
        assert!(postfix_op.kind().is_plus_plus() || postfix_op.kind().is_minus_minus());
        assert!(sub_expr.is_expression());
        // FIXME: assert that sub_expr is a valid LVALUE expr.
        PostfixOpExprNode { postfix_op, sub_expr }
    }

    pub fn postfix_op(&self) -> &FullToken {
        &self.postfix_op
    }
    pub fn sub_expr(&self) -> &AstNode {
        self.sub_expr.as_ref()
    }
}
impl AstNode for PostfixOpExprNode {
    fn kind(&self) -> AstKind {
        AstKind::PostfixOpExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("PostfixOpExpr(")?;
        self.postfix_op.write_token(w)?;
        w.write_str("){")?;
        self.sub_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** UnaryOpExprNode ********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct UnaryOpExprNode {
    unary_op: FullToken,
    sub_expr: Box<AstNode>
}
impl UnaryOpExprNode {
    pub fn new(unary_op: FullToken, sub_expr: Box<AstNode>) -> UnaryOpExprNode {
        assert!(unary_op.kind().is_unary_op());
        assert!(sub_expr.is_expression());
        UnaryOpExprNode { unary_op, sub_expr }
    }

    pub fn unary_op(&self) -> &FullToken {
        &self.unary_op
    }
    pub fn sub_expr(&self) -> &AstNode {
        self.sub_expr.as_ref()
    }
}
impl AstNode for UnaryOpExprNode {
    fn kind(&self) -> AstKind {
        AstKind::UnaryOpExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("UnaryOpExpr(")?;
        self.unary_op.write_token(w)?;
        w.write_str("){")?;
        self.sub_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** ConstructExprNode ******************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ConstructExprNode {
    sub_expr: Box<AstNode>,
    arguments: Vec<Box<AstNode>>,
    has_arguments: bool
}
impl ConstructExprNode {
    pub fn new_bare(sub_expr: Box<AstNode>) -> ConstructExprNode {
        assert!(sub_expr.is_expression());
        // FIXME: assert that sub_expr is a valid MEMBER expr.
        ConstructExprNode { sub_expr, arguments: Vec::new(), has_arguments: false }
    }
    pub fn new_with_arguments(sub_expr: Box<AstNode>, arguments: Vec<Box<AstNode>>)
        -> ConstructExprNode
    {
        assert!(sub_expr.is_expression());
        // FIXME: assert that sub_expr is a valid MEMBER expr.
        ConstructExprNode { sub_expr, arguments, has_arguments: false }
    }

    pub fn sub_expr(&self) -> &AstNode {
        self.sub_expr.as_ref()
    }
    pub fn has_arguments(&self) -> bool {
        self.has_arguments
    }
    pub fn arguments(&self) -> &Vec<Box<AstNode>> {
        &self.arguments
    }
}
impl AstNode for ConstructExprNode {
    fn kind(&self) -> AstKind {
        AstKind::ConstructExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ConstructExpr{")?;
        self.sub_expr.write_tree(w)?;
        if self.has_arguments {
            w.write_str("(")?;
            let mut first = false;
            for arg in &self.arguments {
                if ! first {
                    w.write_str(", ")?;
                }
                first = false;
                arg.write_tree(w)?;
            }
        }
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** PropertyExprNode *******************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct PropertyExprNode {
    target_expr: Box<AstNode>,
    property_name: FullToken
}
impl PropertyExprNode {
    pub fn new(target_expr: Box<AstNode>, property_name: FullToken) -> PropertyExprNode {
        assert!(target_expr.is_expression());
        assert!(property_name.kind().is_identifier());
        PropertyExprNode { target_expr, property_name }
    }

    pub fn target_expr(&self) -> &AstNode {
        self.target_expr.as_ref()
    }
    pub fn property_name(&self) -> &FullToken {
        &self.property_name
    }
}
impl AstNode for PropertyExprNode {
    fn kind(&self) -> AstKind {
        AstKind::PropertyExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("PropertyExpr{")?;
        self.target_expr.write_tree(w)?;
        w.write_str(";")?;
        self.property_name.write_token(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** AtomicExprNode *********************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct AtomicExprNode {
    name: FullToken
}
impl AtomicExprNode {
    pub fn new(name: FullToken) -> AtomicExprNode {
        assert!(name.kind().is_atomic_expr());
        AtomicExprNode { name }
    }

    pub fn name(&self) -> &FullToken {
        &self.name
    }
}
impl AstNode for AtomicExprNode {
    fn kind(&self) -> AstKind {
        AstKind::AtomicExpr
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("AtomicExpr{")?;
        self.name.write_token(w);
        w.write_str("}")?;
        Ok(())
    }
}
