
pub mod ast;
pub mod ast_builder;
pub mod char_utils;
pub mod input_stream;
pub mod precedence;
pub mod tokenizer;
pub mod token_kind;

/**
 * Initializer for parser module that must be called exactly once at the
 * beginning of program execution.
 */
pub fn initialize_module() {
    token_kind::initialize_module();
    tokenizer::initialize_module();
}

