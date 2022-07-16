use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
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

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    #[test]
    fn cons_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("hello"), Instruction::INIT),
            StackElement::new(MValue::new_list(vec![MValue::new_string("world")], MType::String), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
            level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                match &stack[0].value {
                    MValue::List(list) => {
                        assert_eq!(list.size(), 2);
                        assert_eq!(list.value[0], MValue::new_string("hello"));
                        assert_eq!(list.value[1], MValue::new_string("world"));
                    },
                    _ => assert!(false)
                }
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // wrong stack
    #[test]
    fn cons_wrong_stack() {
        // stack not deep enough
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("hello"), Instruction::INIT)
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
            level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(err, "Unexpected stack length, expected a length of 2 for instruction CONS, got 1"),
            Ok(_) => assert!(false)
        }

        // stack with wrong values
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("hello"), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
            level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(err, "Expected element at position 1 to be of type list, but got mutez for instruction CONS"),
            Ok(_) => assert!(false)
        }
    }

    // value to cons doesn't match list type
    fn cons_wrong_element_type() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::new_list(vec![MValue::new_string("world")], MType::String), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
            level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(err, "Element to prepend to the list with CONS is of type nat, while the list elements are of type string"),
            Ok(_) => assert!(false)
        }
    }
}
