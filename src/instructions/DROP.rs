use crate::errors::{display_error, ErrorCode};
use crate::instructions::Instruction;
use crate::instructions::RunOptions;
use crate::stack::{Stack, StackFuncs};
use serde_json::Value;

pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checks the stack
    match stack.check_depth(1, Instruction::DROP) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // calculates the position of the element to drop
    let el_to_drop_pos = match args {
        None => options.pos,
        Some(arg) => {
            if arg.len() == 1 {
                let arg = &arg[0];
                if arg.is_object() && arg.get("int").is_some() {
                    // gets the int value that will be stored as a string
                    match arg.get("int").unwrap().as_str() {
                        None => panic!("Expected a String in JSON value for DROP"),
                        Some(str) =>
                        // parse the string into a number
                        {
                            match str.parse::<usize>() {
                                Err(_) => panic!(
                                    "JSON value for DROP argument is not a valid number: {}",
                                    str
                                ),
                                Ok(val) => {
                                    // DROP 0 is a noop
                                    if val == 0 {
                                        panic!("{:?}", ErrorCode::Noop(String::from("DROP 0")))
                                    } else {
                                        options.pos + val
                                    }
                                }
                            }
                        }
                    }
                } else {
                    panic!("Unexpected format for DROP argument: {:?}", arg)
                }
            } else {
                panic!(
                    "{:?}",
                    display_error(ErrorCode::UnexpectedArgsNumber((1, arg.len())))
                )
            }
        }
    };

    // drops the element at position - 1
    let new_stack = stack
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i > el_to_drop_pos - 1)
        .map(|(_, e)| e)
        .collect();
    // returns the new stack
    Ok(new_stack)
}
