use crate::instructions::Instruction;
use crate::m_types::{mutez, nat, MType};

#[derive(Debug)]
pub enum ErrorCode {
    InvalidMutez(mutez),
    InvalidNat(nat),
    InvalidArgument((String, String)),
    NoArgument(Instruction),
    Noop(String),
    StackNotDeepEnough((usize, usize, Instruction)),
    UnexpectedArgsNumber((usize, usize)),
    InvalidStack((usize, MType, MType, Instruction)),
    InvalidType((Vec<MType>, MType, Instruction)),
    Unknown,
    WrongType((String, String, Instruction)),
}

pub fn display_error(error_code: ErrorCode) -> String {
    match error_code {
        ErrorCode::StackNotDeepEnough((expected, got, instruction)) => format!(
            "Unexpected stack length, expected a length of {} for instruction {:?}, got {}",
            expected, instruction, got
        ),
        ErrorCode::WrongType((expected, got, instruction)) => {
            format!(
                "Wrong type, expected `{}` for instruction {:?}, got `{}`",
                expected, instruction, got
            )
        }
        ErrorCode::Unknown => String::from("An unknown error has occured"),
        ErrorCode::UnexpectedArgsNumber((expected, got)) => format!(
            "Unexpected number of arguments, expected `{}`, got `{}`",
            expected, got
        ),
        ErrorCode::InvalidNat(val) => format!("Invalid nat value {}", val),
        ErrorCode::InvalidMutez(val) => format!("Invalid mutez value {}", val),
        ErrorCode::Noop(val) => format!("Noop performed: {}", val),
        ErrorCode::NoArgument(instruction) => {
            format!("No argument provided for the {:?} instruction", instruction)
        }
        ErrorCode::InvalidArgument((expected, got)) => format!(
            "Invalid argument provided, expected `{}`, but got `{}`",
            expected, got
        ),
        ErrorCode::InvalidType((expected, got, instruction)) => format!(
            "Invalid type for `{:?}` expected {}, but got {}",
            instruction,
            if expected.len() == 1 {
                expected[0].to_string()
            } else {
                expected
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            },
            got.to_string()
        ),
        ErrorCode::InvalidStack((pos, expected, got, instruction)) => format!(
            "Expected element at position {} to be of type {}, but got {} for instruction {:?}",
            pos,
            expected.to_string(),
            got.to_string(),
            instruction
        ),
    }
}
