use crate::instructions::{Instruction, RunOptions};
use crate::stack::{Stack, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DUG

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DUG) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to move
    match Instruction::DUG.check_num_arg(&args) {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_dig_pos) => {
            let el_pos = options.pos + el_to_dig_pos;
            // checks that the stack is deep enough for the DIG parameter
            match stack.check_depth(el_pos, Instruction::DUG) {
                Err(err) => Err(err),
                Ok(_) => Err(String::from("Work in progress")),
            }
        }
    }
}

/*
    TESTS
*/
