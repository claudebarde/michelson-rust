use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DIG

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DIG) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to move
    let el_to_dig_pos_res: Result<usize, String> = match args {
        None => Err(display_error(ErrorCode::NoArgument(Instruction::DIG))),
        Some(arg) => {
            if arg.len() == 1 {
                let arg = &arg[0];
                if arg.is_object() && arg.get("int").is_some() {
                    // gets the int value that will be stored as a string
                    match arg.get("int").unwrap().as_str() {
                        None => Err(String::from("Expected a string in JSON value for DIG")),
                        Some(str) =>
                        // parse the string into a number
                        {
                            match str.parse::<usize>() {
                                Err(_) => Err(format!(
                                    "JSON value for DIG argument is not a valid number: {}",
                                    str
                                )),
                                Ok(val) => {
                                    // DIG 0 is a noop
                                    if val == 0 {
                                        Err(format!(
                                            "{:?}",
                                            ErrorCode::Noop(String::from("DIG 0 is a noop"))
                                        ))
                                    } else {
                                        // checks that the stack is deep enough for the DIG parameter
                                        match stack.check_depth(options.pos + val, Instruction::DIG)
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
                    Err(format!("Unexpected format for DIG argument: {:?}", arg))
                }
            } else {
                Err(format!(
                    "{:?}",
                    display_error(ErrorCode::UnexpectedArgsNumber((1, arg.len())))
                ))
            }
        }
    };

    match el_to_dig_pos_res {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_dig_pos) => {
            // removes the element at el_to_dig_pos
            let (el_to_insert, new_stack) = stack.remove_at(el_to_dig_pos);
            // changes the instruction name of the element
            let el_to_insert = el_to_insert.change_instruction(Instruction::DIG);
            // adds the element to the top of the stack
            let new_stack = new_stack.insert_at(vec![el_to_insert], options.pos);
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
    use serde_json::json;

    // PASSING
    #[test]
    fn dig_success_one() {
        let arg_value: Value = json!({ "int": "1" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(6));
                assert_eq!(stack[0].instruction, Instruction::DIG);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::String(String::from("test")));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn dig_success_two() {
        let arg_value: Value = json!({ "int": "2" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
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
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::String(String::from("test")));
                assert_eq!(stack[0].instruction, Instruction::DIG);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    // incorrect value for argument
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 4 for instruction DIG, got 2"
    )]
    fn dig_wrong_arg_value() {
        let arg_value: Value = json!({ "int": "4" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // DUP 0
    #[test]
    #[should_panic(expected = "DIG 0 is a noop")]
    fn dup_arg_zero() {
        let arg_value: Value = json!({ "int": "0" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // incorrect argument
    #[test]
    #[should_panic(
        expected = "Unexpected format for DIG argument: Object({\"string\": String(\"test\")})"
    )]
    fn dup_wrong_arg() {
        let arg_value: Value = json!({ "string": "test" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
