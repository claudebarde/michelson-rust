use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DUP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DUP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to duplicate
    match Instruction::DUP.check_num_arg(&args) {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_dig_pos) => {
            let el_pos = options.pos + el_to_dig_pos;
            // checks that the stack is deep enough for the DIG parameter
            match stack.check_depth(el_pos, Instruction::DUP) {
                Err(err) => Err(err),
                Ok(_) => {
                    // duplicates the element at el_to_dup_pos
                    let dupped_el = stack[el_pos - 1].clone();
                    // checks if element is not a ticket
                    if let MValue::Ticket(_) = dupped_el.value {
                        Err(String::from("Tickets cannot be duplicated"))
                    } else {
                        // changes the instruction name of the dupped element
                        let dupped_el = StackElement::new(dupped_el.value, Instruction::DUP);
                        // adds the element to the top of the stack
                        let new_stack = stack.insert_at(vec![dupped_el], options.pos);
                        // updates the stack snapshots
                        stack_snapshots.push(new_stack.clone());
                        // returns the new stack
                        Ok((new_stack, stack_snapshots))
                    }
                }
            }
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
    use crate::m_types::Ticket;
    use serde_json::json;

    // PASSING
    // no arg
    #[test]
    fn dup_no_arg_success() {
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Int(5));
                assert_eq!(stack[0].instruction, Instruction::DUP);
            }
            Err(_) => assert!(false),
        }
    }

    // one arg
    #[test]
    fn dup_one_arg_success() {
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 4);
                assert_eq!(stack[0].value, MValue::Nat(6));
                assert_eq!(stack[0].instruction, Instruction::DUP);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
                assert_eq!(stack[3].value, MValue::String(String::from("test")));
                assert_eq!(stack[3].instruction, Instruction::INIT);
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    // incorrect value for argument
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 4 for instruction DUP, got 2"
    )]
    fn dup_wrong_arg_value() {
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
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

    // DUP 0
    #[test]
    #[should_panic(expected = "DUP 0 is a noop")]
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
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

    // incorrect argument
    #[test]
    #[should_panic(
        expected = "Unexpected format for DUP argument: Object({\"string\": String(\"test\")})"
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
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

    // cannot duplicate tickets
    #[test]
    #[should_panic(expected = "Tickets cannot be duplicated")]
    fn dup_ticket() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Ticket(Box::new(Ticket::new(
                    5,
                    MValue::Int(5),
                    String::from("test_address"),
                ))),
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
                level: 11,
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
