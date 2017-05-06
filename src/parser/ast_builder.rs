
use std::fmt;

use parser::ast;
use parser::ast::{AstKind, AstNode};
use parser::token_kind::TokenKind;
use parser::tokenizer::{Token, TokenError, TokenLocation, Tokenizer, TokenizerMode,
                        TokenizerPosition};
use parser::input_stream::{InputStream, StreamPosition};

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
    None,
    TokenizerError(TokenError),
    UnexpectedToken{expected:TokenKind, got:TokenKind},
    ExpectedVariableName,
    ExpectedCommaOrSemicolon
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
            let token = self.next_token();
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
            match self.try_parse_source_element()? {
                Some(source_element_box) => {
                    program_node.add_source_element(source_element_box);
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

    fn try_parse_source_element(&mut self) -> MaybeParseResult<Box<AstNode>> {
        // Try and parse a statement.
        if let Some(stmt) = self.try_parse_statement()? {
            return Ok(Some(stmt));
        }
        Ok(None)
    }

    fn try_parse_statement(&mut self) -> MaybeParseResult<Box<AstNode>> {
        let begin_position = self.mark_position();
        let tok = self.next_token();
        if tok.kind().is_open_brace() {
            return Ok(Some(Box::new(self.parse_block_statement()?)));
        }
        if tok.kind().is_var_keyword() {
            return Ok(Some(Box::new(self.parse_var_statement()?)));
        }
        if tok.kind().is_semicolon() {
            return Ok(Some(Box::new(ast::EmptyStatementNode::new())));
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
            let name_token = match self.expect_get_token(TokenKind::identifier()) {
                Some(token) => token,
                None => { return Err(ParseError::ExpectedVariableName); }
            };
            var_statement.add_variable(name_token);

            let next_tok = self.next_token();
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

    fn must_expect_token(&mut self, kind: TokenKind) -> ParseResult<()> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token();
        if token.kind() == kind {
            Ok(())
        } else {
            self.rewind_position(position);
            Err(ParseError::UnexpectedToken {expected: kind, got: token.kind()})
        }
    }

    fn expect_token(&mut self, kind: TokenKind) -> bool {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token();
        if token.kind() == kind {
            true
        } else {
            self.rewind_position(position);
            false
        }
    }
    fn expect_get_token(&mut self, kind: TokenKind) -> Option<FullToken> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token();
        if token.kind() == kind {
            Some(token)
        } else {
            self.rewind_position(position);
            None
        }
    }

    fn mark_position(&mut self) -> TokenizerPosition {
        self.tokenizer.mark_position()
    }
    fn rewind_position(&mut self, position: TokenizerPosition) {
        self.tokenizer.rewind_position(position)
    }

    fn next_token(&mut self) -> FullToken {
        self.next_token_impl(/* check_kw = */ true, /* want_newlines = */ false)
    }
    fn next_token_no_keywords(&mut self) -> FullToken {
        self.next_token_impl(/* check_kw = */ true, /* want_newlines = */ false)
    }

    fn next_token_impl(&mut self, check_kw: bool, want_newlines: bool) -> FullToken {
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
            let kw_str = if check_kw { "kw" } else { "no-kw" };
            let nl_str = if want_newlines { "nl" } else { "no-nl" };
            println!("next_token({}, {}): {}", kw_str, nl_str, token.token_string());
            return token;
        }
    }
}
