use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, OptionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-GET
// https://tezos.gitlab.io/michelson-reference/#instr-GETN

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checking the stack depends on the presence of arguments for GET
    let new_stack: Stack = match args {
        None => {
            // this will get values from maps and bigmaps by key
            stack.check_depth(options.pos + 2, Instruction::GET)?;
            // the first value must of the same type like the keys
            match (&stack[options.pos].value, &stack[options.pos + 1].value) {
                (key, MValue::Map(map)) | (key, MValue::Big_map(map)) => {
                    // checks if the key is of the right type
                    if key.get_type() == map.key_type {
                        // gets the corresponding value
                        let val = match map.value.get(key) {
                            None => MValue::Option(OptionValue::new(None, map.value_type.clone())),
                            Some(el) => MValue::Option(OptionValue::new(
                                Some(el.clone()),
                                map.value_type.clone(),
                            )),
                        };
                        // removes the current elements from the stack
                        let mut new_stack = stack;
                        new_stack.drain(options.pos..options.pos + 2);
                        // pushes the new element to the stack
                        let new_stack = new_stack
                            .insert_at(vec![StackElement::new(val, Instruction::GET)], options.pos);

                        Ok(new_stack)
                    } else {
                        Err(display_error(ErrorCode::WrongType((
                            map.key_type.to_string(),
                            key.get_type().to_string(),
                            Instruction::GET
                        ))))
                    }
                }
                _ => Err(format!(
                    "Invalid type for `GET n` expected 'map' or 'big_map', but got {}",
                    stack[options.pos + 1].value.get_type().to_string()
                )),
            }
        }
        Some(arg) => {
            // this will get values in nested pairs
            // checks the stack
            stack.check_depth(options.pos + 1, Instruction::GET)?;
            // verifies that the element on the stack is a pair
            match &stack[options.pos].value {
                MValue::Pair(pair) => {
                    // verifies that the argument is correct
                    if arg.len() == 1 {
                        if arg[0].is_object() {
                            match arg[0]["int"].as_str() {
                                None => Err(String::from("Expected argument for GET instruction to be an object with an 'int' property")),
                                Some(val) => {
                                    match val.parse::<usize>() {
                                        Err(err) => Err(format!("Expected argument for GET instruction to be a number, but got {} instead ({:?})", val, err)),
                                        Ok(el_pos) => {
                                            // checks if the pair is right-combed with the right depth
                                            let new_val: MValue = match &pair.check_right_comb_depth() {
                                                None => if el_pos == 0 
                                                    { 
                                                        Ok(MValue::Pair(pair.clone())) 
                                                    } else { 
                                                        Err(format!("The pair for the instruction GET doesn't have the correct depth for the provided argument: {}", el_pos)) 
                                                    },
                                                Some(depth) => {
                                                    // checks if the depth of the pair matches the requested depth
                                                    let required_depth = if el_pos % 2 != 0 { (el_pos + 1) / 2 } else { el_pos / 2 };
                                                    if required_depth > *depth {
                                                        Err(format!("The pair is not deep enough for instruction GET, expected a depth of {}, but got {}", required_depth, depth))
                                                    } else {
                                                        match pair.unfold(required_depth) {
                                                            Err(err) => Err(err),
                                                            Ok(pair_val) => {
                                                                if el_pos % 2 == 0 {
                                                                    // right field
                                                                    let m_type = &pair_val.value.1.get_type();
                                                                    Ok(
                                                                        MValue::Option(
                                                                            OptionValue::new(
                                                                                Some(pair_val.value.1), 
                                                                                m_type.clone()
                                                                            )
                                                                        )
                                                                    )
                                                                } else {
                                                                    // left field
                                                                    let m_type = &pair_val.value.0.get_type();
                                                                    Ok(
                                                                        MValue::Option(
                                                                            OptionValue::new(
                                                                                Some(pair_val.value.0), 
                                                                                m_type.clone()
                                                                            )
                                                                        )
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }?;

                                            Ok(stack.replace(vec![StackElement::new(new_val, Instruction::GET)], options.pos))
                                        }                                        
                                    }
                                }
                            }
                        } else {
                            Err(String::from(
                                "Expected a 'serde_json::Value' of type object for GET instruction",
                            ))
                        }
                    } else {
                        Err(format!(
                            "Unexpected length of arg vector for GET instruction, expected 1, but got {}",
                            arg.len()
                        ))
                    }
                }
                _ => Err(
                    display_error(
                        ErrorCode::WrongType((String::from("pair"), stack[options.pos].value.get_type().to_string(), Instruction::GET))
                    )
                )
            }
        }
    }?;

    //updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, PairValue};
    use serde_json::json;

    // PASSING
    // get value out of a map
    #[test]
    fn get_map_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(6), Instruction::INIT),
            StackElement::new(
                MValue::new_map(
                    MType::Int, 
                    MType::String, 
                    vec![
                        (MValue::Int(5), MValue::new_string("tezos")),
                        (MValue::Int(6), MValue::new_string("taquito")),
                        (MValue::Int(7), MValue::new_string("hello")),
                        (MValue::Int(8), MValue::new_string("world")),
                        (MValue::Int(9), MValue::new_string("blockchain")),
                    ]), 
                Instruction::INIT
            ),
            StackElement::new(MValue::Int(8), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };
        let args: Option<&Vec<Value>> = None;

        assert!(initial_stack.len() == 4);

        match run(initial_stack, args, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Option(OptionValue::new(Some(MValue::new_string("taquito")), MType::String)));
                assert_eq!(stack[0].instruction, Instruction::GET);
                assert_eq!(stack[1].value, MValue::Int(8));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // get value out of a pair
    #[test]
    fn get_pair_success() {
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Pair(PairValue::new(
                    MValue::Nat(9),
                    MValue::Pair(PairValue::new(
                        MValue::Nat(11),
                        MValue::Pair(PairValue::new(
                            MValue::Nat(12),
                            MValue::new_string("taquito")
                        ))
                    ))
                )), 
                Instruction::INIT
            ),
            StackElement::new(MValue::Int(8), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };
        let arg_value: Value = json!({ "int": "5" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);

        assert!(initial_stack.len() == 3);

        match run(initial_stack, args, &options, stack_snapshots) {
            Err(err) => {
                println!("{}", err);
                assert!(false)
            },
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Option(OptionValue::new(Some(MValue::Nat(12)), MType::Nat)));
                assert_eq!(stack[0].instruction, Instruction::GET);
                assert_eq!(stack[1].value, MValue::Int(8));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
}
