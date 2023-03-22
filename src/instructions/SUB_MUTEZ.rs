use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SUB_MUTEZ

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::SUB_MUTEZ;
    // checks the stack
    stack.check_depth(options.pos + 2, this_instruction)?;
    // elements on the stack must be two mutez values
    let new_stack_el = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Mutez(minuend), MValue::Mutez(subtrahend)) => {
            // gets the result of the subtraction
            let result = if minuend < subtrahend {
                None
            } else {
                Some(MValue::Mutez(minuend - subtrahend))
            };
            // creates an optional value
            let new_val = MValue::Option(OptionValue {
                m_type: MType::Mutez,
                value: Box::new(result),
            });
            // creates a new stack element
            Ok(StackElement::new(new_val, this_instruction))
        }
        (MValue::Mutez(_), val) | (val, MValue::Mutez(_)) => {
            Err(display_error(ErrorCode::WrongType((
                String::from("mutez"),
                val.get_type().to_string(),
                this_instruction,
            ))))
        }
        _ => Err(String::from(
            "SUB_MUTEZ instruction requires 2 mutez values on the stack",
        )),
    }?;

    let (_, new_stack) = stack.remove_at(options.pos);
    let new_stack = new_stack.insert_at(vec![new_stack_el], options.pos);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}
