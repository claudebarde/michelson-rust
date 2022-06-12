pub enum ErrorCode {
    StackNotDeepEnough ((usize, usize)),
    WrongType ((String, String)),
    Unknown
}

pub fn error_code(error_code: ErrorCode) -> String {
    match error_code {
        ErrorCode::StackNotDeepEnough ((expected, got)) => 
            format!("Unexpected stack length, expected a length of {}, got {}", expected, got),
        ErrorCode::WrongType ((expected, got)) => 
            format!("Wrong type, expected {}, got {}", expected, got),
        ErrorCode::Unknown => String::from("An unknown error has occured")
    }
}