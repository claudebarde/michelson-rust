use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackFuncs, StackSnapshots};
use serde_json::Value;

// https://tezos.gitlab.io/michelson-reference/#instr-DUP

pub fn run(
    stack: Stack,
    args: Option<&Vec<Value>>,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::DUP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to duplicate
    let el_to_dup_pos_res: Result<usize, String> = match args {
        None => Ok(options.pos + 1),
        Some(arg) => {
            if arg.len() == 1 {
                let arg = &arg[0];
                if arg.is_object() && arg.get("int").is_some() {
                    // gets the int value that will be stored as a string
                    match arg.get("int").unwrap().as_str() {
                        None => Err(String::from("Expected a string in JSON value for DUP")),
                        Some(str) =>
                        // parse the string into a number
                        {
                            match str.parse::<usize>() {
                                Err(_) => Err(format!(
                                    "JSON value for DUP argument is not a valid number: {}",
                                    str
                                )),
                                Ok(val) => {
                                    // DUP 0 is a noop
                                    if val == 0 {
                                        Err(format!(
                                            "{:?}",
                                            ErrorCode::Noop(String::from("DUP 0 is a noop"))
                                        ))
                                    } else {
                                        // checks that the stack is deep enough for the DUP parameter
                                        match stack.check_depth(options.pos + val, Instruction::DUP)
                                        {
                                            Err(err) => Err(err),
                                            Ok(_) => Ok(options.pos + val),
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Err(format!("Unexpected format for DUP argument: {:?}", arg))
                }
            } else {
                Err(format!(
                    "{:?}",
                    display_error(ErrorCode::UnexpectedArgsNumber((1, arg.len())))
                ))
            }
        }
    };

    match el_to_dup_pos_res {
        Err(err) => Err(err), // forwards the error
        Ok(el_to_dup_pos) => {
            // duplicates the element at el_to_dup_pos
            let dupped_el = stack[el_to_dup_pos - 1].clone();
            // checks if element is not a ticket
            if let MValue::Ticket(_) = dupped_el.value {
                Err(String::from("Tickets cannot be duplicated"))
            } else {
                // adds the element to the top of the stack
                let new_stack = stack.insert_at(vec![dupped_el], 0);
                // updates the stack snapshots
                stack_snapshots.push(new_stack.clone());
                // returns the new stack
                Ok((new_stack, stack_snapshots))
            }
        }
    }
}

/*
    TESTS
*/
