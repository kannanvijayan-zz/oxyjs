
/** The enum of all token kinds. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenKind(u8);
impl TokenKind {
    pub fn error() -> TokenKind {
         TokenKind(KIND_ERROR)
    }
    pub fn is_error(&self) -> bool {
        self.0 == KIND_ERROR
    }

    pub fn end() -> TokenKind {
        TokenKind(KIND_END)
    }
    pub fn is_end(&self) -> bool {
        self.0 == KIND_END
    }


    pub fn whitespace() -> TokenKind {
        TokenKind(KIND_WHITESPACE)
    }
    pub fn is_whitespace(&self) -> bool {
        self.0 == KIND_WHITESPACE
    }

    pub fn newline() -> TokenKind {
        TokenKind(KIND_NEWLINE)
    }
    pub fn is_newline(&self) -> bool {
        self.0 == KIND_NEWLINE
    }

    pub fn comment() -> TokenKind {
        TokenKind(KIND_COMMENT)
    }
    pub fn is_comment(&self) -> bool {
        self.0 == KIND_COMMENT
    }


    pub fn identifier() -> TokenKind {
        TokenKind(KIND_IDENTIFIER)
    }
    pub fn is_identifier(&self) -> bool {
        self.0 == KIND_IDENTIFIER
    }

    pub fn integer_literal() -> TokenKind {
        TokenKind(KIND_INTEGER_LITERAL)
    }
    pub fn is_integer_literal(&self) -> bool {
        self.0 == KIND_INTEGER_LITERAL
    }

    pub fn hex_integer_literal() -> TokenKind {
        TokenKind(KIND_HEX_INTEGER_LITERAL)
    }
    pub fn is_hex_integer_literal(&self) -> bool {
        self.0 == KIND_HEX_INTEGER_LITERAL
    }

    pub fn oct_integer_literal() -> TokenKind {
        TokenKind(KIND_OCT_INTEGER_LITERAL)
    }
    pub fn is_oct_integer_literal(&self) -> bool {
        self.0 == KIND_OCT_INTEGER_LITERAL
    }

    pub fn float_literal() -> TokenKind {
        TokenKind(KIND_FLOAT_LITERAL)
    }
    pub fn is_float_literal(&self) -> bool {
        self.0 == KIND_FLOAT_LITERAL
    }


    pub fn open_paren() -> TokenKind {
        TokenKind(KIND_OPEN_PAREN)
    }
    pub fn is_open_paren(&self) -> bool {
        self.0 == KIND_OPEN_PAREN
    }

    pub fn close_paren() -> TokenKind {
        TokenKind(KIND_CLOSE_PAREN)
    }
    pub fn is_close_paren(&self) -> bool {
        self.0 == KIND_CLOSE_PAREN
    }

    pub fn open_bracket() -> TokenKind {
        TokenKind(KIND_OPEN_BRACKET)
    }
    pub fn is_open_bracket(&self) -> bool {
        self.0 == KIND_OPEN_BRACKET
    }

    pub fn close_bracket() -> TokenKind {
        TokenKind(KIND_CLOSE_BRACKET)
    }
    pub fn is_close_bracket(&self) -> bool {
        self.0 == KIND_CLOSE_BRACKET
    }

    pub fn open_brace() -> TokenKind {
        TokenKind(KIND_OPEN_BRACE)
    }
    pub fn is_open_brace(&self) -> bool {
        self.0 == KIND_OPEN_BRACE
    }

    pub fn close_brace() -> TokenKind {
        TokenKind(KIND_CLOSE_BRACE)
    }
    pub fn is_close_brace(&self) -> bool {
        self.0 == KIND_CLOSE_BRACE
    }


    pub fn dot() -> TokenKind {
        TokenKind(KIND_DOT)
    }
    pub fn is_dot(&self) -> bool {
        self.0 == KIND_DOT
    }

    pub fn semicolon() -> TokenKind {
        TokenKind(KIND_SEMICOLON)
    }
    pub fn is_semicolon(&self) -> bool {
        self.0 == KIND_SEMICOLON
    }

    pub fn comma() -> TokenKind {
        TokenKind(KIND_COMMA)
    }
    pub fn is_comma(&self) -> bool {
        self.0 == KIND_COMMA
    }

    pub fn question() -> TokenKind {
        TokenKind(KIND_QUESTION)
    }
    pub fn is_question(&self) -> bool {
        self.0 == KIND_QUESTION
    }

    pub fn colon() -> TokenKind {
        TokenKind(KIND_COLON)
    }
    pub fn is_colon(&self) -> bool {
        self.0 == KIND_COLON
    }


    pub fn equal() -> TokenKind {
        TokenKind(KIND_EQUAL)
    }
    pub fn is_equal(&self) -> bool {
        self.0 == KIND_EQUAL
    }

    pub fn strict_equal() -> TokenKind {
        TokenKind(KIND_STRICT_EQUAL)
    }
    pub fn is_strict_equal(&self) -> bool {
        self.0 == KIND_STRICT_EQUAL
    }

    pub fn not_equal() -> TokenKind {
        TokenKind(KIND_NOT_EQUAL)
    }
    pub fn is_not_equal(&self) -> bool {
        self.0 == KIND_NOT_EQUAL
    }

    pub fn strict_not_equal() -> TokenKind {
        TokenKind(KIND_STRICT_NOT_EQUAL)
    }
    pub fn is_strict_not_equal(&self) -> bool {
        self.0 == KIND_STRICT_NOT_EQUAL
    }

    pub fn less() -> TokenKind {
        TokenKind(KIND_LESS)
    }
    pub fn is_less(&self) -> bool {
        self.0 == KIND_LESS
    }

    pub fn less_equal() -> TokenKind {
        TokenKind(KIND_LESS_EQUAL)
    }
    pub fn is_less_equal(&self) -> bool {
        self.0 == KIND_LESS_EQUAL
    }

    pub fn greater() -> TokenKind {
        TokenKind(KIND_GREATER)
    }
    pub fn is_greater(&self) -> bool {
        self.0 == KIND_GREATER
    }

    pub fn greater_equal() -> TokenKind {
        TokenKind(KIND_GREATER_EQUAL)
    }
    pub fn is_greater_equal(&self) -> bool {
        self.0 == KIND_GREATER_EQUAL
    }


    pub fn tilde() -> TokenKind {
        TokenKind(KIND_TILDE)
    }
    pub fn is_tilde(&self) -> bool {
        self.0 == KIND_TILDE
    }

    pub fn bang() -> TokenKind {
        TokenKind(KIND_BANG)
    }
    pub fn is_bang(&self) -> bool {
        self.0 == KIND_BANG
    }

    pub fn plus() -> TokenKind {
        TokenKind(KIND_PLUS)
    }
    pub fn is_plus(&self) -> bool {
        self.0 == KIND_PLUS
    }

    pub fn plus_plus() -> TokenKind {
        TokenKind(KIND_PLUS_PLUS)
    }
    pub fn is_plus_plus(&self) -> bool {
        self.0 == KIND_PLUS_PLUS
    }

    pub fn minus() -> TokenKind {
        TokenKind(KIND_MINUS)
    }
    pub fn is_minus(&self) -> bool {
        self.0 == KIND_MINUS
    }

    pub fn minus_minus() -> TokenKind {
        TokenKind(KIND_MINUS_MINUS)
    }
    pub fn is_minus_minus(&self) -> bool {
        self.0 == KIND_MINUS_MINUS
    }

    pub fn star() -> TokenKind {
        TokenKind(KIND_STAR)
    }
    pub fn is_star(&self) -> bool {
        self.0 == KIND_STAR
    }

    pub fn slash() -> TokenKind {
        TokenKind(KIND_SLASH)
    }
    pub fn is_slash(&self) -> bool {
        self.0 == KIND_SLASH
    }

    pub fn percent() -> TokenKind {
        TokenKind(KIND_PERCENT)
    }
    pub fn is_percent(&self) -> bool {
        self.0 == KIND_PERCENT
    }

    pub fn shift_left() -> TokenKind {
        TokenKind(KIND_SHIFT_LEFT)
    }
    pub fn is_shift_left(&self) -> bool {
        self.0 == KIND_SHIFT_LEFT
    }

    pub fn shift_right() -> TokenKind {
        TokenKind(KIND_SHIFT_RIGHT)
    }
    pub fn is_shift_right(&self) -> bool {
        self.0 == KIND_SHIFT_RIGHT
    }

    pub fn arithmetic_shift_right() -> TokenKind {
        TokenKind(KIND_ARITHMETIC_SHIFT_RIGHT)
    }
    pub fn is_arithmetic_shift_right(&self) -> bool {
        self.0 == KIND_ARITHMETIC_SHIFT_RIGHT
    }

    pub fn bit_and() -> TokenKind {
        TokenKind(KIND_BIT_AND)
    }
    pub fn is_bit_and(&self) -> bool {
        self.0 == KIND_BIT_AND
    }

    pub fn bit_or() -> TokenKind {
        TokenKind(KIND_BIT_OR)
    }
    pub fn is_bit_or(&self) -> bool {
        self.0 == KIND_BIT_OR
    }

    pub fn bit_xor() -> TokenKind {
        TokenKind(KIND_BIT_XOR)
    }
    pub fn is_bit_xor(&self) -> bool {
        self.0 == KIND_BIT_XOR
    }

    pub fn logical_and() -> TokenKind {
        TokenKind(KIND_LOGICAL_AND)
    }
    pub fn is_logical_and(&self) -> bool {
        self.0 == KIND_LOGICAL_AND
    }

    pub fn logical_or() -> TokenKind {
        TokenKind(KIND_LOGICAL_OR)
    }
    pub fn is_logical_or(&self) -> bool {
        self.0 == KIND_LOGICAL_OR
    }


    pub fn assign() -> TokenKind {
        TokenKind(KIND_ASSIGN)
    }
    pub fn is_assign(&self) -> bool {
        self.0 == KIND_ASSIGN
    }

    pub fn plus_assign() -> TokenKind {
        TokenKind(KIND_PLUS_ASSIGN)
    }
    pub fn is_plus_assign(&self) -> bool {
        self.0 == KIND_PLUS_ASSIGN
    }

    pub fn minus_assign() -> TokenKind {
        TokenKind(KIND_MINUS_ASSIGN)
    }
    pub fn is_minus_assign(&self) -> bool {
        self.0 == KIND_MINUS_ASSIGN
    }

    pub fn star_assign() -> TokenKind {
        TokenKind(KIND_STAR_ASSIGN)
    }
    pub fn is_star_assign(&self) -> bool {
        self.0 == KIND_STAR_ASSIGN
    }

    pub fn slash_assign() -> TokenKind {
        TokenKind(KIND_SLASH_ASSIGN)
    }
    pub fn is_slash_assign(&self) -> bool {
        self.0 == KIND_SLASH_ASSIGN
    }

    pub fn percent_assign() -> TokenKind {
        TokenKind(KIND_PERCENT_ASSIGN)
    }
    pub fn is_percent_assign(&self) -> bool {
        self.0 == KIND_PERCENT_ASSIGN
    }

    pub fn shift_left_assign() -> TokenKind {
        TokenKind(KIND_SHIFT_LEFT_ASSIGN)
    }
    pub fn is_shift_left_assign(&self) -> bool {
        self.0 == KIND_SHIFT_LEFT_ASSIGN
    }

    pub fn shift_right_assign() -> TokenKind {
        TokenKind(KIND_SHIFT_RIGHT_ASSIGN)
    }
    pub fn is_shift_right_assign(&self) -> bool {
        self.0 == KIND_SHIFT_RIGHT_ASSIGN
    }

    pub fn arithmetic_shift_right_assign() -> TokenKind {
        TokenKind(KIND_ARITHMETIC_SHIFT_RIGHT_ASSIGN)
    }
    pub fn is_arithmetic_shift_right_assign(&self) -> bool {
        self.0 == KIND_ARITHMETIC_SHIFT_RIGHT_ASSIGN
    }

    pub fn bit_and_assign() -> TokenKind {
        TokenKind(KIND_BIT_AND_ASSIGN)
    }
    pub fn is_bit_and_assign(&self) -> bool {
        self.0 == KIND_BIT_AND_ASSIGN
    }

    pub fn bit_or_assign() -> TokenKind {
        TokenKind(KIND_BIT_OR_ASSIGN)
    }
    pub fn is_bit_or_assign(&self) -> bool {
        self.0 == KIND_BIT_OR_ASSIGN
    }

    pub fn bit_xor_assign() -> TokenKind {
        TokenKind(KIND_BIT_XOR_ASSIGN)
    }
    pub fn is_bit_xor_assign(&self) -> bool {
        self.0 == KIND_BIT_XOR_ASSIGN
    }


    pub fn break_keyword() -> TokenKind {
        TokenKind(KIND_BREAK_KEYWORD)
    }
    pub fn is_break_keyword(&self) -> bool {
        self.0 == KIND_BREAK_KEYWORD
    }

    pub fn case_keyword() -> TokenKind {
        TokenKind(KIND_CASE_KEYWORD)
    }
    pub fn is_case_keyword(&self) -> bool {
        self.0 == KIND_CASE_KEYWORD
    }

    pub fn catch_keyword() -> TokenKind {
        TokenKind(KIND_CATCH_KEYWORD)
    }
    pub fn is_catch_keyword(&self) -> bool {
        self.0 == KIND_CATCH_KEYWORD
    }

    pub fn continue_keyword() -> TokenKind {
        TokenKind(KIND_CONTINUE_KEYWORD)
    }
    pub fn is_continue_keyword(&self) -> bool {
        self.0 == KIND_CONTINUE_KEYWORD
    }

    pub fn default_keyword() -> TokenKind {
        TokenKind(KIND_DEFAULT_KEYWORD)
    }
    pub fn is_default_keyword(&self) -> bool {
        self.0 == KIND_DEFAULT_KEYWORD
    }

    pub fn delete_keyword() -> TokenKind {
        TokenKind(KIND_DELETE_KEYWORD)
    }
    pub fn is_delete_keyword(&self) -> bool {
        self.0 == KIND_DELETE_KEYWORD
    }

    pub fn do_keyword() -> TokenKind {
        TokenKind(KIND_DO_KEYWORD)
    }
    pub fn is_do_keyword(&self) -> bool {
        self.0 == KIND_DO_KEYWORD
    }

    pub fn else_keyword() -> TokenKind {
        TokenKind(KIND_ELSE_KEYWORD)
    }
    pub fn is_else_keyword(&self) -> bool {
        self.0 == KIND_ELSE_KEYWORD
    }

    pub fn finally_keyword() -> TokenKind {
        TokenKind(KIND_FINALLY_KEYWORD)
    }
    pub fn is_finally_keyword(&self) -> bool {
        self.0 == KIND_FINALLY_KEYWORD
    }

    pub fn for_keyword() -> TokenKind {
        TokenKind(KIND_FOR_KEYWORD)
    }
    pub fn is_for_keyword(&self) -> bool {
        self.0 == KIND_FOR_KEYWORD
    }

    pub fn function_keyword() -> TokenKind {
        TokenKind(KIND_FUNCTION_KEYWORD)
    }
    pub fn is_function_keyword(&self) -> bool {
        self.0 == KIND_FUNCTION_KEYWORD
    }

    pub fn if_keyword() -> TokenKind {
        TokenKind(KIND_IF_KEYWORD)
    }
    pub fn is_if_keyword(&self) -> bool {
        self.0 == KIND_IF_KEYWORD
    }

    pub fn in_keyword() -> TokenKind {
        TokenKind(KIND_IN_KEYWORD)
    }
    pub fn is_in_keyword(&self) -> bool {
        self.0 == KIND_IN_KEYWORD
    }

    pub fn instanceof_keyword() -> TokenKind {
        TokenKind(KIND_INSTANCEOF_KEYWORD)
    }
    pub fn is_instanceof_keyword(&self) -> bool {
        self.0 == KIND_INSTANCEOF_KEYWORD
    }

    pub fn new_keyword() -> TokenKind {
        TokenKind(KIND_NEW_KEYWORD)
    }
    pub fn is_new_keyword(&self) -> bool {
        self.0 == KIND_NEW_KEYWORD
    }

    pub fn return_keyword() -> TokenKind {
        TokenKind(KIND_RETURN_KEYWORD)
    }
    pub fn is_return_keyword(&self) -> bool {
        self.0 == KIND_RETURN_KEYWORD
    }

    pub fn switch_keyword() -> TokenKind {
        TokenKind(KIND_SWITCH_KEYWORD)
    }
    pub fn is_switch_keyword(&self) -> bool {
        self.0 == KIND_SWITCH_KEYWORD
    }

    pub fn this_keyword() -> TokenKind {
        TokenKind(KIND_THIS_KEYWORD)
    }
    pub fn is_this_keyword(&self) -> bool {
        self.0 == KIND_THIS_KEYWORD
    }

    pub fn throw_keyword() -> TokenKind {
        TokenKind(KIND_THROW_KEYWORD)
    }
    pub fn is_throw_keyword(&self) -> bool {
        self.0 == KIND_THROW_KEYWORD
    }

    pub fn try_keyword() -> TokenKind {
        TokenKind(KIND_TRY_KEYWORD)
    }
    pub fn is_try_keyword(&self) -> bool {
        self.0 == KIND_TRY_KEYWORD
    }

    pub fn typeof_keyword() -> TokenKind {
        TokenKind(KIND_TYPEOF_KEYWORD)
    }
    pub fn is_typeof_keyword(&self) -> bool {
        self.0 == KIND_TYPEOF_KEYWORD
    }

    pub fn var_keyword() -> TokenKind {
        TokenKind(KIND_VAR_KEYWORD)
    }
    pub fn is_var_keyword(&self) -> bool {
        self.0 == KIND_VAR_KEYWORD
    }

    pub fn void_keyword() -> TokenKind {
        TokenKind(KIND_VOID_KEYWORD)
    }
    pub fn is_void_keyword(&self) -> bool {
        self.0 == KIND_VOID_KEYWORD
    }

    pub fn while_keyword() -> TokenKind {
        TokenKind(KIND_WHILE_KEYWORD)
    }
    pub fn is_while_keyword(&self) -> bool {
        self.0 == KIND_WHILE_KEYWORD
    }


    pub fn is_keyword(&self) -> bool {
        (self.0 >= MIN_KEYWORD_KIND) && (self.0 <= MAX_KEYWORD_KIND)
    }
}

const KIND_ERROR: u8 = 0;
const KIND_END: u8 = 0;

const KIND_WHITESPACE: u8 = 0;
const KIND_NEWLINE: u8 = 0;
const KIND_COMMENT: u8 = 0;

const KIND_IDENTIFIER: u8 = 0;
const KIND_INTEGER_LITERAL: u8 = 0;
const KIND_HEX_INTEGER_LITERAL: u8 = 0;
const KIND_OCT_INTEGER_LITERAL: u8 = 0;
const KIND_FLOAT_LITERAL: u8 = 0;

// Braces.
const KIND_OPEN_PAREN: u8 = 0;
const KIND_CLOSE_PAREN: u8 = 0;
const KIND_OPEN_BRACKET: u8 = 0;
const KIND_CLOSE_BRACKET: u8 = 0;
const KIND_OPEN_BRACE: u8 = 0;
const KIND_CLOSE_BRACE: u8 = 0;

// Punctuation.
const KIND_DOT: u8 = 0;
const KIND_SEMICOLON: u8 = 0;
const KIND_COMMA: u8 = 0;
const KIND_QUESTION: u8 = 0;
const KIND_COLON: u8 = 0;

// Comparison
const KIND_EQUAL: u8 = 0;
const KIND_STRICT_EQUAL: u8 = 0;
const KIND_NOT_EQUAL: u8 = 0;
const KIND_STRICT_NOT_EQUAL: u8 = 0;
const KIND_LESS: u8 = 0;
const KIND_LESS_EQUAL: u8 = 0;
const KIND_GREATER: u8 = 0;
const KIND_GREATER_EQUAL: u8 = 0;

// Operator
const KIND_TILDE: u8 = 0;
const KIND_BANG: u8 = 0;
const KIND_PLUS: u8 = 0;
const KIND_PLUS_PLUS: u8 = 0;
const KIND_MINUS: u8 = 0;
const KIND_MINUS_MINUS: u8 = 0;
const KIND_STAR: u8 = 0;
const KIND_SLASH: u8 = 0;
const KIND_PERCENT: u8 = 0;
const KIND_SHIFT_LEFT: u8 = 0;
const KIND_SHIFT_RIGHT: u8 = 0;
const KIND_ARITHMETIC_SHIFT_RIGHT: u8 = 0;
const KIND_BIT_AND: u8 = 0;
const KIND_BIT_OR: u8 = 0;
const KIND_BIT_XOR: u8 = 0;
const KIND_LOGICAL_AND: u8 = 0;
const KIND_LOGICAL_OR: u8 = 0;

// Assignment
const KIND_ASSIGN: u8 = 0;
const KIND_PLUS_ASSIGN: u8 = 0;
const KIND_MINUS_ASSIGN: u8 = 0;
const KIND_STAR_ASSIGN: u8 = 0;
const KIND_SLASH_ASSIGN: u8 = 0;
const KIND_PERCENT_ASSIGN: u8 = 0;
const KIND_SHIFT_LEFT_ASSIGN: u8 = 0;
const KIND_SHIFT_RIGHT_ASSIGN: u8 = 0;
const KIND_ARITHMETIC_SHIFT_RIGHT_ASSIGN: u8 = 0;
const KIND_BIT_AND_ASSIGN: u8 = 0;
const KIND_BIT_OR_ASSIGN: u8 = 0;
const KIND_BIT_XOR_ASSIGN: u8 = 0;

    // Keywords
const KIND_BREAK_KEYWORD: u8 = 0;
const KIND_CASE_KEYWORD: u8 = 0;
const KIND_CATCH_KEYWORD: u8 = 0;
const KIND_CONTINUE_KEYWORD: u8 = 0;
const KIND_DEFAULT_KEYWORD: u8 = 0;
const KIND_DELETE_KEYWORD: u8 = 0;
const KIND_DO_KEYWORD: u8 = 0;
const KIND_ELSE_KEYWORD: u8 = 0;
const KIND_FINALLY_KEYWORD: u8 = 0;
const KIND_FOR_KEYWORD: u8 = 0;
const KIND_FUNCTION_KEYWORD: u8 = 0;
const KIND_IF_KEYWORD: u8 = 0;
const KIND_IN_KEYWORD: u8 = 0;
const KIND_INSTANCEOF_KEYWORD: u8 = 0;
const KIND_NEW_KEYWORD: u8 = 0;
const KIND_RETURN_KEYWORD: u8 = 0;
const KIND_SWITCH_KEYWORD: u8 = 0;
const KIND_THIS_KEYWORD: u8 = 0;
const KIND_THROW_KEYWORD: u8 = 0;
const KIND_TRY_KEYWORD: u8 = 0;
const KIND_TYPEOF_KEYWORD: u8 = 0;
const KIND_VAR_KEYWORD: u8 = 0;
const KIND_VOID_KEYWORD: u8 = 0;
const KIND_WHILE_KEYWORD: u8 = 0;

const MIN_KEYWORD_KIND: u8 = KIND_BREAK_KEYWORD;
const MAX_KEYWORD_KIND: u8 = KIND_WHILE_KEYWORD;
