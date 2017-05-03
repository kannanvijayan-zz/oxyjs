
use parser::tokenizer::{Token, TokenError, TokenKind, TokenLocation, Tokenizer, TokenizerMode};
use parser::input_stream::InputStream;
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

pub struct AstBuilder<STREAM: InputStream> {
    tokenizer: Tokenizer<STREAM, FullTokenizerMode>
}
impl<STREAM: InputStream> AstBuilder<STREAM> {
    pub fn new(stream: STREAM) -> AstBuilder<STREAM> {
        AstBuilder {
            tokenizer: Tokenizer::new(stream, FullTokenizerMode{})
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<()> {
        // Just read tokens and print them out until we're done, then return Error.
        loop {
            let tok = self.next_token();
            println!("Token: {:?}", tok.kind());
            if tok.is_error() {
                return Result::Err(ParseError::TokenizerError(self.tokenizer.get_error()));
            }
            if tok.is_end() {
                break;
            }
        }
        Ok(())
    }

    fn next_token(&mut self) -> FullToken {
        self.tokenizer.next_token(true)
    }
}
