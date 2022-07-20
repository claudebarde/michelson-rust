use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, Or, OrValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-LEFT

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::LEFT) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    match args {
        Some(arg) => {
            if arg[0].is_object() {
                let new_union = match arg[0]["prim"].as_str() {
                    None => Err(String::from("Expected string for the union element type")),
                    Some(str) => match MType::from_string(str) {
                        Err(err) => Err(err),
                        Ok(right_type) => {
                            let val_to_wrap = stack[options.pos].value.clone();
                            // gets the type for the left value
                            let left_type = stack[options.pos].value.get_type();
                            // wraps the value at options.pos
                            let new_val: MValue = MValue::Or(OrValue::new(
                                Or::Left(val_to_wrap),
                                (left_type, right_type),
                            ));
                            // creates the new stack element
                            Ok(new_val)
                        }
                    },
                }?;
                // updates the stack
                let new_stack = stack.replace(
                    vec![StackElement::new(new_union, Instruction::LEFT)],
                    options.pos,
                );
                // updates the stack snapshots
                stack_snapshots.push(new_stack.clone());

                Ok((new_stack, stack_snapshots))
            } else {
                Err(String::from(
                    "Expected a 'serde_json::Value' of type object for LEFT instruction",
                ))
            }
        }
        None => Err(String::from(
            "Arguments for LEFT instruction cannot be empty",
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

        match run(initial_stack, args, &options, stack_snapshots) {
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
}
