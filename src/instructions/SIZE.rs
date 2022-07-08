use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{nat, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SIZE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::SIZE) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // SIZE can be used only with string, list, set, map or bytes
    let size_res: Result<MValue, String> = match &stack[options.pos].value {
        MValue::String(val) => Ok(MValue::Nat(val.len() as nat)),
        MValue::Bytes(val) => Ok(MValue::Nat(val.len() as nat)),
        MValue::List(list) => Ok(MValue::Nat(list.value.len() as nat)),
        MValue::Set(set) => Ok(MValue::Nat(set.value.len() as nat)),
        MValue::Map(map) => Ok(MValue::Nat(map.size as nat)),
        _ => Err(format!(
            "Expected string, bytes, list, set or map for SIZE, but got {}",
            &stack[options.pos].value.to_string()
        )),
    };

    match size_res {
        Ok(mval) => {
            // removes the element affected by SIZE
            // pushes the size of the element to the stack
            let new_stack = stack.replace(
                vec![StackElement::new(mval, Instruction::SIZE)],
                options.pos,
            );
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());
            // returns the new stack
            Ok((new_stack, stack_snapshots))
        }
        Err(err) => Err(err),
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::MType;

    // PASSING
    // test for string
    #[test]
    fn size_string_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(4));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // test for bytes
    #[test]
    fn size_bytes_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bytes(String::from("74657374")), Instruction::INIT),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(8));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // test for list
    #[test]
    fn size_list_success() {
        // populated list
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_list(
                    vec![MValue::Nat(3), MValue::Nat(4), MValue::Nat(5)],
                    MType::Nat,
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(3));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }

        // empty list
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_empty_list(MType::Nat), Instruction::INIT),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(0));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // test for set
    #[test]
    fn size_set_success() {
        // populated list
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_set(
                    vec![MValue::Nat(3), MValue::Nat(4), MValue::Nat(5)],
                    MType::Nat,
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(3));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }

        // empty set
        let initial_stack: Stack = vec![
            StackElement::new(MValue::new_empty_set(MType::Nat), Instruction::INIT),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(0));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // test for map
    #[test]
    fn size_map_success() {
        // populated list
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_map(
                    MType::Nat,
                    MType::Int,
                    vec![
                        (MValue::Nat(1), MValue::Nat(2)),
                        (MValue::Nat(3), MValue::Nat(4)),
                        (MValue::Nat(5), MValue::Nat(6)),
                    ],
                ),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(3));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }

        // empty set
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::new_empty_map(MType::Nat, MType::Nat),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(0));
                assert_eq!(stack[0].instruction, Instruction::SIZE);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
}
