use crate::errors::{error_code, ErrorCode};
use crate::instructions::RunOptions;
use crate::stack::Stack;

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack) -> Result<(), String> {
    // stack must have at least 2 elements
    if stack.len() < 2 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((2, stack.len()))));
    }

    Ok(())
}

pub fn run(stack: Stack, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    let mut new_stack: Stack = stack.clone();
    new_stack.swap(options.pos, options.pos + 1);

    Ok(new_stack)
}
