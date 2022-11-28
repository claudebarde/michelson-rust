use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-MAP

/*
TEST CONTRACT

parameter (list nat) ;
storage (list nat) ;
code {
    CAR ;
    MAP {
        PUSH nat 2 ;
        MUL ;
    } ;
    NIL operation ;
    PAIR ;
} ;
*/

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::EDIV;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // checks that the value on the stack is correct
    let new_val: MValue = match stack[options.pos].get_val() {
        MValue::List(list) => {
            // checks that all the elements of the list are of the same type
            // loops through the list and applies instructions
            Ok(MValue::Int(69))
        },
        MValue::Map(map) => Ok(MValue::Int(69)),
        _ => Err(format!(
            "Invalid type on the stack at position {} for instruction `{:?}`, expected list or map, but got {:?}",
            options.pos,
            this_instruction,
            stack[options.pos].get_val().get_type()
        )),
    }?;

    Ok((stack, stack_snapshots))
}
