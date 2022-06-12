use serde_json::{Value};
use crate::stack::{ Stack, remove_at };
use crate::instructions::{ RunOptions };
use crate::errors::{ ErrorCode, error_code };
use crate::m_types::{ MValue };

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least one element
    if stack.len() < 1 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((1, stack.len()))))
    }
    // element at pos index must be of type or
    match stack[pos].value {
        MValue::Or (_) => Ok(()),
        _ => Err(error_code(ErrorCode::WrongType((String::from("or"), MValue::to_string(&stack[pos].value)))))
    }
}

/// runs the instruction with the provided stack and options
pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok (_) => (),
        Err (err) => panic!("{}", err)
    };
    // unwraps the value
    let (or_element, stack) = remove_at(stack, options.pos);

    Ok(stack)
}