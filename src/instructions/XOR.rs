use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, nat, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-XOR

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 2, Instruction::XOR) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different numeric types
    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        // boolean XOR
        (MValue::Bool(left), MValue::Bool(right)) => Ok(MValue::Bool(left ^ right)),
        // bitwise XOR
        (MValue::Nat(left), MValue::Nat(right)) => Ok(MValue::Nat(left ^ right)),
        _ => Err(format!(
            "Invalid types for `XOR` expected `bool/bool` or `nat/nat`, but got `{}/{}`",
            stack[options.pos].value.get_type().to_string(),
            stack[options.pos + 1].value.get_type().to_string()
        )),
    }?;
    // removes the 2 elements being compared from the stack
    let (_, new_stack) = stack.remove_at(options.pos);
    // pushes the new element to the stack
    let new_stack = new_stack.replace(
        vec![StackElement::new(new_val, Instruction::XOR)],
        options.pos,
    );
    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    // returns the stack
    Ok((new_stack, stack_snapshots))
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;

    // PASSING
    // Tests XOR with 2 bools
    #[test]
    fn or_bool_bool() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Bool(true), Instruction::INIT),
            StackElement::new(MValue::Bool(false), Instruction::INIT),
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
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Bool(false));
                assert_eq!(stack[0].instruction, Instruction::XOR);
                assert_eq!(stack[1].value, MValue::Bool(false));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // Tests XOR with 2 nats
    #[test]
    fn or_nat_nat() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(2), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::Nat(44), Instruction::INIT),
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
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 2);
                assert_eq!(stack[0].value, MValue::Nat(1));
                assert_eq!(stack[0].instruction, Instruction::XOR);
                assert_eq!(stack[1].value, MValue::Nat(44));
                assert_eq!(stack[1].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    // wrong types
    #[test]
    #[should_panic(
        expected = "Invalid types for `XOR` expected `bool/bool` or `nat/nat`, but got `string/nat`"
    )]
    fn and_wrong_types() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("test")), Instruction::INIT),
            StackElement::new(MValue::Nat(3), Instruction::INIT),
            StackElement::new(MValue::Nat(44), Instruction::INIT),
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
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 3);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    // wrong stack depth
    #[test]
    #[should_panic(
        expected = "Unexpected stack length, expected a length of 2 for instruction XOR, got 1"
    )]
    fn and_wrong_stack_depth() -> () {
        let initial_stack: Stack = vec![StackElement::new(MValue::Nat(3), Instruction::INIT)];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext {
                amount: 0,
                sender: String::from("test_sender"),
                source: String::from("test_source"),
                self_address: String::from("KT1L7GvUxZH5tfa6cgZKnH6vpp2uVxnFVHKu"),
                balance: 50_000_000,
                level: 11,
            },
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
