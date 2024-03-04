pub use function::function;
use torchc_cgen::cgen;
mod function;

pub trait IllegalIndentAccordingTo<'according> {
    /// Adds to the error message a custom extension for the respective statement combinations
    /// in the illegal indentation.
    fn illegal_indent_according_to(
        &'according mut self,
        parent: &cgen::Stmt,
        child: &cgen::Stmt,
    ) -> &'according str;
}
impl<'according> IllegalIndentAccordingTo<'according> for String {
    fn illegal_indent_according_to(
        &'according mut self,
        parent: &cgen::Stmt,
        child: &cgen::Stmt,
    ) -> &'according str {
        match (parent, child) {
            // Illegal: indented function.
            (_, cgen::Stmt::Fn(_)) => {
                self.push_str(", the function statement must go in the root");
            }
            // Default.
            _ => {}
        }
        self
    }
}
