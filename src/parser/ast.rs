
pub enum AstKind {
    Program,
    BlockStatement,
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
 **** BlockNode **************************************************************
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
