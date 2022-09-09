use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MType, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-ADDRESS

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 1, Instruction::ADDRESS) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
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
    use crate::m_types::ContractValue;

    // PASSING
    #[test]
    fn address_success() {
        let new_contract = match MValue::new_address(String::from("KT1BQuSVXWz23iGeXQCrAGR6GcVcqKeE1F7T")) {
            None => panic!("Address is not valid for ADDRESS test"),
            Some (addr) => MValue::Contract(ContractValue::new(addr.get)
        };
        let initial_stack: Stack = vec![
            StackElement::new(), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
                chain_id: String::from("chain_id"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[options.pos].value, MValue::Nat(5));
            }
        }
    }
}
