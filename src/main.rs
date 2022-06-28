#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod errors;
mod instructions;
mod m_types;
mod parser;
mod stack;
mod utils;
use instructions::Instruction;
use m_types::{or, MType, MValue, Or, OrValue, PairValue};
use stack::{Stack, StackElement, StackSnapshots};

fn main() {
    let michelson_code = r#"
        UNPAIR ;
        IF_LEFT { IF_LEFT { SWAP ; SUB } { ADD } } { DROP 2 ; PUSH int 0 } ;
        NIL operation ;
        PAIR
    "#;
    /*
        UNPAIR ;
         PUSH mutez 0 ;
         AMOUNT ;
         COMPARE ;
         NEQ ;
         IF { DROP 2 ; PUSH string "NO_AMOUNT_EXPECTED" ; FAILWITH }
            { IF_LEFT { IF_LEFT { SWAP ; SUB } { ADD } } { DROP 2 ; PUSH int 0 } ;
              NIL operation ;
              PAIR }
    */
    let parsed_michelson = parser::parse(String::from(michelson_code));
    // println!("{:#?}", parsed_michelson);
    let parsed_json = match parsed_michelson {
        Ok(mich) => parser::to_json(&mich),
        Err((err, _)) => panic!("{}", err),
    };
    // println!("{:?}", parsed_json.clone().unwrap());
    let run_result: Result<(Stack, StackSnapshots), String> = match parsed_json {
        Ok(json) => {
            // (or (or (int %decrement) (int %increment)) (unit %reset))
            // addition params
            let param_type: or<MType, MType> =
                (MType::Or(Box::new((MType::Int, MType::Int))), MType::Unit);
            /*
            let param = MValue::Or(OrValue {
                m_type: param_type.clone(),
                value: Box::new(Or::Left(MValue::Or(OrValue {
                    m_type: (MType::Int, MType::Int),
                    value: Box::new(Or::Right(MValue::Int(6))),
                }))),
            });*/
            // subtraction params
            /*let param = MValue::Or(OrValue {
                m_type: param_type.clone(),
                value: Box::new(Or::Left(MValue::Or(OrValue {
                    m_type: (MType::Int, MType::Int),
                    value: Box::new(Or::Left(MValue::Int(6))),
                }))),
            });*/
            // reset params
            let param = MValue::Or(OrValue {
                m_type: param_type.clone(),
                value: Box::new(Or::Right(MValue::Unit)),
            });
            let storage = MValue::Int(5);
            let storage_type = MType::Int;
            println!("\nInput: param = {:?} / storage = {:?}\n", param, storage);
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
        Err(err) => panic!("{}", err),
    };
    let (new_stack, stack_snapshots) = run_result.unwrap();
    println!("\nNew stack: {:#?}", new_stack);
    println!("Number of elements in the stack: {}", new_stack.len());
    // println!("{:#?}", stack_snapshots);
    println!("Number of stack snapshots: {}", stack_snapshots.len());
    /*let michelson_code = r#"
        ## Checks if amount is equal to zero
        AMOUNT ;
        PUSH mutez 0 ;
        IFCMPNEQ
            { PUSH string "NOAMOUNTALLOWED" ; FAILWITH }
            {} ;
        UNPPAIIR ;
        DUP ;
        SENDER ;
        IFCMPEQ
            { PUSH string "FORBIDDENSELFTRANFER" ; FAILWITH }
            {} ;
        ## Checks if source is in the ledger
        DIG 2 ;
        DUP ;
        SENDER ;
        MEM ;
        IF
            {
                ## Checks if source has enough balance
                DUP ;
                SENDER ;
                GET ;
                IF_NONE
                    { PUSH string "ERROR" ; FAILWITH }
                    {
                        DUP ;
                        DIP 4 { DUP } ;
                        DIG 4 ;
                        IFCMPGT { PUSH string "INSUFFICIENTBALANCE" ; FAILWITH } {} ;
                    } ;
                ## Updates sender's balance
                DIP 3 { DUP } ;
                DIG 3 ;
                SWAP ;
                SUB ;
                ABS ;
                SOME ;
                SENDER ;
                UPDATE ;
                ## Updates recipient's balance
                DIP { DUP } ;
                SWAP ;
                DIP { DUP } ;
                MEM ;
                IF
                    {
                        SWAP ;
                        DIP { DUP } ;
                        DUP ;
                        DIP { SWAP } ;
                        GET ;
                        IF_NONE
                            {
                                PUSH string "UNKNOWNBALANCE" ; FAILWITH ;
                            }
                            {
                                DIG 3 ;
                                ADD ;
                                SOME ;
                                SWAP ;
                                UPDATE ;
                            } ;
                    }
                    {
                        DUG 2 ;
                        DIP { SOME } ;
                        UPDATE ;
                    } ;
                ## Ends execution
                NIL operation ;
                PAIR ;
            }
            { PUSH string "UNKNOWNSPENDER" ; FAILWITH } ;
    "#;*/
}
