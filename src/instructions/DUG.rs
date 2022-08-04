use crate::instructions::{Instruction, RunOptions};
use crate::stack::{Stack, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DUG

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DUG) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to move
    match Instruction::DUG.check_num_arg(&args) {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_dug_pos) => {
            let el_pos = options.pos + el_to_dug_pos;
            // checks that the stack is deep enough for the DUG parameter
            match stack.check_depth(el_pos, Instruction::DUG) {
                Err(err) => Err(err),
                Ok(_) => {
                    // removes the element at el_pos - 1
                    let (el_to_dug, new_stack) = stack.remove_at(options.pos);
                    // changes the element instruction
                    let el_to_dug = el_to_dug.change_instruction(Instruction::DUG);
                    // insert the element at el_pos - 1
                    let new_stack = new_stack.insert_at(vec![el_to_dug], el_pos);
                    // updates the stack snapshots
                    stack_snapshots.push(new_stack.clone());
                    // returns the stack and stack snapshots
                    Ok((new_stack, stack_snapshots))
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
    use crate::m_types::MValue;
    use crate::stack::StackElement;
    use serde_json::json;

    // PASSING
    #[test]
    fn dug_success_one() {
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
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(6));
                assert_eq!(stack[0].instruction, Instruction::INIT);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::DUG);
                assert_eq!(stack[2].value, MValue::String(String::from("test")));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn dug_success_two() {
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
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Nat(6));
                assert_eq!(stack[0].instruction, Instruction::INIT);
                assert_eq!(stack[1].value, MValue::String(String::from("test")));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Int(5));
                assert_eq!(stack[2].instruction, Instruction::DUG);
            }
            Err(_) => assert!(false),
        }
    }

    // FAILING
    // incorrect value for argument
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 4 for instruction DUG, got 2"
    )]
    fn dug_wrong_arg_value() {
        let arg_value: Value = json!({ "int": "4" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    // DUG 0
    #[test]
    #[should_panic(expected = "DUG 0 is a noop")]
    fn dig_arg_zero() {
        let arg_value: Value = json!({ "int": "0" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
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
        expected = "Unexpected format for DUG argument: Object({\"string\": String(\"test\")})"
    )]
    fn dig_wrong_arg() {
        let arg_value: Value = json!({ "string": "test" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, args, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }
}
