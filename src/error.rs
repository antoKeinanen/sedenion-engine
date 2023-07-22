use thiserror::Error;


#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Syntax error: no name found for function (this should not happen)")]
    NoFunctionName,
    #[error("Syntax error: invalid token '{0}'")]
    InvalidToken(String),
    #[error("Syntax error: invalid operator '{0}'")]
    InvalidOperator(String),
}

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Syntax error: can't find function with the name '{0}'")]
    UnknownFunction(String),
    #[error("Error while parsing: {0}")]
    ParseFailure(ParserError),
}



