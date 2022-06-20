use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use crate::utils::pair;

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::PAIR) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    // creates the new pair
    let new_pair: MValue = pair::pair(
        stack[options.pos].value.clone(),
        stack[options.pos + 1].value.clone(),
    );
    // drops the 2 elements from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // pushes the new pair to the stack
    let mut stack_with_pair: Stack = vec![StackElement::new(new_pair, Instruction::PAIR)];
    let mut old_stack = new_stack.clone();
    stack_with_pair.append(&mut old_stack);
    // updates the stack snapshots
    stack_snapshots.push(stack_with_pair.clone());

    Ok((stack_with_pair, stack_snapshots))
}
