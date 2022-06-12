use serde_json::{Value};
use crate::stack::{ Stack, create_stack_element, insert_at };
use crate::instructions::{ RunOptions, Instruction };
use crate::errors::{ ErrorCode, error_code };
use crate::m_types::{ MValue };
use crate::utils::{ pair };

/// checks if the stack has the correct properties
fn check_stack(stack: &Stack, pos: usize) -> Result<(), String> {
    // stack must have at least one element
    if stack.len() < 1 {
        return Err(error_code(ErrorCode::StackNotDeepEnough((1, stack.len()))))
    }
    // the element at pos index must be a pair
    match stack[pos].value {
        MValue::Pair (_) => Ok(()),
        _ => Err(error_code(ErrorCode::WrongType((String::from("pair"), MValue::to_string(&stack[0].value)))))
    }
}

/// runs the instruction with the provided stack and options
pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match check_stack(&stack, options.pos) {
        Ok (_) => (),
        Err (err) => panic!("{}", err)
    };
    // unpairs the value
    match pair::unpair(stack[options.pos].value.clone()) {
        Ok ((el1, el2)) => {
            // creates the new stack elements
            let stack_el1 = create_stack_element(el1, Instruction::UNPAIR);
            let stack_el2 = create_stack_element(el2, Instruction::UNPAIR);
            let els_to_insert = vec!(stack_el1, stack_el2);
            let new_stack = insert_at(stack.clone(), els_to_insert, options.pos);
            
            Ok(new_stack)
        },
        Err (err) => panic!("{}", err)
    }
}