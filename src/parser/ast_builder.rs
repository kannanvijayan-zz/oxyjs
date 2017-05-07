
use std::fmt;

use parser::ast;
use parser::ast::{AstKind, AstNode};
use parser::input_stream::{InputStream, StreamPosition};
use parser::precedence::Precedence;
use parser::token_kind::TokenKind;
use parser::tokenizer::{Token, TokenError, TokenLocation, Tokenizer, TokenizerMode,
                        TokenizerPosition};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullToken {
    kind: TokenKind,
    location: TokenLocation
}
impl FullToken {
    pub fn new(kind: TokenKind, location: TokenLocation) -> FullToken {
        FullToken {
            kind: kind,
            location: location
        }
    }

    pub fn location(&self) -> &TokenLocation {
        &self.location
    }
}
impl Token for FullToken {
    fn make(kind: TokenKind, location: TokenLocation) -> FullToken {
        FullToken::new(kind, location)
    }
    fn kind(&self) -> TokenKind {
        self.kind
    }
    fn start_offset(&self) -> StreamPosition {
        self.location.start_offset()
    }
    fn write_token(&self, w: &mut fmt::Write) -> Result<(), fmt::Error> {
        write!(w, "Token({}@{})", self.kind.name(), self.location.range_string())
    }
}

pub struct FullTokenizerMode {
}
impl TokenizerMode for FullTokenizerMode {
    type Tok = FullToken;

    fn cares_about_newline(&self) -> bool {
        false
    }
    fn note_newline(&mut self) {}
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Unspecified,
    ErrorToken(FullToken),
    TokenizerError(TokenError),
    UnexpectedToken{expected:TokenKind, got:TokenKind},
    ExpectedVariableName,
    ExpectedCommaOrSemicolon,
    ExpectedExpression
}
pub type ParseResult<T> = Result<T, ParseError>;
pub type MaybeParseResult<T> = ParseResult<Option<T>>;

pub struct AstBuilder<STREAM: InputStream> {
    tokenizer: Tokenizer<STREAM, FullTokenizerMode>
}
impl<STREAM: InputStream> AstBuilder<STREAM> {
    pub fn new(stream: STREAM) -> AstBuilder<STREAM> {
        AstBuilder {
            tokenizer: Tokenizer::new(stream, FullTokenizerMode{})
        }
    }

