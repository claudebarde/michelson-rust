use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, timestamp, MType, MValue};
use crate::stack::{create_stack_element, Stack, StackFuncs};

pub fn run(stack: Stack, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match stack.check_depth(2) {
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
    let mut stack_head = vec![create_stack_element(new_val, Instruction::SUB)];
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // returns the stack
    Ok(stack_head)
}
