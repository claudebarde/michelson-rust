use crate::instructions::{EmptyCollection, Instruction, RunOptions};
use crate::m_types::{CollectionValue, MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-EMPTY_SET
// https://tezos.gitlab.io/michelson-reference/#instr-EMPTY_MAP
// https://tezos.gitlab.io/michelson-reference/#instr-EMPTY_BIG_MAP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
    instr: EmptyCollection,
) -> Result<(Stack, StackSnapshots), String> {
    // no need to check the stack, it can be empty
    // gets the instruction
    let instruction = match &instr {
        EmptyCollection::Bigmap => Instruction::EMPTY_BIG_MAP,
        EmptyCollection::Map => Instruction::EMPTY_MAP,
        EmptyCollection::Set => Instruction::EMPTY_SET,
    };
    let new_stack_res: Result<Stack, String> = match args {
        None => Err(format!(
            "Arguments for {:?} instruction cannot be empty",
            instruction
        )),
        Some(val) => {
            if val[0].is_object() {
                let new_collection = match val[0]["prim"].as_str() {
                    None => Err(format!(
                        "Missing 'prim' field for {:?} instruction",
                        instruction
                    )),
                    Some(str) => match MType::from_string(str) {
                        Err(err) => Err(err),
                        Ok(collection_type) => {
                            // TODO: handle cases for complex types with "args" property
                            match &instr {
                                EmptyCollection::Bigmap | EmptyCollection::Map => {
                                    // there has to be an "args" property with the type of the keys and values
                                    match val[0]["args"].as_array() {
                                        None => Err(format!(
                                            "Missing 'args' field for {:?} instruction",
                                            instruction
                                        )),
                                        Some(args) => {
                                            if args.len() != 2 {
                                                Err(format!("'args' field for {:?} instruction must have 2 elements", instruction))
                                            } else {
                                                match (args[0].as_str(), args[1].as_str()) {
                                                    (None, _) | (_, None) => Err(format!("Unexpected argument for {:?} instruction in 'args' array", instruction)),
                                                    (Some(key_type_str), Some(value_type_str)) => {
                                                        match (MType::from_string(key_type_str), MType::from_string(value_type_str)) {
                                                            (Err(_), _) | (_, Err(_)) => Err(format!("Unexpected type for {:?} instruction in 'args' array", instruction)),
                                                            (Ok(key_type), Ok(value_type)) =>
                                                                match instr {
                                                                    EmptyCollection::Bigmap => Ok(MValue::new_empty_big_map(key_type, value_type)),
                                                                    EmptyCollection::Map => Ok(MValue::new_empty_big_map(key_type, value_type)),
                                                                    _ => Err(format!("Unexpected collection type for {:?} instruction", instruction))
                                                                }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                EmptyCollection::Set => Ok(MValue::new_empty_set(collection_type)),
                            }
                        }
                    },
                }?;
                Ok(stack)
            } else {
                // TODO: refactor this part so the function can output an Err and not panic (cf LEFT)
                panic!("Expected a 'serde_json::Value' of type object for NIL instruction")
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
