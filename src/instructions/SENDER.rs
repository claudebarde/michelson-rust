use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SENDER

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos, Instruction::SENDER) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // checks if the provided address is valid
    match MValue::new_address(options.context.sender.clone()) {
        None => Err(format!(
            "Provided address for SENDER is not a valid address: {:?}",
            options.context.sender
        )),
        Some(addr) => {
            // updates the stack
            let new_el = StackElement::new(addr, Instruction::SENDER);
            let new_stack = stack.insert_at(vec![new_el], options.pos);
            // updates the stack snapshots
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

    #[test]
    fn sender_success() {
        // should push the address to the stack
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Address(String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL"))
                );
                assert_eq!(stack[0].instruction, Instruction::SENDER);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    #[should_panic(
        expected = "Provided address for SENDER is not a valid address: \"test_sender\""
    )]
    fn sender_wrong_address() {
        // should push the address to the stack
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
