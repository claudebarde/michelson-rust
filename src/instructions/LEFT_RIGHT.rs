use crate::instructions::{Instruction, LeftOrRight, RunOptions};
use crate::m_types::{MType, MValue, Or, OrValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-LEFT

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
    left_or_right: LeftOrRight,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::LEFT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // picks the right instruction
    let instruction = match left_or_right {
        LeftOrRight::Left => Instruction::LEFT,
        LeftOrRight::Right => Instruction::RIGHT,
    };
    // matches on the argument
    match args {
        Some(arg) => {
            if arg[0].is_object() {
                let new_union = match arg[0]["prim"].as_str() {
                    None => Err(format!(
                        "Missing 'prim' field for {:?} instruction",
                        instruction
                    )),
                    Some(str) => match MType::from_string(str) {
                        Err(err) => Err(err),
                        Ok(second_type) => {
                            let val_to_wrap = stack[options.pos].value.clone();
                            // gets the type for the value
                            let val_type = stack[options.pos].value.get_type();
                            // creates the value and wraps the value at options.pos
                            let new_val = match left_or_right {
                                LeftOrRight::Left => MValue::Or(OrValue::new(
                                    Or::Left(val_to_wrap),
                                    (val_type, second_type),
                                )),
                                LeftOrRight::Right => MValue::Or(OrValue::new(
                                    Or::Right(val_to_wrap),
                                    (second_type, val_type),
                                )),
                            };
                            // creates the new stack element
                            Ok(new_val)
                        }
                    },
                }?;
                // updates the stack
                let new_stack =
                    stack.replace(vec![StackElement::new(new_union, instruction)], options.pos);
                // updates the stack snapshots
                stack_snapshots.push(new_stack.clone());

                Ok((new_stack, stack_snapshots))
            } else {
                Err(format!(
                    "Expected a 'serde_json::Value' of type object for {:?} instruction",
                    instruction
                ))
            }
        }
        None => Err(format!(
            "Arguments for {:?} instruction cannot be empty",
            instruction
        )),
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, OrValue};
    use serde_json::json;

    // PASSING TESTS
    // pushes a new empty list of nat to the stack
    #[test]
    fn left_success() {
        let arg_value: Value = json!({ "prim": "nat" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
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

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            LeftOrRight::Left,
        ) {
            Ok((stack, _)) => {
                let expected_union = OrValue {
                    m_type: (MType::Int, MType::Nat),
                    value: Box::new(Or::Left(MValue::Int(5))),
                };
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Or(expected_union));
                assert_eq!(stack[0].instruction, Instruction::LEFT);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn right_success() {
        let arg_value: Value = json!({ "prim": "int" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
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

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            LeftOrRight::Right,
        ) {
            Ok((stack, _)) => {
                let expected_union = OrValue {
                    m_type: (MType::Int, MType::Nat),
                    value: Box::new(Or::Right(MValue::Nat(5))),
                };
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Or(expected_union));
                assert_eq!(stack[0].instruction, Instruction::RIGHT);
            }
            Err(err) => panic!("{}", err),
        }
    }

    // FAILING TESTS
    #[test]
    #[should_panic(expected = "Arguments for LEFT instruction cannot be empty")]
    fn left_empty_arg() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
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

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            LeftOrRight::Left,
        ) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    #[should_panic(expected = "Missing 'prim' field for RIGHT instruction")]
    fn right_arg_wrong_format() {
        let arg_value: Value = json!({ "string": "int" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
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

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            LeftOrRight::Right,
        ) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction LEFT, got 0"
    )]
    fn left_wrong_stack() {
        let arg_value: Value = json!({ "prim": "int" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![];
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

        assert!(initial_stack.len() == 0);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            LeftOrRight::Left,
        ) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
