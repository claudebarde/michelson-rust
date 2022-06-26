use crate::errors::{display_error, ErrorCode};
use crate::instructions::Instruction;
use crate::instructions::RunOptions;
use crate::stack::{Stack, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DROP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DROP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to drop
    let el_to_drop_pos_res: Result<usize, String> = match args {
        None => Ok(options.pos + 1),
        Some(arg) => {
            if arg.len() == 1 {
                let arg = &arg[0];
                if arg.is_object() && arg.get("int").is_some() {
                    // gets the int value that will be stored as a string
                    match arg.get("int").unwrap().as_str() {
                        None => Err(String::from("Expected a String in JSON value for DROP")),
                        Some(str) =>
                        // parse the string into a number
                        {
                            match str.parse::<usize>() {
                                Err(_) => Err(format!(
                                    "JSON value for DROP argument is not a valid number: {}",
                                    str
                                )),
                                Ok(val) => {
                                    // DROP 0 is a noop
                                    if val == 0 {
                                        Err(format!(
                                            "{:?}",
                                            ErrorCode::Noop(String::from("DROP 0 is a noop"))
                                        ))
                                    } else {
                                        // checks that the stack is deep enough for the DROP parameter
                                        match stack
                                            .check_depth(options.pos + val, Instruction::DROP)
                                        {
                                            Err(err) => Err(err),
                                            Ok(_) => Ok(options.pos + val),
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Err(format!("Unexpected format for DROP argument: {:?}", arg))
                }
            } else {
                Err(format!(
                    "{:?}",
                    display_error(ErrorCode::UnexpectedArgsNumber((1, arg.len())))
                ))
            }
        }
    };

    match el_to_drop_pos_res {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_drop_pos) => {
            // drops the element at position - 1
            let new_stack: Stack = stack
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i > el_to_drop_pos - 1)
                .map(|(_, e)| e)
                .collect();
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());
            // returns the new stack
            Ok((new_stack, stack_snapshots))
        }
    }
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::MValue;
    use crate::stack::StackElement;
    use serde_json::json;

    // PASSING TESTS
    #[test]
    fn drop_one_no_args() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Nat(6));
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn drop_one_with_args() {
        let arg_value: Value = json!({ "int": "1" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Nat(6));
                assert_eq!(stack[1].value, MValue::Mutez(6_000_000));
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn drop_two_with_args() {
        let arg_value: Value = json!({ "int": "2" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Mutez(6_000_000));
            }
            Err(err) => panic!("{}", err),
        }
    }

    // FAILING TESTS
    // Wrong arguments
    #[test]
    #[should_panic(
        expected = "Unexpected format for DROP argument: Object({\"string\": String(\"test\")})"
    )]
    fn drop_wrong_args() {
        let arg_value: Value = json!({ "string": "test" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // DROP 0 is a noop
    #[test]
    #[should_panic(expected = "DROP 0 is a noop")]
    fn drop_zero() {
        let arg_value: Value = json!({ "int": "0" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // Stack is not deep enough for DROP
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 1 for instruction DROP, got 0"
    )]
    fn drop_zero_depth() {
        let arg_value: Value = json!({ "int": "1" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // Stack is not deep enough for DROP argument
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 3 for instruction DROP, got 2"
    )]
    fn drop_wrong_depth() {
        let arg_value: Value = json!({ "int": "3" });
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
            },
            pos: 0,
        };

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
