use crate::stack::{ Stack, remove_at, create_stack_element };
use crate::instructions::{ RunOptions, Instruction };
use crate::errors::{ ErrorCode, error_code };
use crate::m_types::{ MValue, int, timestamp };

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least 2 elements
    if stack.len() < 2 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((2, stack.len()))))
    }

    Ok(())
}

pub fn run(stack: Stack, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok (_) => (),
        Err (err) => panic!("{}", err)
    };
    // pattern matches the different numeric types
    let new_val: MValue =
        match (stack[options.pos].get_val(), stack[options.pos + 1].get_val()) {
            (MValue::Int(left), MValue::Int(right))         => MValue::Int(left + right), // int
            (MValue::Int(left), MValue::Nat(right))         => MValue::Int(left + right as int), // int
            (MValue::Nat(left), MValue::Int(right))         => MValue::Int(left as int + right), // int
            (MValue::Nat(left), MValue::Nat(right))         => MValue::Nat(left + right), // nat
            (MValue::Timestamp(left), MValue::Int(right))   => MValue::Timestamp((left as int + right) as timestamp), // timestamp
            (MValue::Int(left), MValue::Timestamp(right))   => MValue::Timestamp((left + right as int) as timestamp), // timestamp
            (MValue::Mutez(left), MValue::Mutez(right))     => MValue::Mutez(left + right), // mutez
            (m_val_left, m_val_right)                       => 
                panic!("Cannot add together values of type {} and {}", m_val_left.to_string(), m_val_right.to_string())
        };
    // updates the stack by removing the 2 elements
    let (_, new_stack) = remove_at(stack, options.pos);
    let (_, new_stack) = remove_at(new_stack, options.pos);
    // pushes the new value to the top of the stack
    let mut stack_head = vec!(create_stack_element(new_val, Instruction::ADD));
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // returns the stack
    Ok(stack_head)
}