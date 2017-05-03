
use parser::input_stream::{InputStream, StreamPosition};
use parser::char_utils::{AsciiChar};

/** The enum of all token kinds. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Error,
    End,

    Whitespace,
    Newline,
    LineComment,
    BlockComment,

    Identifier,
    IntegerLiteral,
    HexIntegerLiteral,
    OctIntegerLiteral,
    FloatLiteral,

    // Braces.
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    // Punctuation.
    Dot,
    Semicolon,
    Comma,
    Question,
    Colon,

    // Comparison
    Equal,
    StrictEqual,
    NotEqual,
    StrictNotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Operators
    Tilde,
    Bang,
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    Percent,
    ShiftLeft,
    ShiftRight,
    ArithmeticShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    LogicalAnd,
    LogicalOr,

    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    ArithmeticShiftRightAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,

    // Keywords
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ContinueKeyword,
    DefaultKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    FinallyKeyword,
    ForKeyword,
    FunctionKeyword,
    IfKeyword,
    InKeyword,
    InstanceofKeyword,
    NewKeyword,
    ReturnKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TryKeyword,
    TypeofKeyword,
    VarKeyword,
    VoidKeyword,
    WhileKeyword
}

pub enum TokenError {
    None,
    PrematureEnd(TokenKind),
    CantHandleUnicodeYet,
    BadNumber
}

/** Raw information required to extract the token from the source text. */
pub struct TokenLocation {
    start_offset: StreamPosition,
    end_offset: StreamPosition
}
impl TokenLocation {
    pub fn new(start_offset: StreamPosition, end_offset: StreamPosition) -> TokenLocation {
        TokenLocation {
            start_offset: start_offset,
            end_offset: end_offset
        }
    }
    pub fn default() -> TokenLocation {
        Self::new(StreamPosition::default(), StreamPosition::default())
    }
}

/**
 * Trait representing the minimal information we care about a token.
 * The implementors of this will be a FullToken or a SyntaxToken, depending
 * on the kind of parse we're doiing.
 */
pub trait Token where Self: Clone + Copy {
    fn make(kind: TokenKind, location: TokenLocation) -> Self;
    fn kind(&self) -> TokenKind;
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
 * The actual tokenizer is parameterized on the input stream type and the
 * token type we're using.
 *
 * The token type will depend on the kind of parse we're doing: full parse or
 * syntax parse.
 */
pub struct Tokenizer<STREAM: InputStream, MODE: TokenizerMode> {
    input_stream: STREAM,
    tokenizer_mode: MODE,
    pushed_back_token: Option<MODE::Tok>,
    token_start_position: StreamPosition,
    token_error: TokenError
}
impl<STREAM, MODE> Tokenizer<STREAM, MODE>
    where STREAM: InputStream,
          MODE: TokenizerMode
{
    pub fn new(input_stream: STREAM, tokenizer_mode: MODE) -> Tokenizer<STREAM, MODE> {
        Tokenizer {
            input_stream: input_stream,
            tokenizer_mode: tokenizer_mode,
            pushed_back_token: None,
            token_start_position: StreamPosition::default(),
            token_error: TokenError::None
        }
    }

    pub fn next_token(&mut self, check_kw: bool) -> MODE::Tok {
        match self.pushed_back_token.take() {
            Option::Some(token) => token,
            Option::None => self.read_token(check_kw)
        }
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
                return self.read_identifier();
            } else {
                return self.read_identifier_or_keyword();
            }
        }

        // Check single-char tokens.
        // Covers: Open/Close Paren/Bracket/Brace, Dot, Semicolon, Comma, Question, Colon, Tilde
        let single_kind = check_single_char_token(ch0.octet_value());
        if single_kind != TokenKind::Error {
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
                return self.emit_token(TokenKind::SlashAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Slash);
        }

