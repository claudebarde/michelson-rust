use crate::instructions::{EmptyCollection, Instruction, RunOptions};
use crate::m_types::{MType, MValue};
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
    match args {
        None => Err(format!(
            "Arguments for {:?} instruction cannot be empty",
            instruction
        )),
        Some(val) => {
            if val[0].is_object() {
                let new_collection = match (val[0]["prim"].as_str(), &instr) {
                    (None, EmptyCollection::Set) => Err(format!(
                        "Missing 'prim' field in argument for {:?} instruction",
                        instruction
                    )),
                    (Some(str), EmptyCollection::Set) => match MType::from_string(str) {
                        Err(err) => Err(err),
                        Ok(collection_type) => Ok(MValue::new_empty_set(collection_type)),
                    },
                    (None, EmptyCollection::Map | EmptyCollection::Bigmap) => {
                        // there has to be an "args" property with the type of the keys and values
                        match val[0]["args"].as_array() {
                            None => Err(format!(
                                "Missing 'args' field in argument for {:?} instruction",
                                instruction
                            )),
                            Some(args) => {
                                if args.len() != 2 {
                                    Err(format!(
                                        "'args' field for {:?} instruction must have 2 elements, got {:?}",
                                        instruction, args.len()
                                    ))
                                } else {
                                    match (args[0]["prim"].as_str(), args[1]["prim"].as_str()) {
                                        (None, _) | (_, None) => Err(format!("Unexpected argument for {:?} instruction in 'args' array", instruction)),
                                        (Some(key_type_str), Some(value_type_str)) => {
                                            match (MType::from_string(key_type_str), MType::from_string(value_type_str)) {
                                                (Err(err), _) => Err(format!("Unexpected type for {:?} instruction in 'args' array: {:?}", instruction, err)),
                                                (_, Err(err)) => Err(format!("Unexpected type for {:?} instruction in 'args' array: {:?}", instruction, err)),
                                                (Ok(key_type), Ok(value_type)) =>
                                                    match instr {
                                                        EmptyCollection::Bigmap => Ok(MValue::new_empty_big_map(key_type, value_type)),
                                                        EmptyCollection::Map => Ok(MValue::new_empty_map(key_type, value_type)),
                                                        _ => Err(format!("Unexpected collection type for {:?} instruction", instruction))
                                                    }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => Err(format!(
                        "Unexpected pattern for {:?} instruction to build new empty collection",
                        instruction
                    )),
                }?;
                // inserts the new element into the stack
                let new_stack = stack.insert_at(
                    vec![StackElement::new(new_collection, instruction)],
                    options.pos,
                );
                // updates the stack snapshots
                stack_snapshots.push(new_stack.clone());

                Ok((new_stack, stack_snapshots))
            } else {
                Err(format!(
                    "Expected a 'serde_json::Value' of type object as argument for {:?} instruction",
                    instruction
                ))
            }
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
    use crate::m_types::{CollectionValue, MapValue};
    use serde_json::json;
    use std::collections::HashMap;

    // PASSING
    #[test]
    fn empty_set_success() {
        let arg_value: Value = json!({ "prim": "nat" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Set,
        ) {
            Ok((stack, _)) => {
                let expected_set = CollectionValue {
                    m_type: MType::Nat,
                    value: Box::new(vec![]),
                };
                assert!(stack.len() == 3);
                match &stack[0].value {
                    MValue::Set(set) => {
                        assert_eq!(*set, expected_set);
                        assert_eq!(set.size(), 0);
                    }
                    _ => assert!(false),
                };
                assert_eq!(stack[0].instruction, Instruction::EMPTY_SET);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn empty_map_success() {
        let arg_value: Value = json!({ "args": [{"prim": "nat"}, {"prim": "string"}] });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Map,
        ) {
            Ok((stack, _)) => {
                let expected_map = MapValue {
                    is_map: true,
                    key_type: MType::Nat,
                    value_type: MType::String,
                    value: HashMap::new(),
                };
                assert!(stack.len() == 3);
                match &stack[0].value {
                    MValue::Map(map) => {
                        assert_eq!(map.size(), Ok(0));
                        assert_eq!(*map, expected_map)
                    }
                    _ => assert!(false),
                };
                assert_eq!(stack[0].instruction, Instruction::EMPTY_MAP);
            }
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn empty_big_map_success() {
        let arg_value: Value = json!({ "args": [{"prim": "nat"}, {"prim": "string"}] });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Bigmap,
        ) {
            Ok((stack, _)) => {
                let expected_big_map = MapValue {
                    is_map: false,
                    key_type: MType::Nat,
                    value_type: MType::String,
                    value: HashMap::new(),
                };
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::Big_map(expected_big_map));
                assert_eq!(stack[0].instruction, Instruction::EMPTY_BIG_MAP);
            }
            Err(err) => panic!("{}", err),
        }
    }

    // FAILING
    #[test]
    #[should_panic(expected = "Arguments for EMPTY_SET instruction cannot be empty")]
    fn empty_set_no_args() {
        let args: Option<&Vec<Value>> = None;
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Set,
        ) {
            Ok(_) => assert!(false),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn empty_set_wrong_args() {
        // arg is not an object
        let arg_value: Value = json!("test");
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Set,
        ) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "Expected a 'serde_json::Value' of type object as argument for EMPTY_SET instruction"
            ),
        };

        // arg object is wrong
        let arg_value: Value = json!({ "string": "nat" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Set,
        ) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "Missing 'prim' field in argument for EMPTY_SET instruction"
            ),
        }
    }

    #[test]
    fn empty_map_wrong_args() {
        // let arg_value: Value = json!({ "args": [{"prim": "nat"}, {"prim": "string"}] });
        // wrong serde object
        let arg_value: Value = json!({ "string": "nat" });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Map,
        ) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "Missing 'args' field in argument for EMPTY_MAP instruction"
            ),
        }

        // wrong args length
        let arg_value: Value =
            json!({ "args": [{"prim": "nat"}, {"prim": "string"}, {"prim": "string"}] });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Map,
        ) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "'args' field for EMPTY_MAP instruction must have 2 elements, got 3"
            ),
        }

        // wrong args values
        let arg_value: Value = json!({ "args": [{"prim": "nat"}, {"test": "string"}] });
        let arg_vec = vec![arg_value];
        let args: Option<&Vec<Value>> = Some(&arg_vec);
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];

        assert!(initial_stack.len() == 2);

        match run(
            initial_stack,
            args,
            &options,
            stack_snapshots,
            EmptyCollection::Map,
        ) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                "Unexpected argument for EMPTY_MAP instruction in 'args' array"
            ),
        }
    }
}