    pub fn read_and_print_tokens(&mut self) {
        // Just read tokens and print them out until we're done, then return Error.
        loop {
            let token = self.next_token().unwrap();
            println!("Token: {}", token.kind().name());
            if token.kind().is_error() {
                panic!("Got token error: {:?}", self.tokenizer.get_error());
            }
            if token.kind().is_end() {
                break;
            }
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Box<ast::ProgramNode>> {
        let mut program_node = ast::ProgramNode::new();
        loop {
            match self.try_parse_statement()? {
                Some(boxed_source_element) => {
                    program_node.add_source_element(boxed_source_element);
                }
                None => {
                    break;
                }
            }
        }

        // Must have reached end of stream.
        self.must_expect_token(TokenKind::end())?;

        Ok(Box::new(program_node))
    }

    fn try_parse_statement(&mut self) -> MaybeParseResult<Box<AstNode>> {
        let begin_position = self.mark_position();
        let tok = self.next_token()?;
        if tok.kind().is_open_brace() {
            // FIXME: parse either a block or an object literal.
            // The spec says a '{' at the statement level can only be a block,
            // but practical implementations seem to allow bare object literals.
            return Ok(Some(Box::new(self.parse_block_statement()?)));
        }
        if tok.kind().is_var_keyword() {
            return Ok(Some(Box::new(self.parse_var_statement()?)));
        }
        if tok.kind().is_semicolon() {
            return Ok(Some(Box::new(ast::EmptyStatementNode::new())));
        }
        if tok.kind().is_if_keyword() {
            return Ok(Some(self.parse_if_statement()?));
        }

        if let Some(boxed_expr) = self.try_parse_expression(tok, Precedence::lowest())? {
            return Ok(Some(Box::new(ast::ExpressionStatementNode::new(boxed_expr))));
        }

        self.rewind_position(begin_position);
        Ok(None)
    }

    fn parse_block_statement(&mut self) -> ParseResult<ast::BlockStatementNode> {
        // FIXME: Parse list of statements.
        self.must_expect_token(TokenKind::close_brace())?;
        Ok(ast::BlockStatementNode::new())
    }

    fn parse_var_statement(&mut self) -> ParseResult<ast::VarStatementNode> {
        let mut var_statement = ast::VarStatementNode::new();
        loop {
            // FIXME: Support initializer expressions.
            // For now, we match only a VarName ("," VarName)* ";"
            let name_token = match self.expect_get_token(TokenKind::identifier())? {
                Some(token) => token,
                None => { return Err(ParseError::ExpectedVariableName); }
            };
            var_statement.add_variable(name_token);

            let next_tok = self.next_token()?;
            if next_tok.kind().is_semicolon() {
                break;
            }
            if next_tok.kind().is_comma() {
                continue;
            }
            return Err(ParseError::ExpectedCommaOrSemicolon);
        }
        Ok(var_statement)
    }

    fn parse_if_statement(&mut self) -> ParseResult<Box<AstNode>> {
        // FIXME: implement this after expression parsing is supported.
        panic!("parse_if_statement is not implemented.")
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Box<AstNode>> {
        println!("BEGIN (parse_expression)");
        let token = self.next_token()?;
        if let Some(boxed_expr) = self.try_parse_expression(token, precedence)? {
            println!("END (parse_expression)");
            return Ok(boxed_expr);
        }

        Err(ParseError::ExpectedExpression)
    }

    fn try_parse_expression(&mut self, tok: FullToken, precedence: Precedence)
        -> MaybeParseResult<Box<AstNode>>
    {
        if tok.kind().is_identifier() {
            let name_expr = Box::new(ast::NameExpressionNode::new(tok));
            return Ok(Some(self.parse_rest_of_expression(name_expr, precedence)?));
        }
        Ok(None)
    }

    fn parse_rest_of_expression(&mut self, left_expr: Box<AstNode>, precedence: Precedence)
        -> ParseResult<Box<AstNode>>
    {
        assert!(left_expr.is_expression());

        let mut cur_expr = left_expr;
        loop {
            let position = self.mark_position();
            let tok = self.next_token()?;
            if tok.kind.is_comma() {
                if precedence >= Precedence::comma() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                // Accumulate any commas.
                let right_expr = self.parse_expression(Precedence::comma())?;
                cur_expr = Box::new(ast::CommaExpressionNode::new(cur_expr, right_expr));
                continue;
            }

            // Unknown token terminates expression.
            self.rewind_position(position);
            break;
        }

        Ok(cur_expr)
    }

    fn must_expect_token(&mut self, kind: TokenKind) -> ParseResult<()> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token()?;
        if token.kind() == kind {
            Ok(())
        } else {
            self.rewind_position(position);
            Err(ParseError::UnexpectedToken {expected: kind, got: token.kind()})
        }
    }

    fn expect_token(&mut self, kind: TokenKind) -> ParseResult<bool> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token()?;
        if token.kind() == kind {
            Ok(true)
        } else {
            self.rewind_position(position);
            Ok(false)
        }
    }
    fn expect_get_token(&mut self, kind: TokenKind) -> ParseResult<Option<FullToken>> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token()?;
        if token.kind() == kind {
            Ok(Some(token))
        } else {
            self.rewind_position(position);
            Ok(None)
        }
    }

    fn mark_position(&mut self) -> TokenizerPosition {
        self.tokenizer.mark_position()
    }
    fn rewind_position(&mut self, position: TokenizerPosition) {
        self.tokenizer.rewind_position(position)
    }

    fn next_token(&mut self) -> ParseResult<FullToken> {
        self.next_token_impl(/* check_kw = */ true, /* want_newlines = */ false)
    }
    fn next_token_no_keywords(&mut self) -> ParseResult<FullToken> {
        self.next_token_impl(/* check_kw = */ true, /* want_newlines = */ false)
    }

    fn next_token_impl(&mut self, check_kw: bool, want_newlines: bool) -> ParseResult<FullToken> {
        loop {
            let token = self.tokenizer.next_token(/* check_kw = */ true);
            let kind = token.kind();
            // Ignore whitespace and comment and newline tokens.
            if kind.is_whitespace() || kind.is_comment() {
                continue;
            }
            if !want_newlines && kind.is_newline() {
                continue;
            }
            if kind.is_error() {
                return Err(ParseError::ErrorToken(token));
            }
            let kw_str = if check_kw { "kw" } else { "no-kw" };
            let nl_str = if want_newlines { "nl" } else { "no-nl" };
            println!("next_token({}, {}): {}", kw_str, nl_str, token.token_string());
            return Ok(token);
        }
    }
}
