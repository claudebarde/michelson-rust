use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, CollectionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-CONS

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 2, Instruction::CONS)?;
    // checks if second element is a list
    match (&stack[options.pos].value, &stack[options.pos + 1].value) {
        (first_el, MValue::List(list)) => {
            // checks if first element is the same type like the list elements
            let list_el_type = &list.m_type;
            let stack_el_type = &first_el.get_type();
            if stack_el_type == list_el_type {
                // pushes the element in the list
                let new_list = list.cons(first_el.clone());
                // creates a new list with the updated collection
                let new_val = MValue::List(new_list);
                // updates the stack
                let (_, new_stack) = stack.remove_at(options.pos);
                // pushes the new element to the stack
                let new_stack = new_stack.replace(
                    vec![StackElement::new(new_val, Instruction::CONS)],
                    options.pos,
                );
                // updates the stack snapshots
                stack_snapshots.push(new_stack.clone());
                // updates the stack snapshots
                Ok((new_stack, stack_snapshots))
            } else {
                // element to prepend is of the wrong type
                Err(String::from(
                    format!("Element to prepend to the list with CONS is of type {}, while the list elements are of type {}", 
                    stack_el_type.to_string(), 
                    list_el_type.to_string())
                ))
            }
        },
        (first_el, second_el) => Err(display_error(ErrorCode::InvalidStack((
            options.pos + 1,
            MType::List(Box::new(first_el.get_type())),
            second_el.get_type(),
            Instruction::CONS,
        )))),
    }
}
