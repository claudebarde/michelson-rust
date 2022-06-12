use crate::m_types::{ MValue };
use crate::instructions::{ Instruction };

#[derive(Debug, Clone)]
pub struct StackElement {
    pub value: MValue,
    pub instruction: Instruction
}

pub type Stack = Vec<StackElement>;

/// Helper function to create a new stack element
pub fn create_stack_element(value: MValue, instruction: Instruction) -> StackElement {
    StackElement {
        value,
        instruction
    }
}

/// Helper function to insert one or multiple stack elements
/// at a given position in the stack
pub fn insert_at(stack: Stack, els_to_insert: Vec<StackElement>, index: usize) -> Stack {
    let mut stack_start = stack.clone();
    let mut vec_tail = stack_start.split_off(index);
    // reverses the vector order to remove the first element
    vec_tail.reverse();
    // removes element at index
    vec_tail.pop();
    // puts the vector back in the right order
    vec_tail.reverse();
    // concatenates the 3 vectors
    stack_start.extend(els_to_insert);
    stack_start.extend(vec_tail);
    return stack_start
}