use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-SOURCE

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos, Instruction::SOURCE) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // checks if the provided address is valid
    match MValue::new_address(options.context.source.clone()) {
        None => Err(format!(
            "Provided address for SOURCE is not a valid address: {:?}",
            options.context.source
        )),
        Some(addr) => {
            // updates the stack
            let new_el = StackElement::new(addr, Instruction::SOURCE);
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
    fn source_success() {
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
                source: String::from("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"),
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
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Address(String::from("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"))
                );
                assert_eq!(stack[0].instruction, Instruction::SOURCE);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    #[should_panic(
        expected = "Provided address for SOURCE is not a valid address: \"test_source\""
    )]
    fn source_invalid_address() {
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
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
                chain_id: String::from("chain_id"),
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
