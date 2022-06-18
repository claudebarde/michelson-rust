use crate::m_types::{mutez, nat};

#[derive(Debug)]
pub enum ErrorCode {
    StackNotDeepEnough((usize, usize)),
    WrongType((String, String)),
    UnexpectedArgsNumber((usize, usize)),
    InvalidNat(nat),
    InvalidMutez(mutez),
    Noop(String),
    Unknown,
}

pub fn error_code(error_code: ErrorCode) -> String {
    match error_code {
        ErrorCode::StackNotDeepEnough((expected, got)) => format!(
            "Unexpected stack length, expected a length of {}, got {}",
            expected, got
        ),
        ErrorCode::WrongType((expected, got)) => {
            format!("Wrong type, expected {}, got {}", expected, got)
        }
        ErrorCode::Unknown => String::from("An unknown error has occured"),
        ErrorCode::UnexpectedArgsNumber((expected, got)) => format!(
            "Unexpected number of arguments, expected {}, got {}",
            expected, got
        ),
        ErrorCode::InvalidNat(val) => format!("Invalid nat value {}", val),
        ErrorCode::InvalidMutez(val) => format!("Invalid mutez value {}", val),
        ErrorCode::Noop(val) => format!("Noop performed: {}", val),
    }
}
