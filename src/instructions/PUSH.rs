use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, mutez, nat, timestamp, MType, MValue};
use crate::stack::{create_stack_element, Stack, StackElement, StackFuncs};
use serde_json::Value;

pub fn run(stack: Stack, args: Option<&Vec<Value>>, options: &RunOptions) -> Result<Stack, String> {
    // checking the stack is not required
    // checks that the arguments are correct
    let new_stack_element: Result<StackElement, String> = match args {
        None => Err(display_error(ErrorCode::NoArgument(String::from("PUSH")))),
        Some(arg) => {
            // argument must be a vector of 2 elements
            if arg.len() == 2 {
                // extracts the first argument
                let first_arg = &arg[0];
                let element_type_res: Result<MType, String> =
                    if first_arg.is_object() && first_arg.get("prim").is_some() {
                        // checks if the value is a string
                        match first_arg["prim"].as_str() {
                            None => Err(format!(
                                "JSON value for PUSH type argument is not a valid string: {}",
                                first_arg
                            )),
                            Some(str) => {
                                // checks if the type is a valid Michelson type
                                MType::from_string(str)
                            }
                        }
                    } else {
                        Err(format!(
                        "Expected an object with a \"prim\" field in JSON value for PUSH, got {:?}",
                        first_arg
                    ))
                    };
                let element_type = match element_type_res {
                    Ok(el_type) => el_type,
                    Err(err) => panic!("{}", err),
                };
                // extracts the second argument
                let second_arg = &arg[1];
                let element_value_res: Result<(String, String), String> = if second_arg.is_object()
                {
                    if second_arg.get("int").is_some() {
                        // int value
                        match second_arg.get("int").unwrap().as_str() {
                            None => Err(String::from("Expected value for \"int\" property to be a string (at PUSH instruction)")),
                            Some (str) => Ok((String::from("int"), String::from(str)))
                        }
                    } else if second_arg.get("string").is_some() {
                        // string value
                        match second_arg.get("string").unwrap().as_str() {
                            None => Err(String::from("Expected value for \"string\" property to be a string (at PUSH instruction)")),
                            Some (str) => Ok((String::from("string"), String::from(str)))
                        }
                    } else {
                        Err(format!(
                            "JSON value for PUSH value argument is not valid: expected \"int\" or \"string\", but got {}",
                            second_arg
                        ))
                    }
                } else {
                    Err(format!(
                        "Expected an object in JSON value for PUSH, got {:?}",
                        second_arg
                    ))
                };
                let element_value = match element_value_res {
                    Ok(el_value) => el_value,
                    Err(err) => panic!("{}", err),
                };
                // checks that the value matches the type
                match element_type {
                    MType::Int => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<int>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(create_stack_element(MValue::Int(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Nat => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<nat>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(create_stack_element(MValue::Nat(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Mutez => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<mutez>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(create_stack_element(MValue::Mutez(val), Instruction::PUSH))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    MType::Timestamp => {
                        let (val_type, value) = element_value;
                        if val_type == "int" {
                            match value.parse::<timestamp>() {
                                Ok(val) => {
                                    // creates the new stack element
                                    Ok(create_stack_element(
                                        MValue::Timestamp(val),
                                        Instruction::PUSH,
                                    ))
                                }
                                Err(_) => Err(display_error(ErrorCode::InvalidArgument((
                                    String::from("numeric value"),
                                    value,
                                )))),
                            }
                        } else {
                            Err(display_error(ErrorCode::InvalidArgument((
                                String::from("int"),
                                val_type,
                            ))))
                        }
                    }
                    // TODO: handle all the possible cases
                    _ => Err(String::from(
                        "Unhandled patterns to check type/value in PUSH instruction",
                    )),
                }
            } else {
                Err(display_error(ErrorCode::UnexpectedArgsNumber((
                    2,
                    arg.len(),
                ))))
            }
        }
    };
    // pushes the element to the stack
    match new_stack_element {
        Err(err) => Err(err),
        Ok(stack_el) => {
            let new_stack = stack.insert_at(vec![stack_el], options.pos);
            // returns the new stack
            Ok(new_stack)
        }
    }
}