        if ch0.is_char('=') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::StrictEqual);
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::Equal);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Assign);
        }

        if ch0.is_char('!') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::StrictNotEqual);
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::NotEqual);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Bang);
        }

        if ch0.is_char('<') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::LessEqual);
            }
            if ch1.is_char('<') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::ShiftLeftAssign);
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::ShiftLeft);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Less);
        }

        if ch0.is_char('>') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::GreaterEqual);
            }
            if ch1.is_char('>') {
                let ch2 = self.read_ascii_char();
                if ch2.is_char('=') {
                    return self.emit_token(TokenKind::ShiftRightAssign);
                }
                if ch2.is_char('>') {
                    let ch3 = self.read_ascii_char();
                    if ch3.is_char('=') {
                        return self.emit_token(TokenKind::ArithmeticShiftRightAssign);
                    }
                    self.unread_ascii_char(ch3);
                    return self.emit_token(TokenKind::ArithmeticShiftRight);
                }
                self.unread_ascii_char(ch2);
                return self.emit_token(TokenKind::ShiftRight);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Greater);
        }

        if ch0.is_char('*') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::StarAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Star);
        }

        if ch0.is_char('+') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('+') {
                return self.emit_token(TokenKind::PlusPlus);
            }
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::PlusAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Plus);
        }

        if ch0.is_char('-') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('-') {
                return self.emit_token(TokenKind::MinusMinus);
            }
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::MinusAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Minus);
        }

        if ch0.is_char('%') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::PercentAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::Percent);
        }

        if ch0.is_char('&') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::BitAndAssign);
            }
            if ch1.is_char('&') {
                return self.emit_token(TokenKind::LogicalAnd);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::BitAnd);
        }

        if ch0.is_char('|') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::BitOrAssign);
            }
            if ch1.is_char('&') {
                return self.emit_token(TokenKind::LogicalOr);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::BitOr);
        }

        if ch0.is_char('^') {
            let ch1 = self.read_ascii_char();
            if ch1.is_char('=') {
                return self.emit_token(TokenKind::BitXorAssign);
            }
            self.unread_ascii_char(ch1);
            return self.emit_token(TokenKind::BitXor);
        }

        self.tokenizer_mode.make_token(TokenKind::Error, TokenLocation::default())
    }

    fn read_whitespace(&mut self) -> MODE::Tok {
        loop {
            let ch = self.read_ascii_char();
            if ! ch.is_whitespace() {
                self.unread_ascii_char(ch);
                break;
            }
        }
        self.emit_token(TokenKind::Whitespace)
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
        self.emit_token(TokenKind::Identifier)
    }

    fn read_identifier_or_keyword(&mut self) -> MODE::Tok {
        // Keep a track of the last 4 bytes of the identifier.
        let mut tail_word = 0 as u32;
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
            return self.emit_token(TokenKind::ThisKeyword);

        } else if tail_word == Self::make_tail_word_4('t', 'i', 'o', 'n') {
            if self.input_stream.check_ascii_text(&['f','u','n','c'], self.token_start_position) {
                return self.emit_token(TokenKind::FunctionKeyword);
            }

        } else if tail_word == Self::make_tail_word_2('i', 'f') {
            return self.emit_token(TokenKind::IfKeyword);

        } else if tail_word == Self::make_tail_word_4('t', 'u', 'r', 'n') {
            if self.input_stream.check_ascii_text(&['r','e','t'], self.token_start_position) {
                return self.emit_token(TokenKind::ReturnKeyword);
            }

        } else if tail_word == Self::make_tail_word_3('v', 'a', 'r') {
            return self.emit_token(TokenKind::VarKeyword);

        } else if tail_word == Self::make_tail_word_4('e', 'l', 's', 'e') {
            return self.emit_token(TokenKind::ElseKeyword);

        } else if tail_word == Self::make_tail_word_3('f', 'o', 'r') {
            return self.emit_token(TokenKind::ForKeyword);

        } else if tail_word == Self::make_tail_word_3('n', 'e', 'w') {
            return self.emit_token(TokenKind::NewKeyword);

        } else if tail_word == Self::make_tail_word_2('i', 'n') {
            return self.emit_token(TokenKind::InKeyword);

        } else if tail_word == Self::make_tail_word_4('p', 'e', 'o', 'f') {
            if self.input_stream.check_ascii_text(&['t','y'], self.token_start_position) {
                return self.emit_token(TokenKind::TypeofKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('h', 'i', 'l', 'e') {
            if self.input_stream.check_ascii_text(&['w'], self.token_start_position) {
                return self.emit_token(TokenKind::WhileKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('c', 'a', 's', 'e') {
            return self.emit_token(TokenKind::CaseKeyword);

        } else if tail_word == Self::make_tail_word_4('r', 'e', 'a', 'k') {
            if self.input_stream.check_ascii_text(&['b'], self.token_start_position) {
                return self.emit_token(TokenKind::BreakKeyword);
            }

        } else if tail_word == Self::make_tail_word_3('t', 'r', 'y') {
            return self.emit_token(TokenKind::TryKeyword);

        } else if tail_word == Self::make_tail_word_4('a', 't', 'c', 'h') {
            if self.input_stream.check_ascii_text(&['c'], self.token_start_position) {
                return self.emit_token(TokenKind::CatchKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('l', 'e', 't', 'e') {
            if self.input_stream.check_ascii_text(&['d', 'e'], self.token_start_position) {
                return self.emit_token(TokenKind::DeleteKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('h', 'r', 'o', 'w') {
            if self.input_stream.check_ascii_text(&['t'], self.token_start_position) {
                return self.emit_token(TokenKind::ThrowKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('i', 't', 'c', 'h') {
            if self.input_stream.check_ascii_text(&['s', 'w'], self.token_start_position) {
                return self.emit_token(TokenKind::SwitchKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('i', 'n', 'u', 'e') {
            if self.input_stream.check_ascii_text(&['c','o','n','t'], self.token_start_position) {
                return self.emit_token(TokenKind::ContinueKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('a', 'u', 'l', 't') {
            if self.input_stream.check_ascii_text(&['d','e','f'], self.token_start_position) {
                return self.emit_token(TokenKind::DefaultKeyword);
            }

        } else if tail_word == Self::make_tail_word_4('c', 'e', 'o', 'f') {
            if self.input_stream.check_ascii_text(&['i','n','s','t','a','n'],
                                                  self.token_start_position) {
                return self.emit_token(TokenKind::DefaultKeyword);
            }

        } else if tail_word == Self::make_tail_word_2('d', 'o') {
            return self.emit_token(TokenKind::DoKeyword);

        } else if tail_word == Self::make_tail_word_4('v', 'o', 'i', 'd') {
            return self.emit_token(TokenKind::VoidKeyword);

        } else if tail_word == Self::make_tail_word_4('a', 'l', 'l', 'y') {
            if self.input_stream.check_ascii_text(&['f','i','n'], self.token_start_position) {
                return self.emit_token(TokenKind::FinallyKeyword);
            }
        }

        self.emit_token(TokenKind::Identifier)
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
        return self.emit_token(TokenKind::IntegerLiteral);
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
        self.emit_token(TokenKind::HexIntegerLiteral)
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
        self.emit_token(TokenKind::OctIntegerLiteral)
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

        return self.emit_token(TokenKind::IntegerLiteral);
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

        return self.emit_token(TokenKind::FloatLiteral);
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
                    return self.emit_error(TokenError::PrematureEnd(TokenKind::FloatLiteral));
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
                return self.emit_error(TokenError::PrematureEnd(TokenKind::FloatLiteral));
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

        return self.emit_token(TokenKind::FloatLiteral);
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
        self.emit_token(TokenKind::LineComment)
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
                return self.emit_error(TokenError::PrematureEnd(TokenKind::Newline));
            }

            // Otherwise, char is ascii and not a comment terminator.  Continue.
        }
        self.emit_token(TokenKind::BlockComment)
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
        self.token_error = err;

        let token_end_position = self.input_stream.mark();
        let token_location = TokenLocation::new(self.token_start_position, token_end_position);
        self.tokenizer_mode.make_token(TokenKind::Error, token_location)
    }

    fn emit_token(&mut self, kind: TokenKind) -> MODE::Tok {
        let token_end_position = self.input_stream.mark();
        let token_location = TokenLocation::new(self.token_start_position, token_end_position);
        self.tokenizer_mode.make_token(kind, token_location)
    }

    fn read_ascii_char(&mut self) -> AsciiChar {
        self.input_stream.read_ascii()
    }
    fn unread_ascii_char(&mut self, ch: AsciiChar) {
        self.input_stream.unread_ascii(ch);
    }
}

fn check_single_char_token(octet: u8) -> TokenKind {
    * unsafe { SINGLE_CHAR_TOKENS.get_unchecked(octet as usize) }
}
lazy_static! {
    static ref SINGLE_CHAR_TOKENS: Vec<TokenKind> = {
        let mut vec = Vec::with_capacity(256);
        for i in 0..256 {
            vec[i] = TokenKind::Error;
        }

        {
            let mut update_vec = |ch, kind| {
                assert!((ch as usize) <= 256);
                vec[ch as usize] = kind;
            };
            update_vec('(', TokenKind::OpenParen);
            update_vec(')', TokenKind::CloseParen);
            update_vec('[', TokenKind::OpenBracket);
            update_vec(']', TokenKind::CloseBracket);
            update_vec('{', TokenKind::OpenBrace);
            update_vec('}', TokenKind::CloseBrace);
            update_vec('.', TokenKind::Dot);
            update_vec(';', TokenKind::Semicolon);
            update_vec(',', TokenKind::Comma);
            update_vec('?', TokenKind::Question);
            update_vec(':', TokenKind::Colon);
            update_vec('~', TokenKind::Tilde);
        }
        vec
    };
}
