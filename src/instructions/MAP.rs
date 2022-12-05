// use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use crate::parser;
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-MAP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::MAP;
    // checks the stack
    stack.check_depth(options.pos + 1, this_instruction)?;
    // checks that the value on the stack is correct
    let new_val: MValue = match stack[options.pos].get_val() {
        MValue::List(list) => {
            // checks that all the elements of the list are of the same type
            list.check_elements_type(this_instruction)?;
            // loops through the list and applies instructions
            match args {
                None => Ok(MValue::List(list)), // an empty instruction block is possible, just returning the list
                Some (args_) => {
                    // converts serde_json Value to string to run the code
                    let code_block_json = 
                        args_
                        .into_iter()
                        .map(|el| el.as_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join(",");
                    // iterates through the list, pushes the current element to the stack and applies instructions
                    let mut new_list_els: Vec<MValue> = vec![];
                    let new_list = 
                        list
                        .value
                        .into_iter()
                        .try_fold(
                            (stack.clone(), stack_snapshots), 
                            |(stack, stack_snapshots), list_el| {
                                let stack_to_process = stack.push(list_el, this_instruction);
                                match parser::run(&code_block_json, stack_to_process, stack_snapshots) {
                                    Ok(result) => {
                                        if result.has_failed {
                                            Err(String::from("Block code for instruction MAP could not be parsed"))
                                        } else {
                                            // copies the new element that was created
                                            // TODO: verifies that the element is of the required type by the list
                                            new_list_els.push(stack[0].value.clone());
                                            // returns the new stack and stack snapshots
                                            // TODO: update the stack and stack snapshots
                                            Ok(
                                                (
                                                    result.stack, 
                                                    result.stack_snapshots                                                     
                                                )
                                            )
                                        }
                                    },
                                    Err(err) => Err(err)
                                }
                        });

                    Ok(MValue::Int(69))
                }
            }
        },
        MValue::Map(map) => Ok(MValue::Int(69)),
        _ => Err(format!(
            "Invalid type on the stack at position {} for instruction `{:?}`, expected list or map, but got {:?}",
            options.pos,
            this_instruction,
            stack[options.pos].get_val().get_type()
        )),
    }?;

    Ok((stack, stack_snapshots))
}
