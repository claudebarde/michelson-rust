use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-CONCAT

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checking the stack will depend on the elements in it
    // but there must be at least one element
    stack.check_depth(options.pos + 1, Instruction::CONCAT)?;
    // stack with 2 strings
    let (concat_res, el_num) = 
        // checks if the stack is correct and does the concatenation according to the stack shape
        if let MValue::String(first_string) = &stack[options.pos].value {
            // the following value on the stack must be a string too
            if let MValue::String(second_string) = &stack[options.pos + 1].value {
                // concatenates the strings
                first_string.to_string().push_str(second_string);
                Ok((MValue::String(first_string.to_string()), 2))
            } else {
                Err(format!(
                    "Expected an element of type string at position {}, but got {}",
                    options.pos + 1,
                    stack[options.pos + 1].value.get_type().to_string()
                ))
            }
        }
        // stack with 2 bytes
        else if let MValue::Bytes(first_bytes) = &stack[options.pos].value {
            // the following value on the stack must be a string of bytes too
            if let MValue::Bytes(second_bytes) = &stack[options.pos + 1].value {
                // concatenates the strings
                first_bytes.to_string().push_str(second_bytes);
                Ok((MValue::String(first_bytes.to_string()), 2))
            } else {
                Err(format!(
                    "Expected an element of type bytes at position {}, but got {}",
                    options.pos + 1,
                    stack[options.pos + 1].value.get_type().to_string()
                ))
            }
        }
        // stack with a list
        else if let MValue::List(list) = &stack[options.pos].value {
            // checks if the list elements are of type string or bytes
            match list.m_type {
                MType::String => Ok(
                    (MValue::String(
                        list.value
                        .clone()
                        .into_iter()
                        .map(|val| match val {
                            MValue::String(str) => str,
                            _ => panic!("Found value of type {} in a list of strings at CONCAT", val.get_type().to_string())
                        })
                        .collect::<Vec<String>>()
                        .join("")
                    ), 1)
                ),
                MType::Bytes => Ok(
                    (MValue::Bytes(
                        list.value
                        .clone()
                        .into_iter()
                        .map(|val| match val {
                            MValue::Bytes(str) => str,
                            _ => panic!("Found value of type {} in a list of bytes at CONCAT", val.get_type().to_string())
                        })
                        .collect::<Vec<String>>()
                        .join("")
                    ), 1)
                ),
                _ => Err(
                    display_error(
                        ErrorCode::InvalidType(
                            (vec![MType::String, MType::Bytes], list.m_type.clone(), Instruction::CONCAT)
                        )
                    )
                )
            }
        } else {
            Err(format!(
                "Expected an element of type string, bytes or list at position {}, but got {}",
                options.pos,
                stack[options.pos].value.get_type().to_string()
            ))
        }?;

    // updates the stack
    let new_stack: Stack = if el_num == 1 {
        // removes the list from the stack
        stack.replace(vec![StackElement::new(concat_res, Instruction::CONCAT)], options.pos)
    } else if el_num == 2 {
        // removes the 2 elements being added from the stack
        let (_, new_stack) = stack.remove_at(options.pos);
        // pushes the new element to the stack
        new_stack.replace(
            vec![StackElement::new(concat_res, Instruction::CONCAT)],
            options.pos,
        )
    } else {
        panic!("Unexpected number of elements to remove for CONCAT, expected 1 or 2, got {}", el_num)
    };
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());
    // returns the new stacka nd stack snapshots
    Ok((new_stack, stack_snapshots))
}
