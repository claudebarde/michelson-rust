#![allow(non_snake_case)]

mod parser;
mod stack;
mod m_types;
mod instructions;
mod errors;
mod utils;
use stack::{ Stack };
use m_types::{ MValue, Or };

fn main() {
    let michelson_code = r#"
        UNPAIR ;
        IF_LEFT { IF_LEFT { SWAP ; SUB } { ADD } } { DROP 2 ; PUSH int 0 } ;
        NIL operation ;
        PAIR
    "#;
    let parsed_michelson = parser::parse(String::from(michelson_code));
    // println!("{:#?}", parsed_michelson);
    let parsed_json = 
        match parsed_michelson {
            Ok (mich) => parser::to_json(&mich),
            Err ((err, _)) => panic!("{}", err)
        };
    // println!("{:?}", parsed_json.clone().unwrap());
    let run_result: Result<Stack, String> =
        match parsed_json {
            Ok (json) => {
                let param = MValue::Or(Box::new(Or::Left(MValue::Or(Box::new(Or::Right(MValue::Int(6)))))));
                let storage = MValue::Int(5);
                parser::run(&json, param , storage)
            },
            Err (err) => panic!("{}", err)
        };
    println!("New stack: {:?}", run_result.unwrap())
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
