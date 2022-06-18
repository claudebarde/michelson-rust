use crate::errors::{error_code, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, timestamp, MType, MValue};
use crate::stack::{create_stack_element, Stack, StackFuncs};

pub fn run(stack: Stack, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match stack.check_depth(2) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different numeric types
    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(left), MValue::Int(right)) => MValue::Int(left + right),
        (MValue::Int(left), MValue::Nat(right)) => {
            if MValue::Nat(right).check_nat() {
                MValue::Int(left + right as int)
            } else {
                panic!("{}", error_code(ErrorCode::InvalidNat(right)))
            }
        } // int
        (MValue::Nat(left), MValue::Int(right)) => {
            if MValue::Nat(left).check_nat() {
                MValue::Int(left as int + right)
            } else {
                panic!("{}", error_code(ErrorCode::InvalidNat(left)))
            }
        } // int
        (MValue::Nat(left), MValue::Nat(right)) => {
            if MValue::Nat(left).check_nat() == false {
                panic!("{}", error_code(ErrorCode::InvalidNat(left)))
            } else if MValue::Nat(right).check_nat() == false {
                panic!("{}", error_code(ErrorCode::InvalidNat(right)))
            } else {
                MValue::Nat(left + right)
            }
        } // nat
        (MValue::Timestamp(left), MValue::Int(right)) => {
            MValue::Timestamp((left as int + right) as timestamp)
        } // timestamp
        (MValue::Int(left), MValue::Timestamp(right)) => {
            MValue::Timestamp((left + right as int) as timestamp)
        } // timestamp
        (MValue::Mutez(left), MValue::Mutez(right)) => {
            if MValue::Mutez(left).check_mutez() == false {
                panic!("{}", error_code(ErrorCode::InvalidMutez(left)))
            } else if MValue::Mutez(right).check_mutez() == false {
                panic!("{}", error_code(ErrorCode::InvalidMutez(right)))
            } else {
                MValue::Mutez(left + right)
            }
        } // mutez
        (m_val_left, m_val_right) => panic!(
            "Cannot add together values of type {} and {}",
            m_val_left.to_string(),
            m_val_right.to_string()
        ),
    };
    // updates the stack by removing the 2 elements
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // pushes the new value to the top of the stack
    let mut stack_head = vec![create_stack_element(new_val, Instruction::ADD)];
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // returns the stack
    Ok(stack_head)
}
