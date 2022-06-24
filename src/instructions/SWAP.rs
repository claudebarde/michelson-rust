use crate::instructions::{Instruction, RunOptions};
use crate::stack::{Stack, StackFuncs, StackSnapshots};

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::SWAP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    let mut new_stack: Stack = stack.clone();
    new_stack.swap(options.pos, options.pos + 1);
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}
