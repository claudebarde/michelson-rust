use crate::errors::{display_error, ErrorCode};
use crate::instructions::Instruction;
use crate::m_types::MValue;

#[derive(Debug, Clone)]
pub struct StackElement {
    pub value: MValue,
    pub instruction: Instruction, // keeps track of the instruction that pushed the value to the stack
}

impl StackElement {
    pub fn new(value: MValue, instruction: Instruction) -> StackElement {
        // TODO: create a function to validate the Michelson type
        StackElement { value, instruction }
    }

    pub fn get_val(&self) -> MValue {
        self.value.clone()
    }

    pub fn change_instruction(self, instruction: Instruction) -> StackElement {
        StackElement::new(self.value, instruction)
    }
}

pub type Stack = Vec<StackElement>;
pub type StackSnapshots = Vec<Stack>;

pub trait StackFuncs {
    /// Helper function to insert one or multiple stack elements
    /// at a given position in the stack
    /// It removes the element currently at the specified index
    fn replace(&self, els_to_insert: Vec<StackElement>, index: usize) -> Stack;
    /// Helper function to insert one or multiple elements
    /// at a given position in the stack
    /// It keeps all the elements in the stack and shifts their position
    fn insert_at(&self, els_to_insert: Vec<StackElement>, index: usize) -> Stack;
    /// Helper function to remove an element from the stack at the provided index
    /// Returns the element and the updated stack
    fn remove_at(&self, pos: usize) -> (StackElement, Stack);
    /// Helper function to push a new element on top of the stack
    /// Returns the new stack
    fn push(&self, el_to_push: MValue, instruction: Instruction) -> Stack;
    /// Helper function to check if the stack has the correct properties
    fn check_depth(&self, expected_size: usize, instruction: Instruction) -> Result<(), String>;
}

impl StackFuncs for Stack {
    /// Helper function to insert one or multiple stack elements
    /// at a given position in the stack
    fn replace(&self, els_to_insert: Vec<StackElement>, index: usize) -> Stack {
        let mut stack_start = self.clone();
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
        return stack_start;
    }
    /// Helper function to insert one or multiple elements
    /// at a given position in the stack
    /// It keeps all the elements in the stack and shifts their position
    fn insert_at(&self, els_to_insert: Vec<StackElement>, index: usize) -> Stack {
        let mut vec_head = self.clone();
        let vec_tail = vec_head.split_off(index);
        // concatenates the 3 vectors
        vec_head.extend(els_to_insert);
        vec_head.extend(vec_tail);
        return vec_head;
    }
    /// Helper function to remove an element from the stack at the provided index
    /// Returns the element and the updated stack
    fn remove_at(&self, pos: usize) -> (StackElement, Stack) {
        let mut new_stack = self.clone();
        let stack_el = new_stack.remove(pos);
        (stack_el, new_stack)
    }
    /// Helper function to push a new element on top of the stack
    /// Returns the new stack
    fn push(&self, val: MValue, instruction: Instruction) -> Stack {
        let stack_start = self.clone();
        // creates the new stack element
        let new_el = StackElement::new(val, instruction);
        let temp_stack = vec![new_el];

        // returns the new stack
        stack_start
            .into_iter()
            .chain(temp_stack.into_iter())
            .collect()
    }
    /// Helper function to check if the stack has the correct properties
    fn check_depth(&self, expected_size: usize, instruction: Instruction) -> Result<(), String> {
        if self.len() < expected_size {
            return Err(display_error(ErrorCode::StackNotDeepEnough((
                expected_size,
                self.len(),
                instruction,
            ))));
        }
        Ok(())
    }
}
