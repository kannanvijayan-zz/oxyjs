
/** The enum of all token kinds. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Precedence(u8);
impl Precedence {
    pub fn lowest() -> Precedence {
        Precedence(PREC_LOWEST)
    }
    pub fn comma() -> Precedence {
        Precedence(PREC_COMMA)
    }
    pub fn assignment() -> Precedence {
        Precedence(PREC_ASSIGNMENT)
    }
    pub fn conditional() -> Precedence {
        Precedence(PREC_CONDITIONAL)
    }
    pub fn logical_or() -> Precedence {
        Precedence(PREC_LOGICAL_OR)
    }
    pub fn logical_and() -> Precedence {
        Precedence(PREC_LOGICAL_AND)
    }
    pub fn bitwise_or() -> Precedence {
        Precedence(PREC_BITWISE_OR)
    }
    pub fn bitwise_xor() -> Precedence {
        Precedence(PREC_BITWISE_XOR)
    }
    pub fn bitwise_and() -> Precedence {
        Precedence(PREC_BITWISE_AND)
    }
    pub fn equality() -> Precedence {
        Precedence(PREC_EQUALITY)
    }
    pub fn relational() -> Precedence {
        Precedence(PREC_RELATIONAL)
    }
    pub fn shift() -> Precedence {
        Precedence(PREC_SHIFT)
    }
    pub fn additive() -> Precedence {
        Precedence(PREC_ADDITIVE)
    }
    pub fn multiplicative() -> Precedence {
        Precedence(PREC_MULTIPLICATIVE)
    }
    pub fn unary() -> Precedence {
        Precedence(PREC_UNARY)
    }
    pub fn postfix() -> Precedence {
        Precedence(PREC_POSTFIX)
    }
    pub fn left_hand_side() -> Precedence {
        Precedence(PREC_LEFT_HAND_SIDE)
    }
    pub fn call_or_new() -> Precedence {
        Precedence(PREC_CALL_OR_NEW)
    }
    pub fn member() -> Precedence {
        Precedence(PREC_MEMBER)
    }
    pub fn primary() -> Precedence {
        Precedence(PREC_PRIMARY)
    }
}

const PREC_LOWEST: u8 = 0;

const PREC_COMMA: u8 = PREC_LOWEST + 1;
const PREC_ASSIGNMENT: u8 = PREC_COMMA + 1;
const PREC_CONDITIONAL: u8 = PREC_ASSIGNMENT + 1;
const PREC_LOGICAL_OR: u8 = PREC_CONDITIONAL + 1;
const PREC_LOGICAL_AND: u8 = PREC_LOGICAL_OR + 1;
const PREC_BITWISE_OR: u8 = PREC_LOGICAL_AND + 1;
const PREC_BITWISE_XOR: u8 = PREC_BITWISE_OR + 1;
const PREC_BITWISE_AND: u8 = PREC_BITWISE_XOR + 1;
const PREC_EQUALITY: u8 = PREC_BITWISE_AND + 1;
const PREC_RELATIONAL: u8 = PREC_EQUALITY + 1;
const PREC_SHIFT: u8 = PREC_RELATIONAL + 1;
const PREC_ADDITIVE: u8 = PREC_SHIFT + 1;
const PREC_MULTIPLICATIVE: u8 = PREC_ADDITIVE + 1;
const PREC_UNARY: u8 = PREC_MULTIPLICATIVE + 1;
const PREC_POSTFIX: u8 = PREC_UNARY + 1;
const PREC_LEFT_HAND_SIDE: u8 = PREC_POSTFIX + 1;
const PREC_CALL_OR_NEW: u8 = PREC_LEFT_HAND_SIDE + 1;
const PREC_MEMBER: u8 = PREC_CALL_OR_NEW + 1;
const PREC_PRIMARY: u8 = PREC_MEMBER + 1;

const PREC_HIGHEST: u8 = 0xff;
