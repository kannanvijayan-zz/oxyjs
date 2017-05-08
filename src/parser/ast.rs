
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
    ExpressionStmt,

    ConditionalExpression,
    AssignmentExpression,
    CommaExpression,
    NameExpression
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
        ProgramNode {
            source_elements: Vec::with_capacity(3)
        }
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
    variables: Vec<Box<FullToken>>
}
impl VarStmtNode {
    pub fn new() -> VarStmtNode {
        VarStmtNode {
            variables: Vec::with_capacity(1)
        }
    }

    pub fn variables(&self) -> &Vec<Box<FullToken>> {
        &self.variables
    }
    pub fn add_variable(&mut self, var: FullToken) {
        self.variables.push(Box::new(var));
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
        for variable in &self.variables {
            if ! first {
                w.write_str(", ")?;
            }
            first = false;
            variable.write_token(w)?;
        }
        w.write_str("}")?;
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
    condition_expression: Box<AstNode>,
    if_statement: Box<AstNode>
}
impl IfStmtNode {
    pub fn new_if(condition_expression: Box<AstNode>, if_statement: Box<AstNode>)
        -> IfStmtNode
    {
        IfStmtNode {
            condition_expression: condition_expression,
            if_statement: if_statement
        }
    }

    pub fn condition_expression(&self) -> &AstNode {
        self.condition_expression.as_ref()
    }
    pub fn if_statement(&self) -> &AstNode {
        self.if_statement.as_ref()
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
        self.condition_expression.write_tree(w)?;
        w.write_str("){")?;
        self.if_statement.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** ExpressionStmtNode *****************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ExpressionStmtNode {
    expression: Box<AstNode>
}
impl ExpressionStmtNode {
    pub fn new(expression: Box<AstNode>) -> ExpressionStmtNode {
        ExpressionStmtNode {
            expression: expression
        }
    }

    pub fn expression(&self) -> &AstNode {
        self.expression.as_ref()
    }
}
impl AstNode for ExpressionStmtNode {
    fn kind(&self) -> AstKind {
        AstKind::ExpressionStmt
    }
    fn is_statement(&self) -> bool {
        true
    }
    fn is_expression(&self) -> bool {
        false
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ExpressionStmt{")?;
        self.expression.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** ConditionalExpressionNode **********************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ConditionalExpressionNode {
    cond_expr: Box<AstNode>,
    if_expr: Box<AstNode>,
    else_expr: Box<AstNode>
}
impl ConditionalExpressionNode {
    pub fn new(cond_expr: Box<AstNode>, if_expr: Box<AstNode>, else_expr: Box<AstNode>)
        -> ConditionalExpressionNode
    {
        assert!(cond_expr.is_expression());
        assert!(if_expr.is_expression());
        assert!(else_expr.is_expression());
        ConditionalExpressionNode {
            cond_expr: cond_expr,
            if_expr: if_expr,
            else_expr: else_expr
        }
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
impl AstNode for ConditionalExpressionNode {
    fn kind(&self) -> AstKind {
        AstKind::ConditionalExpression
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ConditionalExpr{")?;
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
 **** AssignmentExpressionNode ***********************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct AssignmentExpressionNode {
    assignment_op: FullToken,
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl AssignmentExpressionNode {
    pub fn new(assignment_op: FullToken, left_expr: Box<AstNode>, right_expr: Box<AstNode>)
        -> AssignmentExpressionNode
    {
        // FIXME: assert that left_expr is a valid lvalue expression.
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        assert!(assignment_op.kind().is_assignment_op());
        AssignmentExpressionNode {
            assignment_op: assignment_op,
            left_expr: left_expr,
            right_expr: right_expr
        }
    }

    pub fn assignment_op(&self) -> &FullToken {
        &self.assignment_op
    }
    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for AssignmentExpressionNode {
    fn kind(&self) -> AstKind {
        AstKind::AssignmentExpression
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("AssignmentExpr(")?;
        self.assignment_op.write_token(w)?;
        w.write_str("){")?;
        self.left_expr.write_tree(w)?;
        w.write_str(", ")?;
        self.right_expr.write_tree(w)?;
        w.write_str("}")?;
        Ok(())
    }
}

/*****************************************************************************
 **** CommaExpressionNode ****************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct CommaExpressionNode {
    left_expr: Box<AstNode>,
    right_expr: Box<AstNode>
}
impl CommaExpressionNode {
    pub fn new(left_expr: Box<AstNode>, right_expr: Box<AstNode>) -> CommaExpressionNode {
        assert!(left_expr.is_expression());
        assert!(right_expr.is_expression());
        CommaExpressionNode {
            left_expr: left_expr,
            right_expr: right_expr
        }
    }

    pub fn left_expr(&self) -> &AstNode {
        self.left_expr.as_ref()
    }
    pub fn right_expr(&self) -> &AstNode {
        self.right_expr.as_ref()
    }
}
impl AstNode for CommaExpressionNode {
    fn kind(&self) -> AstKind {
        AstKind::CommaExpression
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
 **** NameExpressionNode *****************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct NameExpressionNode {
    name: FullToken
}
impl NameExpressionNode {
    pub fn new(name: FullToken) -> NameExpressionNode {
        assert!(name.kind().is_identifier());
        NameExpressionNode {
            name: name
        }
    }

    pub fn name(&self) -> &FullToken {
        &self.name
    }
}
impl AstNode for NameExpressionNode {
    fn kind(&self) -> AstKind {
        AstKind::NameExpression
    }
    fn is_statement(&self) -> bool {
        false
    }
    fn is_expression(&self) -> bool {
        true
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("NameExpr{")?;
        self.name.write_token(w);
        w.write_str("}")?;
        Ok(())
    }
}
