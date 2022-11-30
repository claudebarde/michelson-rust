use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-ADDRESS

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 1, Instruction::ADDRESS)?;
    // verifies that the value at options.pos is a contract
    let new_val_res: Result<MValue, String> = match &stack[options.pos].value {
        MValue::Contract(contract) => {
            let address = contract.get_address();
            Ok(MValue::Address(address))
        }
        _ => Err(display_error(ErrorCode::WrongType((
            String::from("contract"),
            stack[options.pos].value.get_type().to_string(),
            Instruction::ADDRESS,
        )))),
    };

    match new_val_res {
        Err(err) => Err(err),
        Ok(new_val) => {
            let new_stack = stack.replace(
                vec![StackElement::new(new_val, Instruction::ADDRESS)],
                options.pos,
            );
            stack_snapshots.push(new_stack.clone());
            Ok((new_stack, stack_snapshots))
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
    use crate::m_types::{ContractValue, MType};

    // PASSING
    #[test]
    fn address_success() {
        let expected_address = "KT1BQuSVXWz23iGeXQCrAGR6GcVcqKeE1F7T";
        let new_contract = match MValue::new_address(String::from(expected_address)) {
            None => panic!("Address is not valid for ADDRESS test"),
            Some(addr) => match addr {
                MValue::Address(addr) => MValue::Contract(ContractValue::new(addr, MType::Int)),
                _ => panic!("Value returned by `MValue::new_address` is not an address"),
            },
        };
        let initial_stack: Stack = vec![
            StackElement::new(new_contract, Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(
                    stack[0].value,
                    MValue::Address(expected_address.to_string())
                );
                assert_eq!(stack[0].instruction, Instruction::ADDRESS);
                assert_eq!(stack[1].value, MValue::Int(6));
                assert_eq!(stack[1].instruction, Instruction::INIT)
            }
        }
    }

    // FAILING
    #[test]
    fn address_wrong_type() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(50_000_000), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                format!("Wrong type, expected `contract` for instruction ADDRESS, got `mutez`")
            ),
        }
    }

    #[test]
    fn address_empty_stack() {
        let initial_stack: Stack = vec![];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 0);

        match run(initial_stack, &options, stack_snapshots) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(
                err,
                format!("Unexpected stack length, expected a length of 1 for instruction ADDRESS, got 0")
            ),
        }
    }
}
