use std::error::Error;

#[derive(Debug)]
pub enum RuntimeError {
    Timer(Box<Error>),
    Stdio(Box<Error>),
    Channel,
    Executor,
}
