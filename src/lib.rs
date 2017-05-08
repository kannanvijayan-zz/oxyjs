
pub mod parser;

/**
 * Initializer for library that must be called exactly once at the
 * beginning of program execution.
 */
pub fn initialize_library() {
    parser::initialize_module();
}
