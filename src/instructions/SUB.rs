use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, timestamp, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(2, Instruction::SUB) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };

    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(left), MValue::Int(right)) => MValue::Int(left - right),
        (MValue::Int(left), MValue::Nat(right)) => MValue::Int(left - (right as int)),
        (MValue::Nat(left), MValue::Int(right)) => MValue::Int((left as int) - right),
        (MValue::Nat(left), MValue::Nat(right)) => MValue::Int((left as int) - (right as int)),
        (MValue::Timestamp(left), MValue::Int(right)) => {
            MValue::Timestamp(left - (right as timestamp))
        }
        (MValue::Timestamp(left), MValue::Timestamp(right)) => MValue::Int((left - right) as int),
        (m_val_left, m_val_right) => panic!(
            "Cannot subtract values of type {} and {} with the SUB instruction",
            m_val_left.to_string(),
            m_val_right.to_string()
        ),
    };
    // updates the stack by removing the 2 elements
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // pushes the new value to the top of the stack
    let mut stack_head = vec![StackElement::new(new_val, Instruction::SUB)];
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // updates the stack snapshots
    stack_snapshots.push(stack_head.clone());
    // returns the stack
    Ok((stack_head, stack_snapshots))
}
