use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-CONCAT

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checking the stack will depend on the elements in it
    // but there must be at least one element
    stack.check_depth(options.pos + 1, Instruction::CONCAT)?;
    // stack with 2 strings
    let (concat_res, el_num) = 
        // checks if the stack is correct and does the concatenation according to the stack shape
        if let MValue::String(first_string) = &stack[options.pos].value {
            // the following value on the stack must be a string too
            if let MValue::String(second_string) = &stack[options.pos + 1].value {
                // concatenates the strings
                let mut concat_string = first_string.to_owned();
                concat_string.push_str(second_string);
                Ok((MValue::String(concat_string), 2))
            } else {
                Err(format!(
                    "Expected an element of type string at position {}, but got {}",
                    options.pos + 1,
                    stack[options.pos + 1].value.get_type().to_string()
                ))
            }
        }
        // stack with 2 bytes
        // TODO: handle cases when the bytes start with 0x
        else if let MValue::Bytes(first_bytes) = &stack[options.pos].value {
            // the following value on the stack must be a string of bytes too
            if let MValue::Bytes(second_bytes) = &stack[options.pos + 1].value {
                // concatenates the strings
                let mut concat_bytes = first_bytes.to_owned();
                concat_bytes.push_str(second_bytes);
                Ok((MValue::Bytes(concat_bytes), 2))
            } else {
                Err(format!(
                    "Expected an element of type bytes at position {}, but got {}",
                    options.pos + 1,
                    stack[options.pos + 1].value.get_type().to_string()
                ))
            }
        }
        // stack with a list
        else if let MValue::List(list) = &stack[options.pos].value {
            // checks if the list elements are of type string or bytes
            match list.m_type {
                MType::String => {
                    // type validation of list elements
                    // NOTE: checking the types of the elements in the list here
                    // may not be necessary as it is done again below
                    list.check_elements_type(MType::String, Instruction::CONCAT)?;
                    // list concatenation
                    Ok((MValue::String(
                        list.value
                        .clone()
                        .into_iter()
                        .map(|val| match val {
                            MValue::String(str) => str,
                            _ => panic!("Found value of type {} in a list of strings at CONCAT", val.get_type().to_string())
                        })
                        .collect::<Vec<String>>()
                        .join("")
                    ), 1))
                }
                MType::Bytes => {
                    // type validation of list elements
                    // NOTE: checking the types of the elements in the list here
                    // may not be necessary as it is done again below
                    list.check_elements_type(MType::Bytes, Instruction::CONCAT)?;
                    // list concatenation
                    Ok((MValue::Bytes(
                        list.value
                        .clone()
                        .into_iter()
                        .map(|val| match val {
                            MValue::Bytes(str) => str,
                            _ => panic!("Found value of type {} in a list of bytes at CONCAT", val.get_type().to_string())
                        })
                        .collect::<Vec<String>>()
                        .join("")
                    ), 1))
                },
                _ => Err(
                    display_error(
                        ErrorCode::InvalidType(
                            (vec![MType::String, MType::Bytes], list.m_type.clone(), Instruction::CONCAT)
                        )
                    )
                )
            }
        } else {
            Err(format!(
                "Expected an element of type string, bytes or list at position {}, but got {}",
                options.pos,
                stack[options.pos].value.get_type().to_string()
            ))
        }?;

    // updates the stack
    let new_stack: Stack = if el_num == 1 {
        // removes the list from the stack
        stack.replace(vec![StackElement::new(concat_res, Instruction::CONCAT)], options.pos)
    } else if el_num == 2 {
        // removes the 2 elements being added from the stack
        let (_, new_stack) = stack.remove_at(options.pos);
        // pushes the new element to the stack
        new_stack.replace(
            vec![StackElement::new(concat_res, Instruction::CONCAT)],
            options.pos,
        )
    } else {
        panic!("Unexpected number of elements to remove for CONCAT, expected 1 or 2, got {}", el_num)
    };
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());
    // returns the new stacka nd stack snapshots
    Ok((new_stack, stack_snapshots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::CollectionValue;

    // PASSING
    #[test]
    // concats strings
    fn concat_string_string() {
        // should output "hello world"
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("hello "), Instruction::INIT),
            StackElement::new(MValue::new_string("world"), Instruction::INIT),
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
                assert_eq!(stack[0].value, MValue::new_string("hello world"));
                assert_eq!(stack[0].instruction, Instruction::CONCAT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    // concats bytes
    fn concat_bytes_bytes() {
        // should output "hello world"
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("68656c6c6f20"), Instruction::INIT),
            StackElement::new(MValue::new_string("776f726c64"), Instruction::INIT),
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
                assert_eq!(stack[0].value, MValue::new_string("68656c6c6f20776f726c64"));
                assert_eq!(stack[0].instruction, Instruction::CONCAT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    // concats a list of strings
    fn concat_list_of_strings() {
        // should output "hello world!"
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_list(
                    vec![MValue::new_string("hello "), MValue::new_string("world"), MValue::new_string("!")], 
                    MType::String
                ), Instruction::INIT
            ),
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
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::new_string("hello world!"));
                assert_eq!(stack[0].instruction, Instruction::CONCAT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    // concats a list of bytes
    fn concat_list_of_bytes() {
        // should output "68656c6c6f20776f726c6421"
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_list(
                    vec![MValue::new_bytes("68656c6c6f20"), MValue::new_bytes("776f726c64"), MValue::new_bytes("21")], 
                    MType::Bytes
                ), Instruction::INIT
            ),
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
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::new_bytes("68656c6c6f20776f726c6421"));
                assert_eq!(stack[0].instruction, Instruction::CONCAT);
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    #[test]
    fn concat_wrong_types() {
        // first element of wrong type
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::new_string("world"), Instruction::INIT),
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
            Err(err) => assert_eq!(
                err, 
                String::from("Expected an element of type string, bytes or list at position 0, but got nat")
            ),
            Ok(_) => assert!(false)
        }

        // second element of wrong type
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_string("world"), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
            Err(err) => assert_eq!(
                err, 
                String::from("Expected an element of type string at position 1, but got nat")
            ),
            Ok(_) => assert!(false)
        }

        // list of elements of wrong type
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_list(vec![MValue::Nat(4), MValue::Nat(5), MValue::Nat(6)], MType::Nat), Instruction::INIT),
            StackElement::new(MValue::new_string("world"), Instruction::INIT),
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
            Err(err) => assert_eq!(
                err, 
                String::from("Invalid type for `CONCAT` expected string | bytes, but got nat")
            ),
            Ok(_) => assert!(false)
        }

        // list of elements with one element of the wrong type
        // NOTE: in theory, this case is impossible, but it's better to test it
        let initial_stack: Stack = vec![
            StackElement::new(MValue::List(CollectionValue {
                m_type: MType::String,
                value: Box::new(vec![MValue::new_string("hello "), MValue::new_string("world"), MValue::Nat(9)]),
            }), Instruction::INIT),
            StackElement::new(MValue::new_string("world"), Instruction::INIT),
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
            Err(err) => assert_eq!(
                err, 
                String::from("Expected values of type string in list for `CONCAT`, but got a value of type nat")
            ),
            Ok(_) => assert!(false)
        }
    }
}
