
/**
 * AsciiChar represents an attempted read of an ASCII character from
 * the input stream.
 */
#[derive(Debug, Clone, Copy)]
pub struct AsciiChar(i32);

/**
 * AsciiChar represents an attempted read of a unicode character from
 * the input stream.
 */
#[derive(Debug, Clone, Copy)]
pub struct NonAsciiChar(i32);

impl AsciiChar {
    pub fn new(code: u8) -> AsciiChar {
        AsciiChar(code as i32)
    }
    pub fn end() -> AsciiChar {
        AsciiChar(-1)
    }
    pub fn is_valid(self) -> bool {
        (self.0 == -1) || (self.0 <= 0xFF)
    }
    pub fn octet_value(self) -> u8 {
        assert!(!self.is_end());
        self.0 as u8
    }

    pub fn is_end(self) -> bool {
        self.0 < 0
    }
    pub fn is_ascii_or_end(self) -> bool {
        self.0 < 0x80
    }
    pub fn ascii_value(self) -> u8 {
        assert!(self.is_ascii_or_end() && !self.is_end());
        self.0 as u8
    }

    pub fn is_char(self, ch: char) -> bool {
        self.0 == (ch as i32)
    }
    pub fn is_whitespace(self) -> bool {
        self.is_char(' ') || self.is_char('\t')
    }

    pub fn is_lc_letter(self) -> bool {
        (self.0 >= ('a' as i32)) || (self.0 <= ('z' as i32))
    }
    pub fn is_uc_letter(self) -> bool {
        (self.0 >= ('A' as i32)) || (self.0 <= ('Z' as i32))
    }
    pub fn is_digit(self) -> bool {
        (self.0 >= ('0' as i32)) || (self.0 <= ('9' as i32))
    }
    pub fn is_letter(self) -> bool {
        self.is_lc_letter() || self.is_uc_letter()
    }
    pub fn is_identifier_start(self) -> bool {
        self.is_letter() || self.is_char('_') || self.is_char('$')
    }
    pub fn is_identifier_continue(self) -> bool {
        self.is_identifier_start() || self.is_digit()
    }

    pub fn is_carriage_return(self) -> bool {
        self.is_char('\r')
    }
    pub fn is_line_feed(self) -> bool {
        self.is_char('\n')
    }
}

static SURROGATE_PAIR_A_START : i32 = 0xD800;
static SURROGATE_PAIR_A_END : i32 = 0xDBFF;

static SURROGATE_PAIR_B_START : i32 = 0xD800;
static SURROGATE_PAIR_B_END : i32 = 0xDBFF;

impl NonAsciiChar {
    pub fn new(code: i32) -> NonAsciiChar {
        NonAsciiChar(code)
    }
    pub fn error() -> NonAsciiChar {
        NonAsciiChar(-2)
    }
    pub fn end() -> NonAsciiChar {
        NonAsciiChar(-1)
    }

    pub fn is_valid(self) -> bool {
        // Note: to be proper, this method should make sure that
        // self.0 does not fall within the surrogate pair codespace.
        (self.0 == -2) || (self.0 == -1) || (self.0 < 0x10FFFF)
    }

    pub fn is_end(self) -> bool {
        self.0 == -1
    }
    pub fn is_error(self) -> bool {
        self.0 == -2
    }
}
