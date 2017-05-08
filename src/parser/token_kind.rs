
// TODO: Use macros to make this file not suck so much.

/** The enum of all token kinds. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenKind(u8);
impl TokenKind {
    pub fn name(&self) -> &'static str {
        TOKEN_STRINGS[self.0 as usize]
    }

    pub fn error() -> TokenKind {
         TokenKind(TOK_ERROR.0)
    }
    pub fn is_error(&self) -> bool {
        self.0 == TOK_ERROR.0
    }

    pub fn end() -> TokenKind {
        TokenKind(TOK_END.0)
    }
    pub fn is_end(&self) -> bool {
        self.0 == TOK_END.0
    }


    pub fn whitespace() -> TokenKind {
        TokenKind(TOK_WHITESPACE.0)
    }
    pub fn is_whitespace(&self) -> bool {
        self.0 == TOK_WHITESPACE.0
    }

    pub fn newline() -> TokenKind {
        TokenKind(TOK_NEWLINE.0)
    }
    pub fn is_newline(&self) -> bool {
        self.0 == TOK_NEWLINE.0
    }

    pub fn comment() -> TokenKind {
        TokenKind(TOK_COMMENT.0)
    }
    pub fn is_comment(&self) -> bool {
        self.0 == TOK_COMMENT.0
    }


    pub fn identifier() -> TokenKind {
        TokenKind(TOK_IDENTIFIER.0)
    }
    pub fn is_identifier(&self) -> bool {
        self.0 == TOK_IDENTIFIER.0
    }

    pub fn integer_literal() -> TokenKind {
        TokenKind(TOK_INTEGER_LITERAL.0)
    }
    pub fn is_integer_literal(&self) -> bool {
        self.0 == TOK_INTEGER_LITERAL.0
    }

    pub fn hex_integer_literal() -> TokenKind {
        TokenKind(TOK_HEX_INTEGER_LITERAL.0)
    }
    pub fn is_hex_integer_literal(&self) -> bool {
        self.0 == TOK_HEX_INTEGER_LITERAL.0
    }

    pub fn oct_integer_literal() -> TokenKind {
        TokenKind(TOK_OCT_INTEGER_LITERAL.0)
    }
    pub fn is_oct_integer_literal(&self) -> bool {
        self.0 == TOK_OCT_INTEGER_LITERAL.0
    }

    pub fn float_literal() -> TokenKind {
        TokenKind(TOK_FLOAT_LITERAL.0)
    }
    pub fn is_float_literal(&self) -> bool {
        self.0 == TOK_FLOAT_LITERAL.0
    }


    pub fn open_paren() -> TokenKind {
        TokenKind(TOK_OPEN_PAREN.0)
    }
    pub fn is_open_paren(&self) -> bool {
        self.0 == TOK_OPEN_PAREN.0
    }

    pub fn close_paren() -> TokenKind {
        TokenKind(TOK_CLOSE_PAREN.0)
    }
    pub fn is_close_paren(&self) -> bool {
        self.0 == TOK_CLOSE_PAREN.0
    }

    pub fn open_bracket() -> TokenKind {
        TokenKind(TOK_OPEN_BRACKET.0)
    }
    pub fn is_open_bracket(&self) -> bool {
        self.0 == TOK_OPEN_BRACKET.0
    }

    pub fn close_bracket() -> TokenKind {
        TokenKind(TOK_CLOSE_BRACKET.0)
    }
    pub fn is_close_bracket(&self) -> bool {
        self.0 == TOK_CLOSE_BRACKET.0
    }

    pub fn open_brace() -> TokenKind {
        TokenKind(TOK_OPEN_BRACE.0)
    }
    pub fn is_open_brace(&self) -> bool {
        self.0 == TOK_OPEN_BRACE.0
    }

    pub fn close_brace() -> TokenKind {
        TokenKind(TOK_CLOSE_BRACE.0)
    }
    pub fn is_close_brace(&self) -> bool {
        self.0 == TOK_CLOSE_BRACE.0
    }


    pub fn dot() -> TokenKind {
        TokenKind(TOK_DOT.0)
    }
    pub fn is_dot(&self) -> bool {
        self.0 == TOK_DOT.0
    }

    pub fn semicolon() -> TokenKind {
        TokenKind(TOK_SEMICOLON.0)
    }
    pub fn is_semicolon(&self) -> bool {
        self.0 == TOK_SEMICOLON.0
    }

    pub fn comma() -> TokenKind {
        TokenKind(TOK_COMMA.0)
    }
    pub fn is_comma(&self) -> bool {
        self.0 == TOK_COMMA.0
    }

    pub fn question() -> TokenKind {
        TokenKind(TOK_QUESTION.0)
    }
    pub fn is_question(&self) -> bool {
        self.0 == TOK_QUESTION.0
    }

    pub fn colon() -> TokenKind {
        TokenKind(TOK_COLON.0)
    }
    pub fn is_colon(&self) -> bool {
        self.0 == TOK_COLON.0
    }


    pub fn equal() -> TokenKind {
        TokenKind(TOK_EQUAL.0)
    }
    pub fn is_equal(&self) -> bool {
        self.0 == TOK_EQUAL.0
    }

    pub fn strict_equal() -> TokenKind {
        TokenKind(TOK_STRICT_EQUAL.0)
    }
    pub fn is_strict_equal(&self) -> bool {
        self.0 == TOK_STRICT_EQUAL.0
    }

    pub fn not_equal() -> TokenKind {
        TokenKind(TOK_NOT_EQUAL.0)
    }
    pub fn is_not_equal(&self) -> bool {
        self.0 == TOK_NOT_EQUAL.0
    }

    pub fn strict_not_equal() -> TokenKind {
        TokenKind(TOK_STRICT_NOT_EQUAL.0)
    }
    pub fn is_strict_not_equal(&self) -> bool {
        self.0 == TOK_STRICT_NOT_EQUAL.0
    }

    pub fn less() -> TokenKind {
        TokenKind(TOK_LESS.0)
    }
    pub fn is_less(&self) -> bool {
        self.0 == TOK_LESS.0
    }

    pub fn less_equal() -> TokenKind {
        TokenKind(TOK_LESS_EQUAL.0)
    }
    pub fn is_less_equal(&self) -> bool {
        self.0 == TOK_LESS_EQUAL.0
    }

    pub fn greater() -> TokenKind {
        TokenKind(TOK_GREATER.0)
    }
    pub fn is_greater(&self) -> bool {
        self.0 == TOK_GREATER.0
    }

    pub fn greater_equal() -> TokenKind {
        TokenKind(TOK_GREATER_EQUAL.0)
    }
    pub fn is_greater_equal(&self) -> bool {
        self.0 == TOK_GREATER_EQUAL.0
    }


    pub fn tilde() -> TokenKind {
        TokenKind(TOK_TILDE.0)
    }
    pub fn is_tilde(&self) -> bool {
        self.0 == TOK_TILDE.0
    }

    pub fn bang() -> TokenKind {
        TokenKind(TOK_BANG.0)
    }
    pub fn is_bang(&self) -> bool {
        self.0 == TOK_BANG.0
    }

    pub fn plus() -> TokenKind {
        TokenKind(TOK_PLUS.0)
    }
    pub fn is_plus(&self) -> bool {
        self.0 == TOK_PLUS.0
    }

    pub fn plus_plus() -> TokenKind {
        TokenKind(TOK_PLUS_PLUS.0)
    }
    pub fn is_plus_plus(&self) -> bool {
        self.0 == TOK_PLUS_PLUS.0
    }

    pub fn minus() -> TokenKind {
        TokenKind(TOK_MINUS.0)
    }
    pub fn is_minus(&self) -> bool {
        self.0 == TOK_MINUS.0
    }

    pub fn minus_minus() -> TokenKind {
        TokenKind(TOK_MINUS_MINUS.0)
    }
    pub fn is_minus_minus(&self) -> bool {
        self.0 == TOK_MINUS_MINUS.0
    }

    pub fn star() -> TokenKind {
        TokenKind(TOK_STAR.0)
    }
    pub fn is_star(&self) -> bool {
        self.0 == TOK_STAR.0
    }

    pub fn slash() -> TokenKind {
        TokenKind(TOK_SLASH.0)
    }
    pub fn is_slash(&self) -> bool {
        self.0 == TOK_SLASH.0
    }

    pub fn percent() -> TokenKind {
        TokenKind(TOK_PERCENT.0)
    }
    pub fn is_percent(&self) -> bool {
        self.0 == TOK_PERCENT.0
    }

    pub fn shift_left() -> TokenKind {
        TokenKind(TOK_SHIFT_LEFT.0)
    }
    pub fn is_shift_left(&self) -> bool {
        self.0 == TOK_SHIFT_LEFT.0
    }

    pub fn shift_right() -> TokenKind {
        TokenKind(TOK_SHIFT_RIGHT.0)
    }
    pub fn is_shift_right(&self) -> bool {
        self.0 == TOK_SHIFT_RIGHT.0
    }

    pub fn arithmetic_shift_right() -> TokenKind {
        TokenKind(TOK_ARITHMETIC_SHIFT_RIGHT.0)
    }
    pub fn is_arithmetic_shift_right(&self) -> bool {
        self.0 == TOK_ARITHMETIC_SHIFT_RIGHT.0
    }

    pub fn bit_and() -> TokenKind {
        TokenKind(TOK_BIT_AND.0)
    }
    pub fn is_bit_and(&self) -> bool {
        self.0 == TOK_BIT_AND.0
    }

    pub fn bit_or() -> TokenKind {
        TokenKind(TOK_BIT_OR.0)
    }
    pub fn is_bit_or(&self) -> bool {
        self.0 == TOK_BIT_OR.0
    }

    pub fn bit_xor() -> TokenKind {
        TokenKind(TOK_BIT_XOR.0)
    }
    pub fn is_bit_xor(&self) -> bool {
        self.0 == TOK_BIT_XOR.0
    }

    pub fn logical_and() -> TokenKind {
        TokenKind(TOK_LOGICAL_AND.0)
    }
    pub fn is_logical_and(&self) -> bool {
        self.0 == TOK_LOGICAL_AND.0
    }

    pub fn logical_or() -> TokenKind {
        TokenKind(TOK_LOGICAL_OR.0)
    }
    pub fn is_logical_or(&self) -> bool {
        self.0 == TOK_LOGICAL_OR.0
    }


    pub fn assign() -> TokenKind {
        TokenKind(TOK_ASSIGN.0)
    }
    pub fn is_assign(&self) -> bool {
        self.0 == TOK_ASSIGN.0
    }

    pub fn plus_assign() -> TokenKind {
        TokenKind(TOK_PLUS_ASSIGN.0)
    }
    pub fn is_plus_assign(&self) -> bool {
        self.0 == TOK_PLUS_ASSIGN.0
    }

    pub fn minus_assign() -> TokenKind {
        TokenKind(TOK_MINUS_ASSIGN.0)
    }
    pub fn is_minus_assign(&self) -> bool {
        self.0 == TOK_MINUS_ASSIGN.0
    }

    pub fn star_assign() -> TokenKind {
        TokenKind(TOK_STAR_ASSIGN.0)
    }
    pub fn is_star_assign(&self) -> bool {
        self.0 == TOK_STAR_ASSIGN.0
    }

    pub fn slash_assign() -> TokenKind {
        TokenKind(TOK_SLASH_ASSIGN.0)
    }
    pub fn is_slash_assign(&self) -> bool {
        self.0 == TOK_SLASH_ASSIGN.0
    }

    pub fn percent_assign() -> TokenKind {
        TokenKind(TOK_PERCENT_ASSIGN.0)
    }
    pub fn is_percent_assign(&self) -> bool {
        self.0 == TOK_PERCENT_ASSIGN.0
    }

    pub fn shift_left_assign() -> TokenKind {
        TokenKind(TOK_SHIFT_LEFT_ASSIGN.0)
    }
    pub fn is_shift_left_assign(&self) -> bool {
        self.0 == TOK_SHIFT_LEFT_ASSIGN.0
    }

    pub fn shift_right_assign() -> TokenKind {
        TokenKind(TOK_SHIFT_RIGHT_ASSIGN.0)
    }
    pub fn is_shift_right_assign(&self) -> bool {
        self.0 == TOK_SHIFT_RIGHT_ASSIGN.0
    }

    pub fn arithmetic_shift_right_assign() -> TokenKind {
        TokenKind(TOK_ARITHMETIC_SHIFT_RIGHT_ASSIGN.0)
    }
    pub fn is_arithmetic_shift_right_assign(&self) -> bool {
        self.0 == TOK_ARITHMETIC_SHIFT_RIGHT_ASSIGN.0
    }

    pub fn bit_and_assign() -> TokenKind {
        TokenKind(TOK_BIT_AND_ASSIGN.0)
    }
    pub fn is_bit_and_assign(&self) -> bool {
        self.0 == TOK_BIT_AND_ASSIGN.0
    }

    pub fn bit_or_assign() -> TokenKind {
        TokenKind(TOK_BIT_OR_ASSIGN.0)
    }
    pub fn is_bit_or_assign(&self) -> bool {
        self.0 == TOK_BIT_OR_ASSIGN.0
    }

    pub fn bit_xor_assign() -> TokenKind {
        TokenKind(TOK_BIT_XOR_ASSIGN.0)
    }
    pub fn is_bit_xor_assign(&self) -> bool {
        self.0 == TOK_BIT_XOR_ASSIGN.0
    }

    pub fn is_assignment_op(&self) -> bool {
        (self.0 >= MIN_ASSIGN_TOK_ID) && (self.0 <= MAX_ASSIGN_TOK_ID)
    }

    pub fn break_keyword() -> TokenKind {
        TokenKind(TOK_BREAK_KEYWORD.0)
    }
    pub fn is_break_keyword(&self) -> bool {
        self.0 == TOK_BREAK_KEYWORD.0
    }

    pub fn case_keyword() -> TokenKind {
        TokenKind(TOK_CASE_KEYWORD.0)
    }
    pub fn is_case_keyword(&self) -> bool {
        self.0 == TOK_CASE_KEYWORD.0
    }

    pub fn catch_keyword() -> TokenKind {
        TokenKind(TOK_CATCH_KEYWORD.0)
    }
    pub fn is_catch_keyword(&self) -> bool {
        self.0 == TOK_CATCH_KEYWORD.0
    }

    pub fn continue_keyword() -> TokenKind {
        TokenKind(TOK_CONTINUE_KEYWORD.0)
    }
    pub fn is_continue_keyword(&self) -> bool {
        self.0 == TOK_CONTINUE_KEYWORD.0
    }

    pub fn default_keyword() -> TokenKind {
        TokenKind(TOK_DEFAULT_KEYWORD.0)
    }
    pub fn is_default_keyword(&self) -> bool {
        self.0 == TOK_DEFAULT_KEYWORD.0
    }

    pub fn delete_keyword() -> TokenKind {
        TokenKind(TOK_DELETE_KEYWORD.0)
    }
    pub fn is_delete_keyword(&self) -> bool {
        self.0 == TOK_DELETE_KEYWORD.0
    }

    pub fn do_keyword() -> TokenKind {
        TokenKind(TOK_DO_KEYWORD.0)
    }
    pub fn is_do_keyword(&self) -> bool {
        self.0 == TOK_DO_KEYWORD.0
    }

    pub fn else_keyword() -> TokenKind {
        TokenKind(TOK_ELSE_KEYWORD.0)
    }
    pub fn is_else_keyword(&self) -> bool {
        self.0 == TOK_ELSE_KEYWORD.0
    }

    pub fn finally_keyword() -> TokenKind {
        TokenKind(TOK_FINALLY_KEYWORD.0)
    }
    pub fn is_finally_keyword(&self) -> bool {
        self.0 == TOK_FINALLY_KEYWORD.0
    }

    pub fn for_keyword() -> TokenKind {
        TokenKind(TOK_FOR_KEYWORD.0)
    }
    pub fn is_for_keyword(&self) -> bool {
        self.0 == TOK_FOR_KEYWORD.0
    }

    pub fn function_keyword() -> TokenKind {
        TokenKind(TOK_FUNCTION_KEYWORD.0)
    }
    pub fn is_function_keyword(&self) -> bool {
        self.0 == TOK_FUNCTION_KEYWORD.0
    }

    pub fn if_keyword() -> TokenKind {
        TokenKind(TOK_IF_KEYWORD.0)
    }
    pub fn is_if_keyword(&self) -> bool {
        self.0 == TOK_IF_KEYWORD.0
    }

    pub fn in_keyword() -> TokenKind {
        TokenKind(TOK_IN_KEYWORD.0)
    }
    pub fn is_in_keyword(&self) -> bool {
        self.0 == TOK_IN_KEYWORD.0
    }

    pub fn instanceof_keyword() -> TokenKind {
        TokenKind(TOK_INSTANCEOF_KEYWORD.0)
    }
    pub fn is_instanceof_keyword(&self) -> bool {
        self.0 == TOK_INSTANCEOF_KEYWORD.0
    }

    pub fn new_keyword() -> TokenKind {
        TokenKind(TOK_NEW_KEYWORD.0)
    }
    pub fn is_new_keyword(&self) -> bool {
        self.0 == TOK_NEW_KEYWORD.0
    }

    pub fn return_keyword() -> TokenKind {
        TokenKind(TOK_RETURN_KEYWORD.0)
    }
    pub fn is_return_keyword(&self) -> bool {
        self.0 == TOK_RETURN_KEYWORD.0
    }

    pub fn switch_keyword() -> TokenKind {
        TokenKind(TOK_SWITCH_KEYWORD.0)
    }
    pub fn is_switch_keyword(&self) -> bool {
        self.0 == TOK_SWITCH_KEYWORD.0
    }

    pub fn this_keyword() -> TokenKind {
        TokenKind(TOK_THIS_KEYWORD.0)
    }
    pub fn is_this_keyword(&self) -> bool {
        self.0 == TOK_THIS_KEYWORD.0
    }

    pub fn throw_keyword() -> TokenKind {
        TokenKind(TOK_THROW_KEYWORD.0)
    }
    pub fn is_throw_keyword(&self) -> bool {
        self.0 == TOK_THROW_KEYWORD.0
    }

    pub fn try_keyword() -> TokenKind {
        TokenKind(TOK_TRY_KEYWORD.0)
    }
    pub fn is_try_keyword(&self) -> bool {
        self.0 == TOK_TRY_KEYWORD.0
    }

    pub fn typeof_keyword() -> TokenKind {
        TokenKind(TOK_TYPEOF_KEYWORD.0)
    }
    pub fn is_typeof_keyword(&self) -> bool {
        self.0 == TOK_TYPEOF_KEYWORD.0
    }

    pub fn var_keyword() -> TokenKind {
        TokenKind(TOK_VAR_KEYWORD.0)
    }
    pub fn is_var_keyword(&self) -> bool {
        self.0 == TOK_VAR_KEYWORD.0
    }

    pub fn void_keyword() -> TokenKind {
        TokenKind(TOK_VOID_KEYWORD.0)
    }
    pub fn is_void_keyword(&self) -> bool {
        self.0 == TOK_VOID_KEYWORD.0
    }

    pub fn while_keyword() -> TokenKind {
        TokenKind(TOK_WHILE_KEYWORD.0)
    }
    pub fn is_while_keyword(&self) -> bool {
        self.0 == TOK_WHILE_KEYWORD.0
    }

    pub fn is_keyword(&self) -> bool {
        (self.0 >= MIN_KEYWORD_TOK_ID) && (self.0 <= MAX_KEYWORD_TOK_ID)
    }
}

const TOK_ERROR: (u8, &'static str) = (0, "error");
const TOK_END: (u8, &'static str) = (TOK_ERROR.0 + 1, "end");

const TOK_WHITESPACE: (u8, &'static str) = (TOK_END.0 + 1, "whitespace");
const TOK_NEWLINE: (u8, &'static str) = (TOK_WHITESPACE.0 + 1, "newline");
const TOK_COMMENT: (u8, &'static str) = (TOK_NEWLINE.0 + 1, "comment");

const TOK_IDENTIFIER: (u8, &'static str) = (TOK_COMMENT.0 + 1, "identifier");
const TOK_INTEGER_LITERAL: (u8, &'static str) = (TOK_IDENTIFIER.0 + 1, "integer_literal");
const TOK_HEX_INTEGER_LITERAL: (u8, &'static str) = (TOK_INTEGER_LITERAL.0 + 1, "hex_integer_literal");
const TOK_OCT_INTEGER_LITERAL: (u8, &'static str) = (TOK_HEX_INTEGER_LITERAL.0 + 1, "oct_integer_literal");
const TOK_FLOAT_LITERAL: (u8, &'static str) = (TOK_OCT_INTEGER_LITERAL.0 + 1, "float_literal");

// Braces.
const TOK_OPEN_PAREN: (u8, &'static str) = (TOK_FLOAT_LITERAL.0 + 1, "open_paren");
const TOK_CLOSE_PAREN: (u8, &'static str) = (TOK_OPEN_PAREN.0 + 1, "close_paren");
const TOK_OPEN_BRACKET: (u8, &'static str) = (TOK_CLOSE_PAREN.0 + 1, "open_bracket");
const TOK_CLOSE_BRACKET: (u8, &'static str) = (TOK_OPEN_BRACKET.0 + 1, "close_bracket");
const TOK_OPEN_BRACE: (u8, &'static str) = (TOK_CLOSE_BRACKET.0 + 1, "open_brace");
const TOK_CLOSE_BRACE: (u8, &'static str) = (TOK_OPEN_BRACE.0 + 1, "close_brace");

// Punctuation.
const TOK_DOT: (u8, &'static str) = (TOK_CLOSE_BRACE.0 + 1, "dot");
const TOK_SEMICOLON: (u8, &'static str) = (TOK_DOT.0 + 1, "semicolon");
const TOK_COMMA: (u8, &'static str) = (TOK_SEMICOLON.0 + 1, "comma");
const TOK_QUESTION: (u8, &'static str) = (TOK_COMMA.0 + 1, "question");
const TOK_COLON: (u8, &'static str) = (TOK_QUESTION.0 + 1, "colon");

// Comparison
const TOK_EQUAL: (u8, &'static str) = (TOK_COLON.0 + 1, "equal");
const TOK_STRICT_EQUAL: (u8, &'static str) = (TOK_EQUAL.0 + 1, "strict_equal");
const TOK_NOT_EQUAL: (u8, &'static str) = (TOK_STRICT_EQUAL.0 + 1, "not_equal");
const TOK_STRICT_NOT_EQUAL: (u8, &'static str) = (TOK_NOT_EQUAL.0 + 1, "strict_not_equal");
const TOK_LESS: (u8, &'static str) = (TOK_STRICT_NOT_EQUAL.0 + 1, "less");
const TOK_LESS_EQUAL: (u8, &'static str) = (TOK_LESS.0 + 1, "less_equal");
const TOK_GREATER: (u8, &'static str) = (TOK_LESS_EQUAL.0 + 1, "greater");
const TOK_GREATER_EQUAL: (u8, &'static str) = (TOK_GREATER.0 + 1, "greater_equal");

// Operator
const TOK_TILDE: (u8, &'static str) = (TOK_GREATER_EQUAL.0 + 1, "tilde");
const TOK_BANG: (u8, &'static str) = (TOK_TILDE.0 + 1, "bang");
const TOK_PLUS: (u8, &'static str) = (TOK_BANG.0 + 1, "plus");
const TOK_PLUS_PLUS: (u8, &'static str) = (TOK_PLUS.0 + 1, "plus_plus");
const TOK_MINUS: (u8, &'static str) = (TOK_PLUS_PLUS.0 + 1, "minus");
const TOK_MINUS_MINUS: (u8, &'static str) = (TOK_MINUS.0 + 1, "minus_minus");
const TOK_STAR: (u8, &'static str) = (TOK_MINUS_MINUS.0 + 1, "star");
const TOK_SLASH: (u8, &'static str) = (TOK_STAR.0 + 1, "slash");
const TOK_PERCENT: (u8, &'static str) = (TOK_SLASH.0 + 1, "percent");
const TOK_SHIFT_LEFT: (u8, &'static str) = (TOK_PERCENT.0 + 1, "shift_left");
const TOK_SHIFT_RIGHT: (u8, &'static str) = (TOK_SHIFT_LEFT.0 + 1, "shift_right");
const TOK_ARITHMETIC_SHIFT_RIGHT: (u8, &'static str) = (TOK_SHIFT_RIGHT.0 + 1, "arithmetic_shift_right");
const TOK_BIT_AND: (u8, &'static str) = (TOK_ARITHMETIC_SHIFT_RIGHT.0 + 1, "bit_and");
const TOK_BIT_OR: (u8, &'static str) = (TOK_BIT_AND.0 + 1, "bit_or");
const TOK_BIT_XOR: (u8, &'static str) = (TOK_BIT_OR.0 + 1, "bit_xor");
const TOK_LOGICAL_AND: (u8, &'static str) = (TOK_BIT_XOR.0 + 1, "logical_and");
const TOK_LOGICAL_OR: (u8, &'static str) = (TOK_LOGICAL_AND.0 + 1, "logical_or");

// Assignment
const TOK_ASSIGN: (u8, &'static str) = (TOK_LOGICAL_OR.0 + 1, "assign");
const TOK_PLUS_ASSIGN: (u8, &'static str) = (TOK_ASSIGN.0 + 1, "plus_assign");
const TOK_MINUS_ASSIGN: (u8, &'static str) = (TOK_PLUS_ASSIGN.0 + 1, "minus_assign");
const TOK_STAR_ASSIGN: (u8, &'static str) = (TOK_MINUS_ASSIGN.0 + 1, "star_assign");
const TOK_SLASH_ASSIGN: (u8, &'static str) = (TOK_STAR_ASSIGN.0 + 1, "slash_assign");
const TOK_PERCENT_ASSIGN: (u8, &'static str) = (TOK_SLASH_ASSIGN.0 + 1, "percent_assign");
const TOK_SHIFT_LEFT_ASSIGN: (u8, &'static str) = (TOK_PERCENT_ASSIGN.0 + 1, "shift_left_assign");
const TOK_SHIFT_RIGHT_ASSIGN: (u8, &'static str) = (TOK_SHIFT_LEFT_ASSIGN.0 + 1, "shift_right_assign");
const TOK_ARITHMETIC_SHIFT_RIGHT_ASSIGN: (u8, &'static str) = (TOK_SHIFT_RIGHT_ASSIGN.0 + 1, "arithmetic_shift_right_assign");
const TOK_BIT_AND_ASSIGN: (u8, &'static str) = (TOK_ARITHMETIC_SHIFT_RIGHT_ASSIGN.0 + 1, "bit_and_assign");
const TOK_BIT_OR_ASSIGN: (u8, &'static str) = (TOK_BIT_AND_ASSIGN.0 + 1, "bit_or_assign");
const TOK_BIT_XOR_ASSIGN: (u8, &'static str) = (TOK_BIT_OR_ASSIGN.0 + 1, "bit_xor_assign");

const MIN_ASSIGN_TOK_ID: u8 = TOK_ASSIGN.0;
const MAX_ASSIGN_TOK_ID: u8 = TOK_BIT_XOR_ASSIGN.0;

// Keywords
const TOK_BREAK_KEYWORD: (u8, &'static str) = (TOK_BIT_XOR_ASSIGN.0 + 1, "break_keyword");
const TOK_CASE_KEYWORD: (u8, &'static str) = (TOK_BREAK_KEYWORD.0 + 1, "case_keyword");
const TOK_CATCH_KEYWORD: (u8, &'static str) = (TOK_CASE_KEYWORD.0 + 1, "catch_keyword");
const TOK_CONTINUE_KEYWORD: (u8, &'static str) = (TOK_CATCH_KEYWORD.0 + 1, "continue_keyword");
const TOK_DEFAULT_KEYWORD: (u8, &'static str) = (TOK_CONTINUE_KEYWORD.0 + 1, "default_keyword");
const TOK_DELETE_KEYWORD: (u8, &'static str) = (TOK_DEFAULT_KEYWORD.0 + 1, "delete_keyword");
const TOK_DO_KEYWORD: (u8, &'static str) = (TOK_DELETE_KEYWORD.0 + 1, "do_keyword");
const TOK_ELSE_KEYWORD: (u8, &'static str) = (TOK_DO_KEYWORD.0 + 1, "else_keyword");
const TOK_FINALLY_KEYWORD: (u8, &'static str) = (TOK_ELSE_KEYWORD.0 + 1, "finally_keyword");
const TOK_FOR_KEYWORD: (u8, &'static str) = (TOK_FINALLY_KEYWORD.0 + 1, "for_keyword");
const TOK_FUNCTION_KEYWORD: (u8, &'static str) = (TOK_FOR_KEYWORD.0 + 1, "function_keyword");
const TOK_IF_KEYWORD: (u8, &'static str) = (TOK_FUNCTION_KEYWORD.0 + 1, "if_keyword");
const TOK_IN_KEYWORD: (u8, &'static str) = (TOK_IF_KEYWORD.0 + 1, "in_keyword");
const TOK_INSTANCEOF_KEYWORD: (u8, &'static str) = (TOK_IN_KEYWORD.0 + 1, "instanceof_keyword");
const TOK_NEW_KEYWORD: (u8, &'static str) = (TOK_INSTANCEOF_KEYWORD.0 + 1, "new_keyword");
const TOK_RETURN_KEYWORD: (u8, &'static str) = (TOK_NEW_KEYWORD.0 + 1, "return_keyword");
const TOK_SWITCH_KEYWORD: (u8, &'static str) = (TOK_RETURN_KEYWORD.0 + 1, "switch_keyword");
const TOK_THIS_KEYWORD: (u8, &'static str) = (TOK_SWITCH_KEYWORD.0 + 1, "this_keyword");
const TOK_THROW_KEYWORD: (u8, &'static str) = (TOK_THIS_KEYWORD.0 + 1, "throw_keyword");
const TOK_TRY_KEYWORD: (u8, &'static str) = (TOK_THROW_KEYWORD.0 + 1, "try_keyword");
const TOK_TYPEOF_KEYWORD: (u8, &'static str) = (TOK_TRY_KEYWORD.0 + 1, "typeof_keyword");
const TOK_VAR_KEYWORD: (u8, &'static str) = (TOK_TYPEOF_KEYWORD.0 + 1, "var_keyword");
const TOK_VOID_KEYWORD: (u8, &'static str) = (TOK_VAR_KEYWORD.0 + 1, "void_keyword");
const TOK_WHILE_KEYWORD: (u8, &'static str) = (TOK_VOID_KEYWORD.0 + 1, "while_keyword");

const MIN_KEYWORD_TOK_ID: u8 = TOK_BREAK_KEYWORD.0;
const MAX_KEYWORD_TOK_ID: u8 = TOK_WHILE_KEYWORD.0;

const MAX_TOK_ID: u8 = TOK_WHILE_KEYWORD.0;

lazy_static! {
    static ref TOKEN_STRINGS: Vec<&'static str> = {
        let mut vec = Vec::with_capacity(MAX_TOK_ID as usize + 1);
        for i in 0..(MAX_TOK_ID+1) {
            vec.push("");
        }

        {
            let mut update_vec = |tok_info: &(u8, &'static str)| {
                vec[tok_info.0 as usize] = tok_info.1;
            };

            update_vec(&TOK_ERROR);
            update_vec(&TOK_END);

            update_vec(&TOK_WHITESPACE);
            update_vec(&TOK_NEWLINE);
            update_vec(&TOK_COMMENT);

            update_vec(&TOK_IDENTIFIER);
            update_vec(&TOK_INTEGER_LITERAL);
            update_vec(&TOK_HEX_INTEGER_LITERAL);
            update_vec(&TOK_OCT_INTEGER_LITERAL);
            update_vec(&TOK_FLOAT_LITERAL);

            update_vec(&TOK_OPEN_PAREN);
            update_vec(&TOK_CLOSE_PAREN);
            update_vec(&TOK_OPEN_BRACKET);
            update_vec(&TOK_CLOSE_BRACKET);
            update_vec(&TOK_OPEN_BRACE);
            update_vec(&TOK_CLOSE_BRACE);

            update_vec(&TOK_DOT);
            update_vec(&TOK_SEMICOLON);
            update_vec(&TOK_COMMA);
            update_vec(&TOK_QUESTION);
            update_vec(&TOK_COLON);

            update_vec(&TOK_EQUAL);
            update_vec(&TOK_STRICT_EQUAL);
            update_vec(&TOK_NOT_EQUAL);
            update_vec(&TOK_STRICT_NOT_EQUAL);
            update_vec(&TOK_LESS);
            update_vec(&TOK_LESS_EQUAL);
            update_vec(&TOK_GREATER);
            update_vec(&TOK_GREATER_EQUAL);

            update_vec(&TOK_TILDE);
            update_vec(&TOK_BANG);
            update_vec(&TOK_PLUS);
            update_vec(&TOK_PLUS_PLUS);
            update_vec(&TOK_MINUS);
            update_vec(&TOK_MINUS_MINUS);
            update_vec(&TOK_STAR);
            update_vec(&TOK_SLASH);
            update_vec(&TOK_PERCENT);
            update_vec(&TOK_SHIFT_LEFT);
            update_vec(&TOK_SHIFT_RIGHT);
            update_vec(&TOK_ARITHMETIC_SHIFT_RIGHT);
            update_vec(&TOK_BIT_AND);
            update_vec(&TOK_BIT_OR);
            update_vec(&TOK_BIT_XOR);
            update_vec(&TOK_LOGICAL_AND);
            update_vec(&TOK_LOGICAL_OR);

            update_vec(&TOK_ASSIGN);
            update_vec(&TOK_PLUS_ASSIGN);
            update_vec(&TOK_MINUS_ASSIGN);
            update_vec(&TOK_STAR_ASSIGN);
            update_vec(&TOK_SLASH_ASSIGN);
            update_vec(&TOK_PERCENT_ASSIGN);
            update_vec(&TOK_SHIFT_LEFT_ASSIGN);
            update_vec(&TOK_SHIFT_RIGHT_ASSIGN);
            update_vec(&TOK_ARITHMETIC_SHIFT_RIGHT_ASSIGN);
            update_vec(&TOK_BIT_AND_ASSIGN);
            update_vec(&TOK_BIT_OR_ASSIGN);
            update_vec(&TOK_BIT_XOR_ASSIGN);

            update_vec(&TOK_BREAK_KEYWORD);
            update_vec(&TOK_CASE_KEYWORD);
            update_vec(&TOK_CATCH_KEYWORD);
            update_vec(&TOK_CONTINUE_KEYWORD);
            update_vec(&TOK_DEFAULT_KEYWORD);
            update_vec(&TOK_DELETE_KEYWORD);
            update_vec(&TOK_DO_KEYWORD);
            update_vec(&TOK_ELSE_KEYWORD);
            update_vec(&TOK_FINALLY_KEYWORD);
            update_vec(&TOK_FOR_KEYWORD);
            update_vec(&TOK_FUNCTION_KEYWORD);
            update_vec(&TOK_IF_KEYWORD);
            update_vec(&TOK_IN_KEYWORD);
            update_vec(&TOK_INSTANCEOF_KEYWORD);
            update_vec(&TOK_NEW_KEYWORD);
            update_vec(&TOK_RETURN_KEYWORD);
            update_vec(&TOK_SWITCH_KEYWORD);
            update_vec(&TOK_THIS_KEYWORD);
            update_vec(&TOK_THROW_KEYWORD);
            update_vec(&TOK_TRY_KEYWORD);
            update_vec(&TOK_TYPEOF_KEYWORD);
            update_vec(&TOK_VAR_KEYWORD);
            update_vec(&TOK_VOID_KEYWORD);
            update_vec(&TOK_WHILE_KEYWORD);
        }
        vec
    };
}
