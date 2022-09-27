use crate::instructions::{Instruction, RunOptions};
use crate::m_types::MValue;
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};
use sha3::{Digest, Keccak256};

// https://tezos.gitlab.io/michelson-reference/#instr-KECCAK

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    stack.check_depth(options.pos + 1, Instruction::KECCAK)?;
    // KECCAK can be used only with bytes
    let keccak_res: Result<MValue, String> = match &stack[options.pos].value {
        MValue::Bytes(val) => {
            let mut hasher = Keccak256::new();
            hasher.update(val.as_bytes());
            let result = hasher.finalize();
            Ok(MValue::Bytes(format!("{:x}", result)))
        }
        _ => Err(format!(
            "Expected value of type bytes for KECCAK, but got {}",
            &stack[options.pos].value.to_string()
        )),
    };

    match keccak_res {
        Ok(mval) => {
            // removes the element affected by KECCAK
            // pushes the hash to the stack
            let new_stack = stack.replace(
                vec![StackElement::new(mval, Instruction::KECCAK)],
                options.pos,
            );
            // updates the stack snapshots
            stack_snapshots.push(new_stack.clone());
            // returns the new stack
            Ok((new_stack, stack_snapshots))
        }
        Err(err) => Err(err),
    }
}

/**
 * TESTS
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    // test for keccak 256 hashing
    #[test]
    fn keccak_success() {
        let initial_stack: Stack = vec![
            StackElement::new(
                MValue::Bytes(String::from("7461717569746f")),
                Instruction::INIT,
            ),
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => {
                println!("{}", err);
                assert!(false)
            }
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Bytes(String::from(
                        "62670a3744b0147d3b94d7f349b72df593039540e3612356d69a21c68d51ebce"
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::KECCAK);
                assert_eq!(stack[1].value, MValue::Int(5));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Nat(6));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }
}
