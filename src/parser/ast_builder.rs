
use parser::token_kind::TokenKind;
use parser::tokenizer::{Token, TokenError, TokenLocation, Tokenizer, TokenizerMode};
use parser::input_stream::{InputStream, StreamPosition};
use parser::ast;
use parser::ast::{AstKind, AstNode};

#[derive(Debug, Clone)]
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
    TokenizerError(TokenError)
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
            let token = self.next_token(/* check_kw = */ true);
            println!("Token: {:?}", token.kind());
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
            match self.parse_source_element() ? {
                Some(source_element_box) => {
                    program_node.add_source_element(source_element_box);
                }
                None => { break; }
            }
        }
        Ok(Box::new(program_node))
    }

    fn parse_source_element(&mut self) -> MaybeParseResult<Box<ast::SourceElement>> {
        Ok(None)
    }

    fn next_token(&mut self, check_kw: bool) -> FullToken {
        loop {
            let token = self.tokenizer.next_token(check_kw);
            let kind = token.kind();
            // Ignore whitespace and comment and newline tokens.
            if kind.is_whitespace() || kind.is_comment() || kind.is_newline() {
                continue;
            }
            return token;
        }
    }

    fn next_token_want_newlines(&mut self, check_kw: bool) -> FullToken {
        loop {
            let token = self.tokenizer.next_token(check_kw);
            let kind = token.kind();
            // Ignore whitespace and comment and newline tokens.
            if kind.is_whitespace() || kind.is_comment() {
                continue;
            }
            return token;
        }
    }
}
