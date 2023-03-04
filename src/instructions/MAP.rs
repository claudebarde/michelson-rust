// use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, CollectionValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
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
            // checks that all the elements of the list are of the same type
            list.check_elements_type(this_instruction)?;
            // removes the list from the stack
            let (_, stack_without_list) = stack.remove_at(options.pos);
            // loops through the list and applies instructions
            match args {
                None => Ok((stack, stack_snapshots)), // an empty instruction block is possible, just returning the current stack
                Some (args_) => {
                    if args_.len() == 1 {
                        // converts serde_json Value to string to run the code
                        let code_block_json = serde_json::to_string(&args_[0]).unwrap();
                        // iterates through the list, pushes the current element to the stack and applies instructions
                        let mut new_list_els: Vec<MValue> = vec![];
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
                                                    new_list_els.push(new_el.value);
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
    
                        // creates the new list
                        let new_list = MValue::List(CollectionValue { m_type: list.m_type, value: Box::new(new_list_els)});
                        // pushes the new list onto the stack
                        let new_stack = new_stack.push(new_list, this_instruction);
    
                        Ok((new_stack, stack_snapshots))
                    } else {
                        Err("Argument for MAP is an empty array, expected an array with 1 element".to_string())
                    }
                }
            }
        },
        MValue::Map(map) => Ok((stack, stack_snapshots)),
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
    use crate::m_types::{MType};
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
}
