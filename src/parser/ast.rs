
use std::fmt;

use parser::ast_builder::FullToken;
use parser::tokenizer::Token;

#[derive(Debug, Clone, Copy)]
pub enum AstKind {
    Program,
    BlockStatement,
    VarStatement,
    ExpressionStatement
}
impl AstKind {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait AstNode where Self: fmt::Debug {
    fn kind(&self) -> AstKind;
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
 **** BlockStatementNode *****************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct BlockStatementNode {
}
impl BlockStatementNode {
    pub fn new() -> BlockStatementNode {
        BlockStatementNode {}
    }
}
impl AstNode for BlockStatementNode {
    fn kind(&self) -> AstKind {
        AstKind::BlockStatement
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("Block{}")
    }
}

/*****************************************************************************
 **** VarStatementNode *******************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct VarStatementNode {
    variables: Vec<Box<FullToken>>
}
impl VarStatementNode {
    pub fn new() -> VarStatementNode {
        VarStatementNode {
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
impl AstNode for VarStatementNode {
    fn kind(&self) -> AstKind {
        AstKind::VarStatement
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
 **** ExpressionStatementNode ************************************************
 *****************************************************************************/
#[derive(Debug)]
pub struct ExpressionStatementNode {
    expression: Box<AstNode>
}
impl ExpressionStatementNode {
    fn new(expression: Box<AstNode>) -> ExpressionStatementNode {
        ExpressionStatementNode {
            expression: expression
        }
    }

    fn expression(&self) -> &AstNode {
        self.expression.as_ref()
    }
}
impl AstNode for ExpressionStatementNode {
    fn kind(&self) -> AstKind {
        AstKind::ExpressionStatement
    }
    fn write_tree(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        w.write_str("ExpressionStatement{}")
    }
}
