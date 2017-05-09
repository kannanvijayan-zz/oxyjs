
use std::fmt;
use std::ptr;
use parser::char_utils::AsciiChar;
use parser::input_stream::{InputStream, StreamPosition};
use parser::token_kind::TokenKind;

/**
 * Initializer for tokenizer module that must be called exactly once at the
 * beginning of program execution.
 */
static mut MODULE_INITIALIZED: bool = false;
pub fn initialize_module() {
    unsafe {
        assert!(!MODULE_INITIALIZED);
        init_single_char_tokens();
        MODULE_INITIALIZED = true;
    }
}

#[derive(Debug, Clone)]
pub enum TokenError {
    PrematureEnd(TokenKind),
    CantHandleUnicodeYet,
    BadNumber,
    UnrecognizedChar(char)
}

/** Raw information required to extract the token from the source text. */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenLocation {
    start_offset: StreamPosition,
    end_offset: StreamPosition
}
impl TokenLocation {
    pub fn new(start_offset: StreamPosition, end_offset: StreamPosition) -> TokenLocation {
        TokenLocation { start_offset, end_offset }
    }
    pub fn default() -> TokenLocation {
        Self::new(StreamPosition::default(), StreamPosition::default())
    }

    pub fn start_offset(&self) -> StreamPosition {
        self.start_offset
    }
    pub fn end_offset(&self) -> StreamPosition {
        self.end_offset
    }

    pub fn range_string(&self) -> String {
        format!("{}-{}", self.start_offset.value(), self.end_offset.value())
    }
}

/**
 * Trait representing the minimal information we care about a token.
 * The implementors of this will be a FullToken or a SyntaxToken, depending
 * on the kind of parse we're doiing.
 */
pub trait Token where Self: Clone + Eq {
    fn make(kind: TokenKind, location: TokenLocation) -> Self;
    fn kind(&self) -> TokenKind;
    fn start_offset(&self) -> StreamPosition;
    fn write_token(&self, w: &mut fmt::Write) -> Result<(), fmt::Error>;
    fn token_string(&self) -> String {
        let mut str = String::new();
        self.write_token(&mut str).unwrap();
        str
    }
}

/**
 * The tokenizer mode is parameterizes a tokenization run.
 * It can either be a FullTokenizerMode or a SyntaxTokenizerMode.
 * It embeds the token type to be used, as well as a number of
 * convenience methods.
 */
pub trait TokenizerMode {
    type Tok: Token;

    fn make_token(&mut self, kind: TokenKind, location: TokenLocation) -> Self::Tok {
        Self::Tok::make(kind, location)
    }

    fn cares_about_newline(&self) -> bool;
    fn note_newline(&mut self);
}

/**
 * TokenizerPosition is similar to StreamPosition.  It records a position in
 * the token stream we can rewind to.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenizerPosition(StreamPosition);

/**
 * The actual tokenizer is parameterized on the input stream type and the
 * token type we're using.
 *
 * The token type will depend on the kind of parse we're doing: full parse or
 * syntax parse.
 */
