use serde_json::{Value};
use crate::stack::{ Stack, remove_at };
use crate::instructions::{ RunOptions };
use crate::errors::{ ErrorCode, error_code };
use crate::m_types::{ MValue, Or };

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least one element
    if stack.len() < 1 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((1, stack.len()))))
    }
    
    Ok(())
}

/// runs the instruction with the provided stack and options
pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok (_) => (),
        Err (err) => panic!("{}", err)
    };
    // checks the arguments
    let args = match args {
        None => panic!("No argument provided for IF_LEFT instruction"),
        Some (args_) => 
            if args_.len() != 2 {
                panic!("{}", Err(error_code(ErrorCode::UnexpectedArgsNumber((2, args_.len())))))
            } else {
                args_
            }
    };
    // unwraps the value
    let (or_element, stack) = remove_at(stack, options.pos);
    // processes the stack element value
    match or_element.value {
        MValue::Or (box_) => {
            match *box_ {
                Or::Left (left_val)     => {
                    let left_args = args[0];
                },
                Or::Right (right_val)   => {
                    let right_args = args[1];
                }
            };

            Ok(vec!())
        },
        _ => Err(error_code(ErrorCode::WrongType((String::from("or"), MValue::to_string(&stack[options.pos].value)))))
    }
}