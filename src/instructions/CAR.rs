use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-CAR

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::CAR;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // checks if the stack has a pair on it
    let new_val: MValue = match stack[options.pos].get_val() {
        MValue::Pair(pair) => {
            // extracts the left field of the pair
            Ok(pair.car())
        }
        val => Err(display_error(ErrorCode::WrongType((
            String::from("pair"),
            val.get_type().to_string(),
            this_instruction,
        )))),
    }?;

    let new_el = StackElement::new(new_val, this_instruction);
    let new_stack = stack.replace(vec![new_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}
