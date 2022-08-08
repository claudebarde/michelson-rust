use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue, OptionValue};
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
            stack.check_depth(options.pos + 1, Instruction::GET)?;

            Ok(stack)
        }
    }?;

    //updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}
