
use std::fmt;
use parser::ast_builder::FullToken;

#[derive(Debug, Clone, Copy)]
pub enum AstKind {
    Program,
    BlockStatement,
    VarStatement,
    ExpressionStatement
}
impl fmt::Display for AstKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub trait AstNode where Self: fmt::Debug + fmt::Display {
    fn kind(&self) -> AstKind;
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
}
impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ProgramNode{")?;
        let mut first = true;
        for source_element in &self.source_elements {
            if ! first {
                f.write_str(", ")?;
            }
            first = false;
            write!(f, "{}", source_element)?;
        }
        f.write_str("}")?;
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
}
impl fmt::Display for BlockStatementNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("Block{}")
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
}
impl fmt::Display for VarStatementNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("Var{")?;
        let mut first = true;
        for variable in &self.variables {
            if ! first {
                f.write_str(", ")?;
            }
            first = false;
            write!(f, "{}", variable)?;
        }
        f.write_str("}")?;
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
}
impl fmt::Display for ExpressionStatementNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str("ExpressionStatement{}")
    }
}
