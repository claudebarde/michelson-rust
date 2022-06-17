use crate::errors::{error_code, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, Or};
use crate::parser;
use crate::stack::{create_stack_element, remove_at, Stack};
use serde_json::Value;

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least one element
    if stack.len() < 1 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((1, stack.len()))));
    }
    Ok(())
}

/// runs the instruction with the provided stack and options
pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // checks the arguments
    let args = match args {
        None => panic!("No argument provided for IF_LEFT instruction"),
        Some(args_) => {
            if args_.len() != 2 {
                panic!(
                    "{:?}",
                    error_code(ErrorCode::UnexpectedArgsNumber((2, args_.len())))
                )
            } else {
                args_
            }
        }
    };
    // unwraps the value
    let (or_element, stack) = remove_at(stack, options.pos);
    // processes the stack element value
    match or_element.value {
        MValue::Or(box_) => {
            // gets the corresponding arguments and m_value
            let (new_args, m_val): (String, MValue) = match *box_.value {
                Or::Left(left_val) => (args[0].to_string(), left_val),
                Or::Right(right_val) => (args[1].to_string(), right_val),
            };
            // Pushes unwrapped value to the stack
            let mut stack_head = vec![create_stack_element(m_val, Instruction::IF_LEFT)];
            let mut stack_tail = stack.clone();
            stack_head.append(&mut stack_tail);
            // runs the code inside the argument
            parser::run(new_args.as_str(), stack_head)
        }
        _ => Err(error_code(ErrorCode::WrongType((
            String::from("or"),
            MValue::to_string(&stack[options.pos].value),
        )))),
    }
}
