// use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, CollectionValue, MType, PairValue};
use crate::stack::{Stack, StackFuncs, StackSnapshots};
use crate::parser;
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-MAP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::MAP;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // checks that the value on the stack is correct
    let (new_stack, stack_snapshots) = match stack[options.pos].get_val() {
        MValue::List(list) => {
            if list.value.len() != 0 {
                // checks that all the elements of the list are of the same type
                list.check_elements_type(this_instruction)?;
                // removes the list from the stack
                let (_, stack_without_list) = stack.remove_at(options.pos);
                // loops through the list and applies instructions
                match args {
                    None => Ok((stack, stack_snapshots)), // an empty instruction block is possible, just returning the current stack
                    Some (args_) => {
                        if args_.len() == 1 {
                            let list_len = list.value.len();
                            // converts serde_json Value to string to run the code
                            let code_block_json = serde_json::to_string(&args_[0]).unwrap();
                            // iterates through the list, pushes the current element to the stack and applies instructions
                            let mut new_list: Vec<MValue> = vec![];
                            let (new_stack, stack_snapshots) = 
                            list
                            .value
                            .into_iter()
                            .try_fold(
                                (stack_without_list, stack_snapshots), 
                                |(stack, stack_snapshots), list_el| {
                                    let stack_to_process = stack.push(list_el, this_instruction);
                                    println!("{:?}", stack_to_process);
                                    match parser::run(&code_block_json, stack_to_process, stack_snapshots) {
                                        Ok(result) => {
                                            if result.has_failed {
                                                Err(String::from("Block code for instruction MAP could not be parsed"))
                                            } else {
                                                // removes the elements that was created from the stack
                                                let (new_el, truncated_stack) = result.stack.remove_at(0);
                                                // saves the new element in the list of new elements
                                                new_list.push(new_el.value);
                                                // returns the new stack and stack snapshots
                                                Ok(
                                                    (
                                                        truncated_stack, 
                                                        result.stack_snapshots                                                     
                                                    )
                                                )
                                            }
                                        },
                                        Err(err) => Err(err)
                                    }
                                })?;                                
                                // checks that the new list length is the same as the original list
                                if list_len == new_list.len() {
                                    // checks that all the elements in the new list are of the same type
                                    // and figures out the type of the elements of the new list
                                    let list_el_type = MType::check_vec_els_type(&new_list, this_instruction)?;
                                    // creates the new list
                                    let collection = CollectionValue { 
                                        m_type: list_el_type, 
                                        value: Box::new(new_list)
                                    };
                                    let new_list = MValue::List(collection);
                                    // pushes the new list onto the stack
                                    let new_stack = new_stack.push(new_list, this_instruction);
                                    
                                    Ok((new_stack, stack_snapshots))
                                } else {
                                    Err(
                                        format!("List generated by MAP instruction has a different length, expected a length of {}, but got {}", list_len, new_list.len())
                                    )
                                }                        
                            } else {
                                Err("Argument for MAP is an empty array, expected an array with 1 element".to_string())
                            }
                        }
                    }
                } else {
                    // returns now if there are no element in the list
                    Ok((stack, stack_snapshots))
                }
            },
            MValue::Map(map) => {
                match map.size() {
                    Ok(map_size) => {
                        if map_size != 0 {
                            // removes the map from the stack
                            let (_, stack_without_map) = stack.remove_at(options.pos);
                            // loops through the list and applies instructions
                            match args {
                                None => Ok((stack, stack_snapshots)), // an empty instruction block is possible, just returning the current stack
                                Some (args_) => {
                                    if args_.len() == 1 {
                                        // converts serde_json Value to string to run the code
                                        let code_block_json = serde_json::to_string(&args_[0]).unwrap();
                                        // iterates through the map, pushes the key and value as a pair to the stack and applies instructions
                                        let mut new_map_els: Vec<(MValue, MValue)> = vec![];
                                        let mut map_key_type: Option<MType> = None;
                                        let (new_stack, stack_snapshots) = 
                                            map
                                            .value
                                            .into_iter()
                                            .try_fold(
                                                (stack_without_map, stack_snapshots), 
                                                |(stack, stack_snapshots), pair| {
                                                    let (key, value) = pair;
                                                    // checks that the type is the same
                                                    // TODO: it might be wiser to check that the key type is consistent throughout the loop
                                                    map_key_type = Some(key.get_type());
                                                    // creates the pair to be pushed to the stack
                                                    let map_el = MValue::Pair(PairValue::new(key.clone(), value));
                                                    let stack_to_process = stack.push(map_el, this_instruction);
                                                    println!("{:?}", stack_to_process);
                                                    match parser::run(&code_block_json, stack_to_process, stack_snapshots) {
                                                        Ok(result) => {
                                                            if result.has_failed {
                                                                Err(String::from("Block code for instruction MAP could not be parsed"))
                                                            } else {
                                                                // removes the elements that was created from the stack
                                                                let (new_el, truncated_stack) = result.stack.remove_at(0);
                                                                // saves the new element in the list of new elements
                                                                new_map_els.push((key, new_el.value));
                                                                // returns the new stack and stack snapshots
                                                                Ok(
                                                                    (
                                                                        truncated_stack, 
                                                                        result.stack_snapshots                                                     
                                                                    )
                                                                )
                                                            }
                                                        },
                                                        Err(err) => Err(err)
                                                    }
                                                })?;                                
                                            // checks that the length of the vector with the returned elements is the same as the original map
                                            if map_size == new_map_els.len() {
                                                match map_key_type {
                                                    None => Err("No key type for the map created by MAP instruction was generated".to_string()),
                                                    Some(key_type) => {
                                                        // checks that all the elements in the new list are of the same type
                                                        // and figures out the type of the elements of the new list
                                                        let map_value_type = MType::check_vec_els_type(
                                                            &new_map_els.clone().into_iter().map(|el| el.1).collect(), 
                                                            this_instruction
                                                        )?;
                                                        // creates the new map
                                                        let new_map = MValue::new_map(key_type, map_value_type, new_map_els);
                                                        // pushes the new list onto the stack
                                                        let new_stack = new_stack.push(new_map, this_instruction);
                                                        
                                                        Ok((new_stack, stack_snapshots))
                                                    }
                                                }
                                            } else {
                                                Err(
                                                    format!("Map generated by MAP instruction has a different length, expected a length of {}, but got {}", map_size, new_map_els.len())
                                                )
                                            }
                                    } else {
                                        Err("Argument for MAP is an empty array, expected an array with 1 element".to_string())
                                    }
                                }}
                        } else {
                            // the map is empty
                            Ok((stack, stack_snapshots))
                        }
                    }
                    Err(err) => Err(format!("Error while reading the size of a map at MAP instruction: {}", err))
                }
            },
            _ => Err(format!(
            "Invalid type on the stack at position {} for instruction `{:?}`, expected list or map, but got {:?}",
            options.pos,
            this_instruction,
            stack[options.pos].get_val().get_type()
        )),
    }?;

    Ok((new_stack, stack_snapshots))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{MType, PairValue};
    use crate::stack::StackElement;
    use serde_json::json;

    // PASSING
    #[test]
    fn map_success_list_of_nats() {
        let initial_list = vec![MValue::Nat(2), MValue::Nat(3), MValue::Nat(4), MValue::Nat(5)];
        let initial_stack: Stack = vec![
            StackElement::new(MValue::List(CollectionValue { m_type: MType::Nat, value: Box::new(initial_list) }), Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        let args = vec![
            json!([{ "prim": "PUSH", "args": [{"prim":"nat"}, {"int": "3"}] }, { "prim": "MUL" }])
        ];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, Some(&args), &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::List(
                    CollectionValue { 
                        m_type: MType::Nat, 
                        value: Box::new(vec![MValue::Nat(6), MValue::Nat(9), MValue::Nat(12), MValue::Nat(15)]) 
                    }
                ));
                assert_eq!(stack[0].instruction, Instruction::MAP);
                assert_eq!(stack[1].value, MValue::Int(-22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn map_success_list_of_pairs() {
        let initial_list = vec![
            MValue::Pair(PairValue::new(MValue::Nat(3), MValue::String(String::from("pair_1")))),
            MValue::Pair(PairValue::new(MValue::Nat(5), MValue::String(String::from("pair_2")))),
            MValue::Pair(PairValue::new(MValue::Nat(7), MValue::String(String::from("pair_3")))),
            MValue::Pair(PairValue::new(MValue::Nat(9), MValue::String(String::from("pair_4"))))
        ];
        let initial_stack: Stack = vec![
            StackElement::new(MValue::List(CollectionValue { 
                m_type: MType::Pair(Box::new((MType::Nat, MType::String))), 
                value: Box::new(initial_list) }), 
                Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        let args = vec![
            json!([
                { "prim": "UNPAIR" },
                { "prim": "PUSH", "args": [{"prim": "nat"}, {"int": "2"}] },
                { "prim": "ADD" },
                { "prim": "SWAP" },
                { "prim": "PUSH", "args": [{"prim": "string"}, {"string": "_good"}] },
                { "prim": "SWAP" },
                { "prim": "CONCAT" },
                { "prim": "PAIR" }
            ])
        ];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, Some(&args), &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, MValue::List(
                    CollectionValue { 
                        m_type: MType::Pair(Box::new((MType::String, MType::Nat))), 
                        value: Box::new(vec![
                            MValue::Pair(PairValue::new(MValue::String(String::from("pair_1_good")), MValue::Nat(5))),
                            MValue::Pair(PairValue::new(MValue::String(String::from("pair_2_good")), MValue::Nat(7))),
                            MValue::Pair(PairValue::new(MValue::String(String::from("pair_3_good")), MValue::Nat(9))),
                            MValue::Pair(PairValue::new(MValue::String(String::from("pair_4_good")), MValue::Nat(11)))
                        ]) 
                    }
                ));
                assert_eq!(stack[0].instruction, Instruction::MAP);
                assert_eq!(stack[1].value, MValue::Int(-22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn map_success_with_simple_map() {
        let initial_map = MValue::new_map(
            MType::String, 
            MType::Nat, 
            vec![
                (MValue::String(String::from("tezos")), MValue::Nat(3)),
                (MValue::String(String::from("taquito")), MValue::Nat(4)),
                (MValue::String(String::from("tacos")), MValue::Nat(5)),
                (MValue::String(String::from("cardano_lol")), MValue::Nat(6))
            ]
        );
        let initial_stack: Stack = vec![
            StackElement::new(initial_map, Instruction::INIT),
            StackElement::new(MValue::Int(33), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        let args = vec![
            json!([{ "prim": "CDR" }, { "prim": "PUSH", "args": [{"prim":"nat"}, {"int": "3"}] }, { "prim": "MUL" }])
        ];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, Some(&args), &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                let output_map = MValue::new_map(
                    MType::String, 
                    MType::Nat, 
                    vec![
                        (MValue::String(String::from("tezos")), MValue::Nat(9)),
                        (MValue::String(String::from("taquito")), MValue::Nat(12)),
                        (MValue::String(String::from("tacos")), MValue::Nat(15)),
                        (MValue::String(String::from("cardano_lol")), MValue::Nat(18))
                    ]
                );
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, output_map);
                assert_eq!(stack[0].instruction, Instruction::MAP);
                assert_eq!(stack[1].value, MValue::Int(33));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn map_success_with_complex_map() {
        let initial_map = MValue::new_map(
            MType::String, 
            MType::Pair(Box::new((MType::Int, MType::Int))), 
            vec![
                (MValue::String(String::from("tezos")), MValue::Pair(PairValue::new(MValue::Int(5), MValue::Int(6)))),
                (MValue::String(String::from("taquito")), MValue::Pair(PairValue::new(MValue::Int(7), MValue::Int(8)))),
                (MValue::String(String::from("tacos")), MValue::Pair(PairValue::new(MValue::Int(9), MValue::Int(10)))),
                (MValue::String(String::from("cardano_lol")), MValue::Pair(PairValue::new(MValue::Int(11), MValue::Int(12))))
            ]
        );
        let initial_stack: Stack = vec![
            StackElement::new(initial_map, Instruction::INIT),
            StackElement::new(MValue::Int(33), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        let args = vec![
            json!([
                { "prim": "CDR" }, 
                { "prim": "UNPAIR" }, 
                { "prim": "ADD" },
                { "prim": "ABS" },
                { "prim": "PUSH", "args": [{"prim":"string"}, {"string": "tillwebezos"}] },
                { "prim": "PAIR" }
            ])
        ];

        assert!(initial_stack.len() == 3);

        match run(initial_stack, Some(&args), &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                let output_map = MValue::new_map(
                    MType::String, 
                    MType::Pair(Box::new((MType::String, MType::Nat))), 
                    vec![
                        (MValue::String(String::from("tezos")), MValue::Pair(PairValue::new(MValue::String(String::from("tillwebezos")), MValue::Nat(11)))),
                        (MValue::String(String::from("taquito")), MValue::Pair(PairValue::new(MValue::String(String::from("tillwebezos")), MValue::Nat(15)))),
                        (MValue::String(String::from("tacos")), MValue::Pair(PairValue::new(MValue::String(String::from("tillwebezos")), MValue::Nat(19)))),
                        (MValue::String(String::from("cardano_lol")), MValue::Pair(PairValue::new(MValue::String(String::from("tillwebezos")), MValue::Nat(23))))
                    ]
                );
                assert!(stack.len() == 3);
                assert_eq!(stack[0].value, output_map);
                assert_eq!(stack[0].instruction, Instruction::MAP);
                assert_eq!(stack[1].value, MValue::Int(33));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
}
