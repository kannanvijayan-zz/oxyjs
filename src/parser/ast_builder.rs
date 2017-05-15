
use std::borrow::Borrow;
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
        FullToken { kind, location }
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
    fn end_offset(&self) -> StreamPosition {
        self.location.end_offset()
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
    ExpectedCommaOrCloseParen,
    ExpectedExpression,
    ExpectedStatement
}
pub type ParseResult<T> = Result<T, ParseError>;
pub type MaybeParseResult<T> = ParseResult<Option<T>>;

pub struct AstBuilder<STREAM: InputStream> {
    tokenizer: Tokenizer<STREAM, FullTokenizerMode>,
    skipped_newline: bool
}
impl<STREAM: InputStream> AstBuilder<STREAM> {
    pub fn new(stream: STREAM) -> AstBuilder<STREAM> {
        AstBuilder {
            tokenizer: Tokenizer::new(stream, FullTokenizerMode{}),
            skipped_newline: false
        }
    }

    pub fn read_and_print_tokens(&mut self) {
        // Just read tokens and print them out until we're done, then return Error.
        loop {
            let token = self.next_token().unwrap();
            self.log_debug(format!("Token: {}", token.kind().name()));
            if token.kind().is_error() {
                panic!("Got token error: {:?}", self.tokenizer.get_error());
            }
            if token.kind().is_end() {
                break;
            }
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Box<ast::ProgramNode>> {
        self.log_debug(format!("parse_program() BEGIN"));
        let mut program_node = ast::ProgramNode::new();
        loop {
            self.log_debug(format!("parse_program() LOOP"));
            match self.check_parse_statement()? {
                Some(boxed_source_element) => {
                    self.log_debug(format!("parse_program() GOT SOURCE ELEMENT"));
                    program_node.add_source_element(boxed_source_element);
                }
                None => {
                    break;
                }
            }
        }

        // Must have reached end of stream.
        self.log_debug(format!("parse_program() CHECK FOR END"));
        self.must_expect_token(TokenKind::end())?;

        self.log_debug(format!("parse_program() END"));
        Ok(Box::new(program_node))
    }

    fn parse_statement(&mut self) -> ParseResult<Box<AstNode>> {
        self.log_debug(format!("parse_statement() BEGIN"));
        if let Some(boxed_stmt) = self.check_parse_statement()? {
            self.log_debug(format!("parse_statement() GOT STATEMENT"));
            Ok(boxed_stmt)
        } else {
            self.log_debug(format!("parse_statement() NO STATEMENT"));
            Err(ParseError::ExpectedStatement)
        }
    }

    fn check_parse_statement(&mut self) -> MaybeParseResult<Box<AstNode>> {
        self.log_debug(format!("check_parse_statement() BEGIN"));
        let begin_position = self.mark_position();
        let tok = self.next_token()?;
        if tok.kind().is_open_brace() {
            self.log_debug(format!("check_parse_statement() OPEN BRACE"));
            // FIXME: parse either a block or an object literal.
            // The spec says a '{' at the statement level can only be a block,
            // but practical implementations seem to allow bare object literals.
            return Ok(Some(self.parse_block_statement()?));
        }
        if tok.kind().is_var_keyword() {
            self.log_debug(format!("check_parse_statement() VAR"));
            return Ok(Some(self.parse_var_statement()?));
        }
        if tok.kind().is_semicolon() {
            self.log_debug(format!("check_parse_statement() SEMICOLON"));
            return Ok(Some(Box::new(ast::EmptyStmtNode::new())));
        }
        if tok.kind().is_if_keyword() {
            self.log_debug(format!("check_parse_statement() IF"));
            return Ok(Some(self.parse_if_statement()?));
        }

        self.log_debug(format!("check_parse_statement() CHECKING FOR EXPRESSION"));
        if let Some(boxed_expr) = self.try_parse_expression_with(tok, Precedence::lowest())? {
            self.log_debug(format!("check_parse_statement() GOT EXPRESSION"));
            return Ok(Some(Box::new(ast::ExprStmtNode::new(boxed_expr))));
        }

        self.log_debug(format!("check_parse_statement() END (FAILED)"));
        self.rewind_position(begin_position);
        Ok(None)
    }

    fn parse_block_statement(&mut self) -> ParseResult<Box<ast::BlockStmtNode>> {
        // FIXME: Parse list of statements.
        self.must_expect_token(TokenKind::close_brace())?;
        Ok(Box::new(ast::BlockStmtNode::new()))
    }

    fn parse_var_statement(&mut self) -> ParseResult<Box<ast::VarStmtNode>> {
        let mut var_statement = Box::new(ast::VarStmtNode::new());
        loop {
            // FIXME: Support initializer expressions.
            // For now, we match only a VarName ("," VarName)* ";"
            let name_token = match self.expect_get_token(TokenKind::identifier())? {
                Some(token) => token,
                None => { return Err(ParseError::ExpectedVariableName); }
            };

            let next_tok = self.next_token()?;
            if next_tok.kind().is_semicolon() {
                break;
            }
            if next_tok.kind().is_assign() {
                // Parse an initializer.
                let boxed_expr = self.parse_expression(Precedence::assignment())?;
                self.log_debug(format!("Got init expr: {}", boxed_expr.tree_string()));
                var_statement.add_var_decl_with_init(name_token, boxed_expr);

                let next_tok = self.next_token()?;
                if next_tok.kind().is_semicolon() {
                    break;
                }
                if next_tok.kind().is_comma() {
                    continue;
                }
                return Err(ParseError::ExpectedCommaOrSemicolon);
            }
            if next_tok.kind().is_comma() {
                var_statement.add_var_decl(name_token);
                continue;
            }
            return Err(ParseError::ExpectedCommaOrSemicolon);
        }
        Ok(var_statement)
    }

    fn parse_if_statement(&mut self) -> ParseResult<Box<AstNode>> {
        // "if" must be followed by "(".
        self.must_expect_token(TokenKind::open_paren())?;
        let cond_expr = self.parse_expression(Precedence::lowest())?;
        self.must_expect_token(TokenKind::close_paren())?;
        let if_true_stmt = self.parse_statement()?;

        // Check for 'else'
        if self.expect_token(TokenKind::else_keyword())? {
            let if_false_stmt = self.parse_statement()?;
            Ok(Box::new(ast::IfStmtNode::new_if_else(cond_expr, if_true_stmt, if_false_stmt)))
        } else {
            Ok(Box::new(ast::IfStmtNode::new_if(cond_expr, if_true_stmt)))
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Box<AstNode>> {
        if let Some(boxed_expr) = self.check_parse_expression(precedence)? {
            Ok(boxed_expr)
        } else {
            Err(ParseError::ExpectedExpression)
        }
    }
    fn check_parse_expression(&mut self, precedence: Precedence) -> MaybeParseResult<Box<AstNode>> {
        let position = self.mark_position();
        let tok = self.next_token()?;
        if let Some(expr) = self.try_parse_expression_with(tok, precedence)? {
            Ok(Some(expr))
        } else {
            self.rewind_position(position);
            Ok(None)
        }
    }

    fn try_parse_expression_with(&mut self, tok: FullToken, precedence: Precedence)
        -> MaybeParseResult<Box<AstNode>>
    {
        self.log_debug("try_parse_expression_with() BEGIN");
        if tok.kind().is_atomic_expr() {
            self.log_debug("try_parse_expression_with() HANDLE ATOMIC EXPR");
            let atomic_expr = Box::new(ast::AtomicExprNode::new(tok));
            return Ok(Some(self.parse_rest_of_expression(atomic_expr, precedence)?));
        }
        if tok.kind().is_unary_op() {
            self.log_debug("try_parse_expression_with() HANDLE UNARY OP");
            let sub_expr = self.parse_expression(Precedence::unary())?;
            let unary_expr = Box::new(ast::UnaryOpExprNode::new(tok, sub_expr));
            return Ok(Some(self.parse_rest_of_expression(unary_expr, precedence)?));
        }
        if tok.kind().is_new_keyword() {
            self.log_debug("try_parse_expression_with() HANDLE NEW");
            assert!(precedence <= Precedence::left_hand_side());
            let mut new_count: usize = 1;
            loop {
                let next_tok = self.next_token()?;
                if ! next_tok.kind().is_new_keyword() {
                    // Reached end of new keywords - this token must begin a member
                    // expression.
                    match self.try_parse_expression_with(next_tok, Precedence::member())? {
                        Some(member_expr) => {
                            return Ok(Some(self.parse_new_tail(new_count, member_expr, precedence)?));
                        }
                        None => {
                            return Err(ParseError::ExpectedExpression);
                        }
                    }
                }
                new_count += 1;
            }
        }
        Ok(None)
    }

    fn parse_new_tail(&mut self, new_count: usize, member_expr: Box<AstNode>,
                      precedence: Precedence)
        -> ParseResult<Box<AstNode>>
    {
        assert!(precedence <= Precedence::left_hand_side());
        let mut cur_expr: Box<AstNode> = member_expr;
        let mut cur_new_count: usize = new_count;
        loop {
            // Check for following "(", up to new_count.
            let position = self.mark_position();
            let next_tok = self.next_token()?;
            if ! next_tok.kind().is_open_paren() {
                self.rewind_position(position);
                break;
            }
            let mut args_vec = Vec::with_capacity(2);
            self.parse_arguments_list(&mut args_vec)?;
            self.must_expect_token(TokenKind::close_paren())?;
            cur_expr = Box::new(ast::ConstructExprNode::new_with_arguments(cur_expr, args_vec));
            cur_new_count += 1;
            if cur_new_count == new_count {
                break;
            }
        }

        if cur_new_count == new_count {
            // If all the 'new's matched with arguments, then we have a MemberExpr
            // that we just parsed.  Finish the parse with call expr precedence.
            cur_expr = self.parse_rest_of_expression(cur_expr, Precedence::call())?;
        } else {
            // Wrap up the remaining "bare" new expressions.
            while cur_new_count < new_count {
                cur_expr = Box::new(ast::ConstructExprNode::new_bare(cur_expr));
                cur_new_count += 1;
            }
        }

        // Parse the rest of the expression with the given precedence.
        self.parse_rest_of_expression(cur_expr, precedence)
    }

    fn parse_arguments_list(&mut self, args_vec: &mut Vec<Box<AstNode>>) -> ParseResult<()> {
        self.log_debug("parse_arguments_list() BEGIN");
        // Check for immediate ')' token.
        if self.expect_token(TokenKind::close_paren())? {
            self.log_debug("parse_arguments_list() GOT CLOSE PAREN");
            return Ok(());
        }

        // Otherwise, parse argument expressions.
        loop {
            args_vec.push(self.parse_expression(Precedence::assignment())?);
            let next_tok = self.next_token()?;
            if next_tok.kind().is_close_paren() {
                break;
            }
            if ! next_tok.kind().is_comma() {
                return Err(ParseError::ExpectedCommaOrCloseParen);
            }
        }
        Ok(())
    }

    fn parse_rest_of_expression(&mut self, left_expr: Box<AstNode>, precedence: Precedence)
        -> ParseResult<Box<AstNode>>
    {
        assert!(left_expr.is_expression());

        self.log_debug("parse_rest_of_expression() BEGIN");
        let mut cur_expr = left_expr;
        loop {
            self.log_debug("parse_rest_of_expression() LOOP");
            let position = self.mark_position();
            let tok = self.next_token()?;
            self.log_debug(format!("parse_rest_of_expression() GOT TOKEN {}", tok.kind().name()));
            if tok.kind().is_comma() {
                if precedence >= Precedence::comma() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::comma())?;
                cur_expr = Box::new(ast::CommaExprNode::new(cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_assignment_op() {
                // FIXME: Check that cur_expr is a valid lvalue expression.
                // Return syntax error if not.

                // Assignment associates right-to-left, so we use '>' for precedence
                // instead of '>='.
                if precedence > Precedence::assignment() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::assignment())?;
                cur_expr = Box::new(ast::AssignExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_question() {
                if precedence > Precedence::conditional() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let if_expr = self.parse_expression(Precedence::assignment())?;
                self.must_expect_token(TokenKind::colon())?;
                let else_expr = self.parse_expression(Precedence::assignment())?;
                cur_expr = Box::new(ast::CondExprNode::new(cur_expr, if_expr, else_expr));
                continue;
            }

            if tok.kind().is_logical_or() {
                if precedence >= Precedence::logical_or() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::logical_or())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_logical_and() {
                if precedence >= Precedence::logical_and() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::logical_and())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_bit_or() {
                if precedence >= Precedence::bitwise_or() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::bitwise_or())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_bit_xor() {
                if precedence >= Precedence::bitwise_xor() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::bitwise_xor())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_bit_and() {
                if precedence >= Precedence::bitwise_and() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::bitwise_and())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_equality_op() {
                if precedence >= Precedence::equality() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::equality())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_relational_op() {
                if precedence >= Precedence::relational() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::relational())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_shift_op() {
                if precedence >= Precedence::shift() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::shift())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_plus() || tok.kind().is_minus() {
                if precedence >= Precedence::additive() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::additive())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_star() || tok.kind().is_slash() || tok.kind().is_percent() {
                if precedence >= Precedence::multiplicative() {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                let right_expr = self.parse_expression(Precedence::multiplicative())?;
                cur_expr = Box::new(ast::BinaryOpExprNode::new(tok, cur_expr, right_expr));
                continue;
            }

            if tok.kind().is_plus_plus() || tok.kind().is_minus_minus() {
                // Precedence doesn't matter here.  The '++' applies to the left-hand-side
                // expression directly.  Only thing to look out for is skipped newlines.
                if self.skipped_newline {
                    self.rewind_position(position);
                    return Ok(cur_expr);
                }

                // FIXME: Check that cur_expr is a proper LVALUE expression.

                cur_expr = Box::new(ast::PostfixOpExprNode::new(tok, cur_expr));
                continue;
            }

            if tok.kind().is_dot() {
                // We should only ever see "dot" with precedence levels <= member.
                assert!(precedence <= Precedence::member());
                let name_tok = self.must_expect_get_token(TokenKind::identifier())?;
                cur_expr = Box::new(ast::PropertyExprNode::new(cur_expr, name_tok));
                continue;
            }

            if tok.kind().is_open_bracket() {
                // We should only ever see "[]" with precedence levels <= member.
                assert!(precedence <= Precedence::member());
                let rest_expr = self.parse_expression(Precedence::lowest())?;
                self.must_expect_token(TokenKind::close_bracket())?;
                cur_expr = Box::new(ast::ElementExprNode::new(cur_expr, rest_expr));
                continue;
            }

            if tok.kind().is_open_paren() {
                // We should only ever see "()" with precedence levels <= member.
                self.log_debug("parse_rest_of_expression() HANDLE OPEN PAREN");
                assert!(precedence <= Precedence::member());
                let mut args_vec = Vec::with_capacity(2);
                self.parse_arguments_list(&mut args_vec)?;
                cur_expr = Box::new(ast::CallExprNode::new(cur_expr, args_vec));
                continue;
            }

            // Unknown token terminates expression.
            self.log_debug("parse_rest_of_expression() REWIND AND RETURN");
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

    fn must_expect_get_token(&mut self, kind: TokenKind) -> ParseResult<FullToken> {
        // Mark the position so we can backtrack.
        let position = self.mark_position();
        let token = self.next_token()?;
        if token.kind() == kind {
            Ok(token)
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
        self.skipped_newline = false;
        loop {
            let token = self.tokenizer.next_token(/* check_kw = */ true);
            let kind = token.kind();
            // Ignore whitespace and comment and newline tokens.
            if kind.is_whitespace() || kind.is_comment() {
                continue;
            }
            if !want_newlines && kind.is_newline() {
                self.skipped_newline = true;
                continue;
            }
            if kind.is_error() {
                return Err(ParseError::ErrorToken(token));
            }
            let kw_str = if check_kw { "kw" } else { "no-kw" };
            let nl_str = if want_newlines { "nl" } else { "no-nl" };
            self.log_debug(format!("next_token({}, {}): {}", kw_str, nl_str, token.token_string()));
            return Ok(token);
        }
    }

    fn log_debug<'a, STR: Borrow<str>>(&self, str: STR) {
        println!("DEBUG: {}", str.borrow());
    }
}
