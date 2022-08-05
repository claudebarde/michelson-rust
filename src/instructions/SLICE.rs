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
            // checks if offset and length stay in the string boundaries
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
            // checks if offset and length stay in the bytes boundaries
            if *offset * 2 >= bytes_to_slice.len() as u128 || offset * 2 + length > bytes_to_slice.len() as u128 {
                Ok(MValue::Option(OptionValue::new(None, MType::Bytes)))
            } else {
                Ok(
                    MValue::Option(
                        OptionValue::new(
                            Some(MValue::Bytes(bytes_to_slice.chars().skip(*offset as usize * 2).take(*length as usize * 2).collect())), 
                            MType::Bytes
                        )
                    )
                )
            }
        }
        _ => Err(format!(
            "Expected a stack of the following types: `nat : nat : string` or `nat : nat : bytes` for instruction SLICE, but got `{} : {} : {}`", 
            stack[options.pos].value.get_type().to_string(), 
            stack[options.pos + 1].value.get_type().to_string(), 
            stack[options.pos + 2].value.get_type().to_string()
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

    // PASSING
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

    #[test]
    fn slice_bytes_success() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(0), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::new_bytes("7461717569746f"), Instruction::INIT),
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
                    MValue::Option(OptionValue::new(Some(MValue::Bytes(String::from("746171"))), MType::Bytes))
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
            StackElement::new(MValue::new_bytes("7461717569746f"), Instruction::INIT),
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
                    MValue::Option(OptionValue::new(Some(MValue::Bytes(String::from("69746f"))), MType::Bytes))
                );
                assert_eq!(stack[0].instruction, Instruction::SLICE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn slice_outofbound_offset() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(7), Instruction::INIT),
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
                    MValue::Option(OptionValue::new(None, MType::String))
                );
                assert_eq!(stack[0].instruction, Instruction::SLICE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn slice_outofbound_length() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
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
                    MValue::Option(OptionValue::new(None, MType::String))
                );
                assert_eq!(stack[0].instruction, Instruction::SLICE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    #[test]
    fn slice_wrong_stack_depth() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::new_bytes("7461717569746f"), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(err, String::from("Unexpected stack length, expected a length of 3 for instruction SLICE, got 2")),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    fn slice_wrong_stack_types() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(0), Instruction::INIT),
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
            Err(err) => assert_eq!(
                err, 
                String::from("Expected a stack of the following types: `nat : nat : string` or `nat : nat : bytes` for instruction SLICE, but got `int : nat : string`")
            ),
            Ok(_) => assert!(false),
        }
    }
 }

