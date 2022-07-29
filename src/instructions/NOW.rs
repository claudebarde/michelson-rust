use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use std::time::{SystemTime, UNIX_EPOCH};

// https://tezos.gitlab.io/michelson-reference/#instr-NOW

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // no stack check required
    // updates the stack
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let new_el = StackElement::new(
        MValue::Timestamp(duration.as_secs() as usize),
        Instruction::NOW,
    );
    let new_stack = stack.insert_at(vec![new_el], options.pos);
    // updates the stack snapshots
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
    fn now_success() {
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
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 2);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Timestamp(duration.as_secs() as usize)
                );
                assert_eq!(stack[0].instruction, Instruction::NOW);
                assert_eq!(stack[1].value, MValue::Int(22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
}
