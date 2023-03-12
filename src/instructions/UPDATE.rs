use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{CollectionValue, MType, MValue, PairValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-MAP

pub fn run(
    mut stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::UPDATE;
    // arguments are only present if UPDATE is used top update a right combed pair
    match args {
        None => {
            // updates a map, a big_map or a set
            // checks the stack
            stack.check_depth(options.pos + 3, this_instruction)?;
            // checks if the elements on the stack are correct
            match (
                stack[options.pos].get_val(),
                stack[options.pos + 1].get_val(),
                stack[options.pos + 2].get_val(),
            ) {
                (key, value, MValue::Big_map(big_map)) => todo!(),
                (key, value, MValue::Map(map)) => todo!(),
                (element, MValue::Bool(flag), MValue::Set(set)) => {
                    let new_set = if element.get_type() == set.m_type {
                        // the set doesn't include the element
                        if flag && !set.value.contains(&element) {
                            // adds the element to the set
                            Ok(set.update(element))
                        } else if !flag && set.value.contains(&element) {
                            // removes the element from the set
                            Ok(set.remove(element))
                        } else {
                            // nothing happens
                            Ok(set)
                        }
                    } else {
                        Err(format!(
                                    "Invalid type for instruction `{:?}` expected {} to update the set, but got {}",
                                    this_instruction,
                                    set.m_type.to_string(),
                                    element.get_type().to_string()
                                ))
                    }?;
                    // updates the stack
                    let _ = stack
                        .splice(
                            options.pos..(options.pos + 3),
                            vec![StackElement::new(MValue::Set(new_set), this_instruction)],
                        )
                        .collect::<Stack>();
                    // updates the stack snapshots
                    stack_snapshots.push(stack.clone());
                    // returns the unchanged stack
                    Ok((stack, stack_snapshots))
                }
                _ => Err(format!(
                    "Invalid stack for instruction UPDATE => 0- {} / 1- {} / 2- {}",
                    stack[options.pos].get_val().get_type().to_string(),
                    stack[options.pos + 1].get_val().get_type().to_string(),
                    stack[options.pos + 2].get_val().get_type().to_string()
                )),
            }
        }
        Some(args) => {
            // updates a pair
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::RunOptionsContext;

    #[test]
    fn update_success_add_simple_set() {
        let initial_set = MValue::new_set(
            vec![
                MValue::Nat(2),
                MValue::Nat(3),
                MValue::Nat(4),
                MValue::Nat(5),
            ],
            MType::Nat,
        );
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(9), Instruction::INIT),
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(initial_set, Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 5);

        match run(initial_stack, None, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::new_set(
                        vec![
                            MValue::Nat(9),
                            MValue::Nat(2),
                            MValue::Nat(3),
                            MValue::Nat(4),
                            MValue::Nat(5)
                        ],
                        MType::Nat
                    )
                );
                assert_eq!(stack[0].instruction, Instruction::UPDATE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn update_success_add_simple_set_no_change() {
        let initial_set = MValue::new_set(
            vec![
                MValue::Nat(2),
                MValue::Nat(3),
                MValue::Nat(4),
                MValue::Nat(5),
            ],
            MType::Nat,
        );
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(4), Instruction::INIT),
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(initial_set, Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 5);

        match run(initial_stack, None, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::new_set(
                        vec![
                            MValue::Nat(2),
                            MValue::Nat(3),
                            MValue::Nat(4),
                            MValue::Nat(5)
                        ],
                        MType::Nat
                    )
                );
                assert_eq!(stack[0].instruction, Instruction::UPDATE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn update_success_remove_simple_set() {
        let initial_set = MValue::new_set(
            vec![
                MValue::Nat(2),
                MValue::Nat(3),
                MValue::Nat(4),
                MValue::Nat(5),
            ],
            MType::Nat,
        );
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(initial_set, Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 5);

        match run(initial_stack, None, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::new_set(
                        vec![MValue::Nat(2), MValue::Nat(4), MValue::Nat(5)],
                        MType::Nat
                    )
                );
                assert_eq!(stack[0].instruction, Instruction::UPDATE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn update_success_remove_simple_set_no_change() {
        let initial_set = MValue::new_set(
            vec![
                MValue::Nat(2),
                MValue::Nat(3),
                MValue::Nat(4),
                MValue::Nat(5),
            ],
            MType::Nat,
        );
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(9), Instruction::INIT),
            StackElement::new(MValue::Bool(false), Instruction::INIT),
            StackElement::new(initial_set, Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 5);

        match run(initial_stack, None, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::new_set(
                        vec![
                            MValue::Nat(2),
                            MValue::Nat(3),
                            MValue::Nat(4),
                            MValue::Nat(5)
                        ],
                        MType::Nat
                    )
                );
                assert_eq!(stack[0].instruction, Instruction::UPDATE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
}
