
use parser::ast_builder::FullToken;

pub enum AstKind {
    Program,
    BlockStatement,
    VarStatement,
    ExpressionStatement
}

pub trait AstNode {
    fn kind(&self) -> AstKind;
}

/*****************************************************************************
 **** ProgramNode ************************************************************
 *****************************************************************************/
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

/*****************************************************************************
 **** BlockStatementNode *****************************************************
 *****************************************************************************/
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

/*****************************************************************************
 **** VarStatementNode *******************************************************
 *****************************************************************************/
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


/*****************************************************************************
 **** ExpressionStatementNode ************************************************
 *****************************************************************************/
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
