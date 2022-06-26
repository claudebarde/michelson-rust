use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, mutez, nat, timestamp, MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-PUSH

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checking the stack is not required
    // checks that the arguments are correct
    let new_stack_element: Result<StackElement, String> = match args {
        None => Err(display_error(ErrorCode::NoArgument(String::from("PUSH")))),
        Some(arg) => {
            // argument must be a vector of 2 elements
            if arg.len() == 2 {
                // extracts the first argument
                let first_arg = &arg[0];
                let element_type_res: Result<MType, String> =
                    if first_arg.is_object() && first_arg.get("prim").is_some() {
                        // checks if the value is a string
                        match first_arg["prim"].as_str() {
                            None => Err(format!(
                                "JSON value for PUSH type argument is not a valid string: {}",
                                first_arg
                            )),
                            Some(str) => {
                                // checks if the type is a valid Michelson type
                                MType::from_string(str)
                            }
                        }
                    } else {
                        Err(format!(
                        "Expected an object with a \"prim\" field in JSON value for PUSH, got {:?}",
                        first_arg
                    ))
                    };
                let element_type = match element_type_res {
                    Ok(el_type) => el_type,
                    Err(err) => panic!("{}", err),
                };
                // extracts the second argument
                let second_arg = &arg[1];
                let element_value_res: Result<(String, String), String> = if second_arg.is_object()
                {
                    if second_arg.get("int").is_some() {
                        // int value
                        match second_arg.get("int").unwrap().as_str() {
                            None => Err(String::from("Expected value for \"int\" property to be a string (at PUSH instruction)")),
                            Some (str) => Ok((String::from("int"), String::from(str)))
                        }
                    } else if second_arg.get("string").is_some() {
                        // string value
                        match second_arg.get("string").unwrap().as_str() {
                            None => Err(String::from("Expected value for \"string\" property to be a string (at PUSH instruction)")),
                            Some (str) => Ok((String::from("string"), String::from(str)))
                        }
                    } else {
                        //TODO: the property can also be "prim"
                        Err(format!(
                            "JSON value for PUSH value argument is not valid: expected \"int\" or \"string\", but got {}",
                            second_arg
                        ))
                    }
                } else {
                    Err(format!(
                        "Expected an object in JSON value for PUSH, got {:?}",
                        second_arg
                    ))
                };
                let element_value = match element_value_res {
                    Ok(el_value) => el_value,
                    Err(err) => panic!("{}", err),
                };
                // checks that the value matches the type
                match element_type {
                    // numeric types
                    MType::Int => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<int>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(StackElement::new(MValue::Int(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Nat => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<nat>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(StackElement::new(MValue::Nat(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Mutez => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<mutez>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(StackElement::new(MValue::Mutez(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Timestamp => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<timestamp>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(StackElement::new(MValue::Timestamp(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    // other types
                    MType::String => {
                        let (val_type, value) = element_value;
                        if val_type == "string" {
                            Ok(StackElement::new(MValue::String(value), Instruction::PUSH))
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("string"),
                                val_type,
                            ))))
                        }
                    }
                    // TODO: handle all the possible cases
                    _ => Err(String::from(
                        "Unhandled patterns to check type/value in PUSH instruction",
                    )),
                }
            } else {
                Err(display_error(ErrorCode::UnexpectedArgsNumber((
                    2,
                    arg.len(),
                ))))
            }
        }
    };
    // pushes the element to the stack
    match new_stack_element {
        Err(err) => Err(err),
        Ok(stack_el) => {
            let new_stack = stack.insert_at(vec![stack_el], options.pos);
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());
            // returns the new stack
            Ok((new_stack, stack_snapshots))
        }
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use serde_json::{json, Value};

    // PASSING
    #[test]
    fn push_success() {
        let arg_type: Value = json!({"prim": "string"});
        let arg_value: Value = json!({"string": "FA2_NOT_OPERATOR"});
        let arg_vec = vec![arg_type, arg_value];
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
            Ok((new_stack, _)) => {
                assert!(new_stack.len() == 4);
                assert!(new_stack[0].value == MValue::String(String::from("FA2_NOT_OPERATOR")));
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    #[test]
    #[should_panic(expected = "Unexpected number of arguments, expected `2`, got `1`")]
    fn pair_wrong_args_number() {
        let arg_value: Value = json!({"string": "FA2_NOT_OPERATOR"});
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // wrong argument type
    #[test]
    #[should_panic(
        expected = "Expected an object with a \"prim\" field in JSON value for PUSH, got Object({\"int\": String(\"0\")}"
    )]
    fn pair_wrong_args_type() {
        let arg_type: Value = json!({"int": "0"});
        let arg_value: Value = json!({"string": "FA2_NOT_OPERATOR"});
        let arg_vec = vec![arg_type, arg_value];
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // wrong Michelson type
    #[test]
    #[should_panic(expected = "Unknown type 'test'")]
    fn pair_wrong_mich_type() {
        let arg_type: Value = json!({"prim": "test"});
        let arg_value: Value = json!({"string": "FA2_NOT_OPERATOR"});
        let arg_vec = vec![arg_type, arg_value];
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // wrong argument value
    #[test]
    #[should_panic(
        expected = "Expected value for \"string\" property to be a string (at PUSH instruction)"
    )]
    fn pair_wrong_args_value() {
        let arg_type: Value = json!({"prim": "string"});
        let arg_value: Value = json!({"string": ["FA2_NOT_OPERATOR"]});
        let arg_vec = vec![arg_type, arg_value];
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // value doesn't match its type
    #[test]
    #[should_panic(expected = "Invalid argument provided, expected `string`, but got `int`")]
    fn pair_wrong_type_value() {
        let arg_type: Value = json!({"prim": "string"});
        let arg_value: Value = json!({"int": "5"});
        let arg_vec = vec![arg_type, arg_value];
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
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
