use crate::errors::{display_error, ErrorCode};
use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{int, MValue};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-MUL

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    // checks the stack
    match stack.check_depth(options.pos + 2, Instruction::MUL) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    };
    // pattern matches the different numeric types
    let new_val: MValue = match (
        stack[options.pos].get_val(),
        stack[options.pos + 1].get_val(),
    ) {
        (MValue::Int(left), MValue::Int(right)) => MValue::Int(left * right),
        (MValue::Int(left), MValue::Nat(right)) => {
            if MValue::Nat(right).check_nat() {
                MValue::Int(left * right as int)
            } else {
                panic!("{}", display_error(ErrorCode::InvalidNat(right)))
            }
        } // int
        (MValue::Nat(left), MValue::Int(right)) => {
            if MValue::Nat(left).check_nat() {
                MValue::Int(left as int * right)
            } else {
                panic!("{}", display_error(ErrorCode::InvalidNat(left)))
            }
        } // int
        (MValue::Nat(left), MValue::Nat(right)) => {
            if MValue::Nat(left).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(left)))
            } else if MValue::Nat(right).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(right)))
            } else {
                MValue::Nat(left * right)
            }
        } // nat
        (MValue::Mutez(left), MValue::Nat(right)) => {
            if MValue::Mutez(left).check_mutez() == false {
                panic!("{}", display_error(ErrorCode::InvalidMutez(left)))
            } else if MValue::Nat(right).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(right)))
            } else {
                MValue::Mutez(left * right)
            }
        } // mutez
        (MValue::Nat(left), MValue::Mutez(right)) => {
            if MValue::Mutez(right).check_mutez() == false {
                panic!("{}", display_error(ErrorCode::InvalidMutez(right)))
            } else if MValue::Nat(left).check_nat() == false {
                panic!("{}", display_error(ErrorCode::InvalidNat(left)))
            } else {
                MValue::Mutez(left * right)
            }
        } // mutez
        (m_val_left, m_val_right) => panic!(
            "Cannot multiply together values of type {} and {}",
            m_val_left.to_string(),
            m_val_right.to_string()
        ),
    };
    // updates the stack by removing the 2 elements
    let (_, new_stack) = stack.remove_at(options.pos);
    let (_, new_stack) = new_stack.remove_at(options.pos);
    // pushes the new value to the top of the stack
    let mut stack_head = vec![StackElement::new(new_val, Instruction::MUL)];
    let mut stack_tail = new_stack;
    stack_head.append(&mut stack_tail);
    // updates the stack snapshots
    stack_snapshots.push(stack_head.clone());
    // returns the stack
    Ok((stack_head, stack_snapshots))
}

/*
    TESTS
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::MValue;

    // PASSING TESTS
    // Tests MUL with 2 ints
    #[test]
    fn mul_int_int() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Int(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Int(30));
            }
        }
    }

    // Tests MUL with 1 int and 1 nat
    #[test]
    fn mul_int_nat() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Int(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Int(30));
            }
        }
    }

    // Tests MUL with 2 nats
    #[test]
    fn mul_nat_nat() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(5), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Nat(30));
            }
        }
    }

    // Tests MUL with 1 mutez and 1 nat
    #[test]
    fn mul_mutez_nat() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(5_000_000), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Mutez(30_000_000));
            }
        }
    }

    // Tests MUL with 1 nat and 1 mutez
    #[test]
    fn mul_nat_mutez() -> () {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(6), Instruction::INIT),
            StackElement::new(MValue::Mutez(5_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 1);
                assert_eq!(stack[0].value, MValue::Mutez(30_000_000));
            }
        }
    }

    // FAILING TESTS
    // MUL with strings
    #[test]
    #[should_panic(expected = "Cannot multiply together values of type string and nat")]
    fn mul_string_nat() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("5")), Instruction::INIT),
            StackElement::new(MValue::Nat(6), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }

    #[test]
    #[should_panic(expected = "Cannot multiply together values of type mutez and mutez")]
    fn mul_mutez_mutez() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Mutez(5_000_000), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => panic!("{}", err),
            Ok(_) => assert!(false),
        }
    }
}
