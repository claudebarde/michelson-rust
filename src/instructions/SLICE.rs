use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, OptionValue, MType};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SLICE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 3, Instruction::SLICE)?;
    // the values on the stack must be nat : nat : string/bytes
    let new_val = match (&stack[options.pos].value, &stack[options.pos + 1].value, &stack[options.pos + 2].value) {
        (MValue::Nat(offset), MValue::Nat(length), MValue::String(str_to_slice)) => {
            // checks if offset and length stays in the string boundaries
            if *offset >= str_to_slice.len() as u128 || offset + length > str_to_slice.len() as u128 {
                Ok(MValue::Option(OptionValue::new(None, MType::String)))
            } else {
                Ok(
                    MValue::Option(
                        OptionValue::new(
                            Some(MValue::String(str_to_slice.chars().skip(*offset as usize).take(*length as usize).collect())), 
                            MType::String
                        )
                    )
                )
            }            
        }
        (MValue::Nat(offset), MValue::Nat(length), MValue::Bytes(bytes_to_slice)) => {
            // checks if offset and length stays in the bytes boundaries
            if *offset < bytes_to_slice.len() as u128 || offset + length <= bytes_to_slice.len() as u128 {
                Ok(MValue::Option(OptionValue::new(None, MType::String)))
            } else {
                Ok(
                    MValue::Option(
                        OptionValue::new(
                            Some(MValue::String(bytes_to_slice.chars().skip(*offset as usize).take(*length as usize).collect())), 
                            MType::String
                        )
                    )
                )
            }
        }
        _ => Err(format!(
            "Expected a stack of the following types: `nat : nat : string` or `nat : nat : bytes` for instruction SLICE, but got `{:?} : {:?} : {:?}`", 
            stack[options.pos].value.get_type(), 
            stack[options.pos + 1].value.get_type(), 
            stack[options.pos + 2].value.get_type()
        ))
    }?;

    // removes the current elements from the stack
    let mut new_stack = stack;
    new_stack.drain(options.pos..options.pos + 3);
    // pushes the new element to the stack
    let new_stack = new_stack.insert_at(vec![StackElement::new(new_val, Instruction::SLICE)], options.pos);
    //updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

/**
 * TESTS
 */

 #[cfg(test)]
 mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    #[test]
    fn slice_string_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(0), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::new_string("taquito"), Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 5);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(Some(MValue::String(String::from("taq"))), MType::String))
                );
                assert_eq!(stack[0].instruction, Instruction::SLICE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }

        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(4), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::new_string("taquito"), Instruction::INIT),
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        
        assert!(initial_stack.len() == 5);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(Some(MValue::String(String::from("ito"))), MType::String))
                );
                assert_eq!(stack[0].instruction, Instruction::SLICE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
 }

