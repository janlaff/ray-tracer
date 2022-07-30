#[derive(Debug)]
pub enum Error {
    CompileError(String),
    LinkError(String)
}