pub struct Tokenizer<STREAM: InputStream, MODE: TokenizerMode> {
    input_stream: STREAM,
    tokenizer_mode: MODE,
    token_start_position: StreamPosition,
    token_error: Option<TokenError>,
    pushed_back_token: Option<MODE::Tok>
}
impl<STREAM, MODE> Tokenizer<STREAM, MODE>
    where STREAM: InputStream,
          MODE: TokenizerMode
{
    pub fn new(input_stream: STREAM, tokenizer_mode: MODE) -> Tokenizer<STREAM, MODE> {
        Tokenizer {
            input_stream: input_stream,
            tokenizer_mode: tokenizer_mode,
            token_start_position: StreamPosition::default(),
            token_error: None,
            pushed_back_token: None
        }
    }

    pub fn get_error(&self) -> TokenError {
        assert!(self.token_error.is_some());
        self.token_error.as_ref().unwrap().clone()
    }

    pub fn next_token(&mut self, check_kw: bool) -> MODE::Tok {
        assert!(self.token_error.is_none());
        if let Some(tok) = self.pushed_back_token.take() {
            return tok;
        }
        self.read_token(check_kw)
    }

    pub fn push_back_token(&mut self, token: MODE::Tok) {
        assert!(self.pushed_back_token.is_none());
        self.pushed_back_token = Some(token);
    }

    pub fn mark_position(&self) -> TokenizerPosition {
        assert!(self.token_error.is_none());
        if let Some(ref tok) = self.pushed_back_token {
            TokenizerPosition(tok.start_offset())
        } else {
            TokenizerPosition(self.input_stream.mark())
        }
    }
    pub fn rewind_position(&mut self, position: TokenizerPosition) {
        assert!(position <= self.mark_position());
        self.pushed_back_token = None;
        self.input_stream.rewind(position.0);
    }

    fn read_token(&mut self, check_kw: bool) -> MODE::Tok {
        // TODO: Order this according to token occurrence probability.
        self.token_start_position = self.input_stream.mark();

        let ch0 = self.read_ascii_char();

        if ch0.is_whitespace() {
            return self.read_whitespace();
        }

        if ch0.is_identifier_start() {
            if check_kw {
                return self.read_identifier_or_keyword(ch0);
            } else {
                return self.read_identifier();
            }
        }

        // Check single-char tokens.
        // Covers: Open/Close Paren/Bracket/Brace, Dot, Semicolon, Comma, Question, Colon, Tilde
        let single_kind = check_single_char_token(ch0);
        if ! single_kind.is_error() {
            return self.emit_token(single_kind);
        }

        if ch0.is_digit() {
            if ch0.is_char('0') {
                return self.read_ascii_number_starting_with_zero();
            } else {
                return self.read_ascii_number();
            }
        }

        if ch0.is_char('/') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('/') {
                return self.read_line_comment();
            }
            if ch1.is_char('*') {
                return self.read_block_comment();
            }
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::slash_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::slash());
        }

        if ch0.is_char('=') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::strict_equal());
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::equal());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::assign());
        }

        if ch0.is_char('!') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::strict_not_equal());
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::not_equal());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::bang());
        }

        if ch0.is_char('<') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::less_equal());
            }
            if ch1.is_char('<') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::shift_left_assign());
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::shift_left());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::less());
        }

        if ch0.is_char('>') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::greater_equal());
            }
            if ch1.is_char('>') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::shift_right_assign());
                }
                if ch2.is_char('>') {
                    let ch3 = self.read_ascii_char();
                    if ch3.is_char('=') {
                        return self.emit_token(TokenKind::arithmetic_shift_right_assign());
                    }
                    self.unread_ascii_char(ch3);
                    return self.emit_token(TokenKind::arithmetic_shift_right());
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::shift_right());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::greater());
        }

        if ch0.is_char('*') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::star_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::star());
        }

        if ch0.is_char('+') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('+') {
                return self.emit_token(TokenKind::plus_plus());
            }
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::plus_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::plus());
        }

        if ch0.is_char('-') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('-') {
                return self.emit_token(TokenKind::minus_minus());
            }
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::minus_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::minus());
        }

        if ch0.is_char('%') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::percent_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::percent());
        }

        if ch0.is_char('&') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::bit_and_assign());
            }
            if ch1.is_char('&') {
                return self.emit_token(TokenKind::logical_and());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::bit_and());
        }

        if ch0.is_char('|') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::bit_or_assign());
            }
            if ch1.is_char('|') {
                return self.emit_token(TokenKind::logical_or());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::bit_or());
        }

        if ch0.is_char('^') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::bit_xor_assign());
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::bit_xor());
        }

        if self.check_and_finish_ascii_newline(ch0) {
            return self.emit_token(TokenKind::newline());
        }

        if ! ch0.is_ascii_or_end() {
            // TODO: Handle unicode.
            return self.emit_error(TokenError::CantHandleUnicodeYet);
        }

        if ch0.is_end() {
            return self.emit_token(TokenKind::end());
        }

        self.emit_error(TokenError::UnrecognizedChar(ch0.octet_value() as char))
    }

    fn read_whitespace(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_whitespace() {
                self.unread_ascii_char(ch);
                break;
            }
        }
        self.emit_token(TokenKind::whitespace())
    }

    fn read_identifier(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_identifier_continue() {
                // TODO: Check for unicode identifier char.
                self.unread_ascii_char(ch);
                break;
            }
        }
        self.emit_token(TokenKind::identifier())
    }

    fn read_identifier_or_keyword(&mut self, ch0: AsciiChar) -> MODE::Tok {
        // Keep a track of the last 4 bytes of the identifier.
        let mut tail_word = ch0.ascii_value() as u32;
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_identifier_continue() {
                // TODO: Check for unicode identifier char.
                self.unread_ascii_char(ch);
                break;
            }

            tail_word = (tail_word << 8) | (ch.ascii_value() as u32);
        }
        self.emit_keyword_or_identifier(tail_word)
    }

    #[inline(always)]
    fn make_tail_word_2(ch0: char, ch1: char) -> u32 {
        ((ch0 as u32) << 8) | (ch1 as u32)
    }
    #[inline(always)]
    fn make_tail_word_3(ch0: char, ch1: char, ch2: char) -> u32 {
        ((ch0 as u32) << 16) | ((ch1 as u32) << 8) | (ch2 as u32)
    }
    #[inline(always)]
    fn make_tail_word_4(ch0: char, ch1: char, ch2: char, ch3: char) -> u32 {
        ((ch0 as u32) << 24) | ((ch1 as u32) << 16) | ((ch2 as u32) << 8) | (ch3 as u32)
    }
    fn emit_keyword_or_identifier(&mut self, tail_word: u32) -> MODE::Tok {
        // Keyword frequencies in JS (from esprima blog post):
        //      this - 3229, function - 3108, if - 3063, return - 2878, var - 2116
        //      else - 562, for - 436, new - 232, in - 225, typeof - 188
        //      while - 143, case - 122, break - 115,
        //      try, catch, delete, throw, switch, continue, default, instanceof
        //      do, void, finally

        if tail_word == Self::make_tail_word_4('t', 'h', 'i', 's') {
            return self.emit_token_check_kw(TokenKind::this_keyword());

        } else if tail_word == Self::make_tail_word_4('t', 'i', 'o', 'n') {
            if self.input_stream.check_ascii_text(&['f','u','n','c'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::function_keyword());
            }

        } else if tail_word == Self::make_tail_word_2('i', 'f') {
            return self.emit_token_check_kw(TokenKind::if_keyword());

        } else if tail_word == Self::make_tail_word_4('t', 'u', 'r', 'n') {
            if self.input_stream.check_ascii_text(&['r','e','t'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::return_keyword());
            }

        } else if tail_word == Self::make_tail_word_3('v', 'a', 'r') {
            return self.emit_token_check_kw(TokenKind::var_keyword());

        } else if tail_word == Self::make_tail_word_4('e', 'l', 's', 'e') {
            return self.emit_token_check_kw(TokenKind::else_keyword());

        } else if tail_word == Self::make_tail_word_3('f', 'o', 'r') {
            return self.emit_token_check_kw(TokenKind::for_keyword());

        } else if tail_word == Self::make_tail_word_3('n', 'e', 'w') {
            return self.emit_token_check_kw(TokenKind::new_keyword());

        } else if tail_word == Self::make_tail_word_2('i', 'n') {
            return self.emit_token_check_kw(TokenKind::in_keyword());

        } else if tail_word == Self::make_tail_word_4('p', 'e', 'o', 'f') {
            if self.input_stream.check_ascii_text(&['t','y'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::typeof_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('h', 'i', 'l', 'e') {
            if self.input_stream.check_ascii_text(&['w'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::while_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('c', 'a', 's', 'e') {
            return self.emit_token_check_kw(TokenKind::case_keyword());

        } else if tail_word == Self::make_tail_word_4('r', 'e', 'a', 'k') {
            if self.input_stream.check_ascii_text(&['b'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::break_keyword());
            }

        } else if tail_word == Self::make_tail_word_3('t', 'r', 'y') {
            return self.emit_token_check_kw(TokenKind::try_keyword());

        } else if tail_word == Self::make_tail_word_4('a', 't', 'c', 'h') {
            if self.input_stream.check_ascii_text(&['c'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::catch_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('l', 'e', 't', 'e') {
            if self.input_stream.check_ascii_text(&['d', 'e'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::delete_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('h', 'r', 'o', 'w') {
            if self.input_stream.check_ascii_text(&['t'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::throw_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('i', 't', 'c', 'h') {
            if self.input_stream.check_ascii_text(&['s', 'w'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::switch_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('i', 'n', 'u', 'e') {
            if self.input_stream.check_ascii_text(&['c','o','n','t'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::continue_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('a', 'u', 'l', 't') {
            if self.input_stream.check_ascii_text(&['d','e','f'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::default_keyword());
            }

        } else if tail_word == Self::make_tail_word_4('c', 'e', 'o', 'f') {
            if self.input_stream.check_ascii_text(&['i','n','s','t','a','n'],
                                                  self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::instanceof_keyword());
            }

        } else if tail_word == Self::make_tail_word_2('d', 'o') {
            return self.emit_token_check_kw(TokenKind::do_keyword());

        } else if tail_word == Self::make_tail_word_4('v', 'o', 'i', 'd') {
            return self.emit_token_check_kw(TokenKind::void_keyword());

        } else if tail_word == Self::make_tail_word_4('a', 'l', 'l', 'y') {
            if self.input_stream.check_ascii_text(&['f','i','n'], self.token_start_position) {
                return self.emit_token_check_kw(TokenKind::finally_keyword());
            }
        }

        self.emit_token_check_kw(TokenKind::identifier())
    }

    fn read_ascii_number_starting_with_zero(&mut self) -> MODE::Tok {
        let ch1 = self.read_ascii_char();
        if ch1.is_char('x') || ch1.is_char('X') {
            return self.read_ascii_hex_number();
        }
        if ch1.is_digit() {
            return self.read_ascii_oct_number();
        }
        if ch1.is_char('.') {
            return self.read_ascii_float_fraction();
        }
        if ch1.is_char('e') || ch1.is_char('E') {
            return self.read_ascii_float_exponent();
        }
        if ch1.is_identifier_continue() {
            return self.emit_error(TokenError::BadNumber);
        }
        if ! ch1.is_ascii_or_end() {
            // TODO: Check for non-ascii identifier and return BadNumber if so.
            return self.emit_error(TokenError::CantHandleUnicodeYet);
        }
        self.unread_ascii_char(ch1);
        return self.emit_token(TokenKind::integer_literal());
    }

    fn read_ascii_hex_number(&mut self) -> MODE::Tok {
        let ch0 = self.read_ascii_char();
        if ! ch0.is_hex_digit() {
            if ! ch0.is_ascii_or_end() {
                // TODO: Read a non-ascii char and return BadNumber.
                return self.emit_error(TokenError::CantHandleUnicodeYet);
            }
            return self.emit_error(TokenError::BadNumber)
        }

        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_hex_digit() {
                // If it's some other identifier character, error out.
                if ch.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }

                // If char is not ascii or end-of-input, unread it and read a unicode char.
                if ! ch.is_ascii_or_end() {
                    // TODO: Handle unicode chars.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }

                // Otherwise it's either end-of-stream or some other char.
                self.unread_ascii_char(ch);
                break;
            }
        }
        self.emit_token(TokenKind::hex_integer_literal())
    }

    fn read_ascii_oct_number(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_oct_digit() {
                // If it's some other identifier character, error out.
                if ch.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }

                // If char is not ascii or end-of-input, unread it and read a unicode char.
                if ! ch.is_ascii_or_end() {
                    // TODO: Handle unicode chars.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }

                // Otherwise it's either end-of-stream or some other char.
                self.unread_ascii_char(ch);
                break;
            }
        }
        self.emit_token(TokenKind::oct_integer_literal())
    }

    fn read_ascii_number(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_digit() {
                if ch.is_char('.') {
                    return self.read_ascii_float_fraction();
                }
                if ch.is_char('e') || ch.is_char('E') {
                    return self.read_ascii_float_exponent();
                }
                if ch.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }
                if ! ch.is_ascii_or_end() {
                    // TODO: Check for non-ascii identifier and return BadNumber if so.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }
                self.unread_ascii_char(ch);
                break;
            }
        }

        return self.emit_token(TokenKind::integer_literal());
    }

    fn read_ascii_float_fraction(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_digit() {
                if ch.is_char('e') || ch.is_char('E') {
                    return self.read_ascii_float_exponent();
                }
                if ch.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }
                if ! ch.is_ascii_or_end() {
                    // TODO: Check for non-ascii identifier and return BadNumber if so.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }
                self.unread_ascii_char(ch);
                break;
            }
        }

        return self.emit_token(TokenKind::float_literal());
    }

    fn read_ascii_float_exponent(&mut self) -> MODE::Tok {
        // Exponent sigil must be followed by digit, optionally preceded by a '+' or '-'.
        let ch0 = self.read_ascii_char();
        if ch0.is_char('+') || ch0.is_char('-') {
            let ch1 = self.read_ascii_char();
            if ! ch1.is_digit() {
                if ch1.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }
                if ! ch1.is_ascii_or_end() {
                    // TODO: Read unicode character and raise error.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }

                if ch1.is_end() {
                    return self.emit_error(TokenError::PrematureEnd(TokenKind::float_literal()));
                }

                self.unread_ascii_char(ch1);
                return self.emit_error(TokenError::BadNumber);
            }
        } else if ! ch0.is_digit() {
            if ch0.is_identifier_continue() {
                return self.emit_error(TokenError::BadNumber);
            }

            if ! ch0.is_ascii_or_end() {
                // TODO: Read unicode character and raise error.
                return self.emit_error(TokenError::CantHandleUnicodeYet);
            }

            if ch0.is_end() {
                return self.emit_error(TokenError::PrematureEnd(TokenKind::float_literal()));
            }

            self.unread_ascii_char(ch0);
            return self.emit_error(TokenError::BadNumber);
        }

        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_digit() {
                if ch.is_identifier_continue() {
                    return self.emit_error(TokenError::BadNumber);
                }
                if ! ch.is_ascii_or_end() {
                    // TODO: Check for non-ascii identifier and return BadNumber if so.
                    return self.emit_error(TokenError::CantHandleUnicodeYet);
                }
                self.unread_ascii_char(ch);
                break;
            }
        }

        return self.emit_token(TokenKind::float_literal());
    }

    fn read_line_comment(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if self.check_and_finish_ascii_newline(ch) {
                break;
            }

            // If char is not ascii or end-of-input, unread it and read a unicode char.
            if ! ch.is_ascii_or_end() {
                // TODO: Handle unicode chars.  Remember to check for unicode newlines.
                return self.emit_error(TokenError::CantHandleUnicodeYet);
            }

            // End-of-input in a line-comment terminates the comment.
            if ch.is_end() {
                break;
            }

            // Otherwise, char is ascii and not a newline.  Continue.
        }
        self.emit_token(TokenKind::comment())
    }

    fn read_block_comment(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            // Check for end of comment.
            if ch.is_char('*') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('/') {
                    break;
                }
                // Unread ch2 because it might be another '*' followed by a '/'.
                self.unread_ascii_char(ch2);
                continue;
            }

            // If tokenizer mode cares, check for new line (to note).
            if self.tokenizer_mode.cares_about_newline() {
                if self.check_and_finish_ascii_newline(ch) {
                    continue;
                }
            }

            // If char is not ascii or end-of-input, unread it and read a unicode char.
            if ! ch.is_ascii_or_end() {
                // TODO: Handle unicode chars.
                return self.emit_error(TokenError::CantHandleUnicodeYet);
            }

            // Check for end of input in the middle of a block comment, which is an error.
            if ch.is_end() {
                return self.emit_error(TokenError::PrematureEnd(TokenKind::newline()));
            }

            // Otherwise, char is ascii and not a comment terminator.  Continue.
        }
        self.emit_token(TokenKind::comment())
    }

    fn check_and_finish_ascii_newline(&mut self, ch: AsciiChar) -> bool {
        if ch.is_line_feed() {
            self.tokenizer_mode.note_newline();
            true
        } else if ch.is_carriage_return() {
            let ch2 = self.read_ascii_char();
            if ! ch2.is_line_feed() {
                self.unread_ascii_char(ch2);
            }
            self.tokenizer_mode.note_newline();
            true
        } else {
            false
        }
    }

    fn emit_error(&mut self, err: TokenError) -> MODE::Tok {
        self.token_error = Some(err);

        let token_end_position = self.input_stream.mark();
        let token_location = TokenLocation::new(self.token_start_position, token_end_position);

        // Error tokens are not saved on the token buffer.
        self.tokenizer_mode.make_token(TokenKind::error(), token_location)
    }

    fn emit_token_impl(&mut self, kind: TokenKind, check_kw: bool) -> MODE::Tok {
        let token_end_position = self.input_stream.mark();
        let token_location = TokenLocation::new(self.token_start_position, token_end_position);
        self.tokenizer_mode.make_token(kind, token_location)
    }

    fn emit_token(&mut self, kind: TokenKind) -> MODE::Tok {
        self.emit_token_impl(kind, /* check_kw = */ false)
    }

    fn emit_token_check_kw(&mut self, kind: TokenKind) -> MODE::Tok {
        self.emit_token_impl(kind, /* check_kw = */ true)
    }

    fn read_ascii_char(&mut self) -> AsciiChar {
        self.input_stream.read_ascii()
    }
    fn unread_ascii_char(&mut self, ch: AsciiChar) {
        self.input_stream.unread_ascii(ch);
    }
}

fn check_single_char_token(ch: AsciiChar) -> TokenKind {
    unsafe {
        *(&SINGLE_CHAR_TOKENS as &[TokenKind]).get_unchecked(ch.octet_value_or_0xff() as usize)
    }
}

static mut SINGLE_CHAR_TOKENS: [TokenKind; 256] = [TokenKind(0); 256];
unsafe fn init_single_char_tokens() {
    for i in 0..255 {
        SINGLE_CHAR_TOKENS[i] = TokenKind::error();
    }

    let update_array = |ch, kind| {
        assert!((ch as usize) <= 256);
        SINGLE_CHAR_TOKENS[ch as usize] = kind;
    };
    update_array('(', TokenKind::open_paren());
    update_array(')', TokenKind::close_paren());
    update_array('[', TokenKind::open_bracket());
    update_array(']', TokenKind::close_bracket());
    update_array('{', TokenKind::open_brace());
    update_array('}', TokenKind::close_brace());
    update_array('.', TokenKind::dot());
    update_array(';', TokenKind::semicolon());
    update_array(',', TokenKind::comma());
    update_array('?', TokenKind::question());
    update_array(':', TokenKind::colon());
    update_array('~', TokenKind::tilde());
}
