use crate::errors::{error_code, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{create_stack_element, remove_at, Stack};
use crate::utils::pair;

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least 2 elements
    if stack.len() < 2 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((2, stack.len()))));
    }

    Ok(())
}

pub fn run(stack: Stack, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    // creates the new pair
    let new_pair: MValue = pair::pair(
        stack[options.pos].value.clone(),
        stack[options.pos + 1].value.clone(),
    );
    // drops the 2 elements from the stack
    let (_, new_stack) = remove_at(stack, options.pos);
    let (_, new_stack) = remove_at(new_stack, options.pos);
    // pushes the new pair to the stack
    let mut stack_with_pair: Stack = vec![create_stack_element(new_pair, Instruction::PAIR)];
    let mut old_stack = new_stack.clone();
    stack_with_pair.append(&mut old_stack);

    Ok(stack_with_pair)
}
