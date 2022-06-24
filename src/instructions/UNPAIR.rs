use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use crate::utils::pair;
use serde_json::Value;

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least one element
    if stack.len() < 1 {
        return Err(display_error(ErrorCode::StackNotDeepEnough((
            1,
            stack.len(),
            Instruction::UNPAIR,
        ))));
    }
    // the element at pos index must be a pair
    match stack[pos].value {
        MValue::Pair(_) => Ok(()),
        _ => Err(display_error(ErrorCode::WrongType((
            String::from("pair"),
            MValue::to_string(&stack[0].value),
        )))),
    }
}

/// runs the instruction with the provided stack and options
pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // unpairs the value
    match pair::unpair(stack[options.pos].value.clone()) {
        Ok((el1, el2)) => {
            // creates the new stack elements
            let stack_el1 = StackElement::new(el1, Instruction::UNPAIR);
            let stack_el2 = StackElement::new(el2, Instruction::UNPAIR);
            let els_to_insert = vec![stack_el1, stack_el2];
            let new_stack = stack.clone().insert_instead(els_to_insert, options.pos);
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());
            Ok((new_stack, stack_snapshots))
        }
        Err(err) => panic!("{}", err),
    }
}
