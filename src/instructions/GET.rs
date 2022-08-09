use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, OptionValue, PairValue};
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
                                        Ok(num_val) => {
                                            // checks if the pair is right-combed with the right depth
                                            let new_val: MValue = match &pair.check_right_comb_depth() {
                                                None => if num_val == 0 
                                                    { 
                                                        Ok(MValue::Pair(pair.clone())) 
                                                    } else { 
                                                        Err(format!("The pair for the instruction GET doesn't have the correct depth for the provided argument: {}", num_val)) 
                                                    },
                                                Some(depth) => {
                                                    Ok(MValue::Pair(pair.clone()))
                                                }
                                            }?;
                                            Ok(stack)
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
