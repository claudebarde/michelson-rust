#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod errors;
mod instructions;
mod m_types;
mod parser;
mod stack;

#[cfg(test)]
mod test {
    use super::*;
    use instructions::Instruction;
    use m_types::{or, CollectionValue, MType, MValue, Or, OrValue, PairValue};
    use parser::RunResult;
    use stack::{Stack, StackElement, StackSnapshots};

    /*
        GENERIC CONTRACT
    */
    #[test]
    fn generic_contract() {
        let michelson_code = r#"
            UNPAIR ;
            PUSH mutez 0 ;
            AMOUNT ;
            COMPARE ;
            NEQ ;
            IF { DROP 2 ; PUSH string "NO_AMOUNT_EXPECTED" ; FAILWITH }
                { IF_LEFT { IF_LEFT { SWAP ; SUB } { ADD } } { DROP 2 ; PUSH int 0 } ; NIL operation ; PAIR }"#;

        let parsed_michelson = parser::parse(String::from(michelson_code));
        let parsed_json = match parsed_michelson {
            Ok(mich) => parser::to_json(&mich),
            Err((err, _)) => panic!("{}", err),
        };

        let run_result: Result<RunResult, String> = match parsed_json {
            Ok(json) => {
                // (or (or (int %decrement) (int %increment)) (unit %reset))
                // reset params
                let param_type: or<MType, MType> =
                    (MType::Or(Box::new((MType::Int, MType::Int))), MType::Unit);
                let param = MValue::Or(OrValue {
                    m_type: param_type.clone(),
                    value: Box::new(Or::Right(MValue::Unit)),
                });

                let storage = MValue::Int(5);
                let storage_type = MType::Int;
                // creates the initial stack
                let stack: Stack = vec![StackElement::new(
                    MValue::Pair(PairValue {
                        m_type: (MType::Or(Box::new(param_type)), storage_type),
                        value: Box::new((param, storage)),
                    }),
                    Instruction::INIT,
                )];
                let stack_snapshots: StackSnapshots = vec![stack.clone()];
                parser::run(&json, stack, stack_snapshots)
            }
            Err(err) => Err(err),
        };

        match run_result {
            Err(_) => assert!(false),
            Ok(result) => {
                assert_eq!(result.stack.len(), 1);
                assert_eq!(result.has_failed, false);
                assert_eq!(
                    result.stack[0].get_val().get_type(),
                    MType::Pair(Box::new((
                        MType::List(Box::new(MType::Operation)),
                        MType::Int
                    )))
                );
                assert_eq!(
                    result.stack[0].get_val(),
                    MValue::Pair(PairValue {
                        m_type: (MType::List(Box::new(MType::Operation)), MType::Int),
                        value: Box::new((
                            MValue::List(CollectionValue {
                                m_type: MType::Operation,
                                value: Box::new(vec![])
                            }),
                            MValue::Int(0)
                        ))
                    })
                )
            }
        }
    }

    /*
        CONTRACT WITH A MAP INSTRUCTION
    */
    #[test]
    fn contract_with_map_instruction() {
        let michelson_code = r#"
            CAR ;
            MAP {
                PUSH nat 2 ;
                MUL ;
            } ;
            NIL operation ;
            PAIR ;
        "#;

        let parsed_michelson = parser::parse(String::from(michelson_code));
        let parsed_json = match parsed_michelson {
            Ok(mich) => parser::to_json(&mich),
            Err((err, _)) => panic!("{}", err),
        };

        let run_result: Result<RunResult, String> = match parsed_json {
            Ok(json) => {
                let param_type: MType = MType::List(Box::new(MType::Int));
                let param = MValue::List(CollectionValue {
                    m_type: MType::Int,
                    value: Box::new(vec![MValue::Int(3), MValue::Int(6), MValue::Int(11)]),
                });
                let storage = MValue::List(CollectionValue {
                    m_type: MType::Int,
                    value: Box::new(vec![MValue::Int(5), MValue::Int(6)]),
                });
                let storage_type = MType::List(Box::new(MType::Int));

                // creates the initial stack
                let stack: Stack = vec![StackElement::new(
                    MValue::Pair(PairValue {
                        m_type: (param_type, storage_type),
                        value: Box::new((param, storage)),
                    }),
                    Instruction::INIT,
                )];
                let stack_snapshots: StackSnapshots = vec![stack.clone()];
                parser::run(&json, stack, stack_snapshots)
            }
            Err(err) => Err(err),
        };

        match run_result {
            Err(_) => assert!(false),
            Ok(result) => {
                assert_eq!(result.stack.len(), 1);
                assert_eq!(result.has_failed, false);
                assert_eq!(
                    result.stack[0].get_val().get_type(),
                    MType::Pair(Box::new((
                        MType::List(Box::new(MType::Operation)),
                        MType::List(Box::new(MType::Int))
                    )))
                );
            }
        }
    }
}
