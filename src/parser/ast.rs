
pub enum AstKind {
    Program,
    ExpressionStatement
}

pub trait AstNode {
    fn kind(&self) -> AstKind;
}
pub trait SourceElement : AstNode {}
pub trait Statement : SourceElement {}
pub trait Expression : AstNode {}

/*****************************************************************************
 **** ProgramNode ************************************************************
 *****************************************************************************/
pub struct ProgramNode {
    source_elements: Vec<Box<SourceElement>>
}
impl ProgramNode {
    pub fn new() -> ProgramNode {
        ProgramNode {
            source_elements: Vec::with_capacity(3)
        }
    }

    pub fn source_elements(&self) -> &Vec<Box<SourceElement>> {
        &self.source_elements
    }
    pub fn add_source_element(&mut self, source_element: Box<SourceElement>) {
        self.source_elements.push(source_element);
    }
}
impl AstNode for ProgramNode {
    fn kind(&self) -> AstKind {
        AstKind::Program
    }
}


/*****************************************************************************
 **** ExpressionStatementNode ************************************************
 *****************************************************************************/
pub struct ExpressionStatementNode {
    expression: Box<Expression>
}
impl ExpressionStatementNode {
    fn new(expression: Box<Expression>) -> ExpressionStatementNode {
        ExpressionStatementNode {
            expression: expression
        }
    }

    fn expression(&self) -> &Expression {
        self.expression.as_ref()
    }
}
impl AstNode for ExpressionStatementNode {
    fn kind(&self) -> AstKind {
        AstKind::ExpressionStatement
    }
}
impl SourceElement for ExpressionStatementNode {}
impl Statement for ExpressionStatementNode {}
