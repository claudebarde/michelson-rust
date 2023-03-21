use crate::instructions::{Instruction, RunOptions};
use crate::m_types::{MValue, OptionValue, MType};
use crate::stack::{Stack, StackElement, StackFuncs, StackSnapshots};

// https://tezos.gitlab.io/michelson-reference/#instr-TICKET

pub fn run(
    stack: Stack,
    options: &RunOptions,
    mut stack_snapshots: StackSnapshots,
) -> Result<(Stack, StackSnapshots), String> {
    let this_instruction = Instruction::TICKET;
    // checks the stack
    stack.check_depth(options.pos + 2, this_instruction)?;
    // elements on the stack must be a Michelson value and a nat value
    let new_stack: Stack = 
        match (stack[options.pos].get_val(), stack[options.pos + 1].get_val()) {
            (value, MValue::Nat(amount)) => {
                let value_type = value.clone().get_type();
                // creates the new ticket
                let new_ticket = 
                    if amount == 0 {
                        Ok(None)
                    } else {
                        match MValue::new_ticket(value, amount, options.context.self_address.clone()) {
                            Ok(ticket) => Ok(Some(ticket)),
                            Err(err) => Err(err)
                        }
                        
                    }?;
                // creates the optional value to wrap the ticket
                let option_ticket = MValue::Option(
                    OptionValue::new(
                        new_ticket, 
                        MType::Ticket(Box::new((value_type, MType::Nat, MType::Address)))
                    )
                );
                // creates the new stack element
                let new_stack_el = StackElement::new(option_ticket, this_instruction);
                // removes the value element from the stack
                let (_, new_stack) = stack.remove_at(options.pos);
                // replaces the amount element with the new stack element 
                let new_stack = new_stack.replace(vec![new_stack_el], options.pos);

                Ok(new_stack)
            },
            (value, should_be_nat) => {
                Err(
                    format!("Wrong type for instruction {:?}, expected a stack of type ['a : nat : S'] but got [{} : {} : S']",
                        this_instruction,
                        value.get_type().to_string(),
                        should_be_nat.get_type().to_string()
                    )
                )
            }
        }?;

    // updates the stack snapshots
    stack_snapshots.push(new_stack.clone());

    Ok((new_stack, stack_snapshots))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::RunOptionsContext;
    use crate::m_types::{Ticket, PairValue};

    // PASSING
    #[test]
    fn create_ticket_success_1() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::Nat(10), Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Ticket(Box::new(
                            Ticket { 
                                value: MValue::String(String::from("tezos")),
                                amount: 10,
                                ticketer: options.context.self_address.clone()
                            }
                        ))), 
                        MType::Ticket(Box::new((MType::String, MType::Nat, MType::Address)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::TICKET);
                assert_eq!(stack[1].value, MValue::Int(-22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn create_ticket_success_2() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Pair(PairValue::new(MValue::Nat(55), MValue::String(String::from("taquito")))), Instruction::INIT),
            StackElement::new(MValue::Nat(22), Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        Some(MValue::Ticket(Box::new(
                            Ticket { 
                                value: MValue::Pair(PairValue { 
                                    m_type: (MType::Nat, MType::String),
                                    value: Box::new((MValue::Nat(55), MValue::String(String::from("taquito"))))
                                }),
                                amount: 22,
                                ticketer: options.context.self_address.clone()
                            }
                        ))), 
                        MType::Ticket(Box::new((MType::Pair(Box::new((MType::Nat, MType::String))), MType::Nat, MType::Address)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::TICKET);
                assert_eq!(stack[1].value, MValue::Int(-22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    #[test]
    fn create_ticket_success_3() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::Nat(0), Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(_) => assert!(false),
            Ok((stack, _)) => {
                assert!(stack.len() == 3);
                assert_eq!(
                    stack[0].value,
                    MValue::Option(OptionValue::new(
                        None, 
                        MType::Ticket(Box::new((MType::String, MType::Nat, MType::Address)))
                    ))
                );
                assert_eq!(stack[0].instruction, Instruction::TICKET);
                assert_eq!(stack[1].value, MValue::Int(-22));
                assert_eq!(stack[1].instruction, Instruction::INIT);
                assert_eq!(stack[2].value, MValue::Mutez(6_000_000));
                assert_eq!(stack[2].instruction, Instruction::INIT);
            }
        }
    }

    // FAILING
    #[test]
    fn create_ticket_fail_1() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(22), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 1);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(
                err, 
                "Unexpected stack length, expected a length of 2 for instruction TICKET, got 1"
            ),
            Ok(_) => assert!(false)
        }
    }

    #[test]
    fn create_ticket_fail_2() {
        let initial_stack: Stack = vec![
            StackElement::new(MValue::Nat(22), Instruction::INIT),
            StackElement::new(MValue::String(String::from("tezos")), Instruction::INIT),
            StackElement::new(MValue::Int(-22), Instruction::INIT),
            StackElement::new(MValue::Mutez(6_000_000), Instruction::INIT),
        ];
        let stack_snapshots = vec![];
        let options = RunOptions {
            context: RunOptionsContext::mock(),
            pos: 0,
        };

        assert!(initial_stack.len() == 4);

        match run(initial_stack, &options, stack_snapshots) {
            Err(err) => assert_eq!(
                err, 
                "Wrong type for instruction TICKET, expected a stack of type ['a : nat : S'] but got [nat : string : S']"
            ),
            Ok(_) => assert!(false)
        }
    }
}
