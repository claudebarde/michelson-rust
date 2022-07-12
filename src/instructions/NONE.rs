use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-NONE

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // no need to check the stack, it can be empty
    let new_stack_res: Result<Stack, String> = match args {
        None => Err(String::from(
            "Arguments for NONE instruction cannot be empty",
        )),
        Some(val) => {
            if val[0].is_object() {
                let new_opt_res: Result<StackElement, String> = match val[0]["prim"].as_str() {
                    None => Err(String::from("Expected string for the option element type")),
                    Some(str) => match MType::from_string(str) {
                        Err(err) => Err(err),
                        Ok(option_type) => Ok(StackElement::new(
                            MValue::Option(OptionValue {
                                m_type: option_type,
                                value: Box::new(None),
                            }),
                            Instruction::NONE,
                        )),
                    },
                };
                match new_opt_res {
                    Err(err) => Err(err),
                    Ok(new_option) => {
                        let new_stack = stack.insert_at(vec![new_option], options.pos);
                        Ok(new_stack)
                    }
                }
            } else {
                panic!("Expected a 'serde_json::Value' of type object")
            }
        }
    };
    match new_stack_res {
        Err(err) => Err(err),
        Ok(new_stack) => {
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());

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
    use crate::m_types::{MType, OptionValue};
    use serde_json::json;

    // PASSING TESTS
    // pushes a new option of type nat to the stack
    #[test]
    fn option_of_nat() {
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

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                let expected_option = OptionValue {
                    m_type: MType::Nat,
                    value: Box::new(None),
                };
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Option(expected_option))
            }
            Err(err) => panic!("{}", err),
        }
    }

    // FAILING TESTS
    // No argument
    #[test]
    #[should_panic(expected = "Arguments for NONE instruction cannot be empty")]
    fn nil_empty_args() {
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

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // Wrong argument
    #[test]
    #[should_panic(expected = "Expected string for the option element type")]
    fn nil_wrong_args() {
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // Wrong argument
    #[test]
    #[should_panic(expected = "Unknown type 'test'")]
    fn nil_wrong_arg_type() {
        let arg_value: Value = json!({ "prim": "test" });
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

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
