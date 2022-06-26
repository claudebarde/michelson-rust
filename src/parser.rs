use regex::Regex;
use serde_json::{Value};
use crate::stack::{ Stack, StackSnapshots };
use crate::instructions::{Instruction, RunOptions, RunOptionsContext};

#[derive(Debug)]
enum ValType {
    Simple,
    OneParam,
    TwoParam,
    Condition,
    Unknown
}

#[derive(Debug)]
pub struct ParsedCode {
    pub value: String,
    val_type: ValType,
    pub args: Option<Vec<Vec<ParsedCode>>>
}

/// Parses Micheline code into AST
pub fn parse(code: String) -> Result<Vec<ParsedCode>, (String, Vec<ParsedCode>)> {
    let mut code_loop: String = String::from(code.trim());
    // REGEX instruction patterns
    let simple_instr_regex = Regex::new(r"^([A-Z_]+)\s*(;|$)").unwrap();
    let one_param_instr_regex = Regex::new(r"^([A-Z_]+)\s+([^\s]+)\s*(;|$)").unwrap();
    let two_param_instr_regex = Regex::new(r"^([A-Z_]+)\s+([^\s{};]+)\s+([^;\s]+)\s*(;|$)").unwrap();
    let if_instr_regex = Regex::new(r"^(IF.*?)\s*\{(.*)\}\s*\{(.*)\}\s*(;|$)").unwrap();

    let mut instructions: Vec<ParsedCode> = Vec::new();
    let mut error = (false, String::new());

    while code_loop.len() > 0 {
        // checks for simple instructions
        if simple_instr_regex.is_match(&code_loop) {
            match simple_instr_regex.captures(&code_loop) {
                None => panic!("An error has occured while parsing the Michelson code (simple instruction regex)"),
                Some (m) => {
                    match m.get(1) {
                        None => panic!("An error has occured while parsing the Michelson code (simple instruction regex)"),
                        Some (v) =>  {
                            // pushes the new instruction
                            instructions.push(ParsedCode {
                                value: String::from(v.as_str()),
                                val_type: ValType::Simple,
                                args: None
                            });
                            // updates the code loop
                            let new_code = code_loop.replace(m.get(0).unwrap().as_str(), "");
                            code_loop = String::from(new_code.trim());
                        }
                    }                    
                }
            }
        }
        // checks for instructions with one parameter
        else if one_param_instr_regex.is_match(&code_loop) {
            match one_param_instr_regex.captures(&code_loop) {
                None => panic!("An error has occured while parsing the Michelson code (one-param instruction regex)"),
                Some (m) => {
                    match (m.get(1), m.get(2)) {
                        (Some (instr), Some (param)) => {
                            instructions.push(ParsedCode {
                                value: String::from(instr.as_str()),
                                val_type: ValType::OneParam,
                                args: Some(vec!(vec!(ParsedCode {
                                    value: String::from(param.as_str().trim()),
                                    val_type: ValType::Simple,
                                    args: None
                                })))
                            });
                            // updates the code loop
                            let new_code = code_loop.replace(m.get(0).unwrap().as_str(), "");
                            code_loop = String::from(new_code.trim());
                        },
                        _ => panic!("An error has occured while parsing the Michelson code (one-param instruction regex)"),
                    }                    
                }
            }
        }
        // checks for instructions with two parameters
        else if two_param_instr_regex.is_match(&code_loop) {
            match two_param_instr_regex.captures(&code_loop) {
                None => panic!("An error has occured while parsing the Michelson code (two-param instruction regex)"),
                Some (m) => {
                    match (m.get(1), m.get(2), m.get(3)) {
                        (Some (instr), Some (param1), Some(param2)) => {
                            instructions.push(ParsedCode {
                                value: String::from(instr.as_str()),
                                val_type: ValType::TwoParam,
                                args: Some(vec!(
                                    vec!(ParsedCode {
                                    value: String::from(param1.as_str()),
                                    val_type: ValType::Unknown,
                                    args: None
                                }), vec!(ParsedCode {
                                    value: String::from(param2.as_str()),
                                    val_type: ValType::Unknown,
                                    args: None
                                })))
                            });
                            // updates the code loop
                            let new_code = code_loop.replace(m.get(0).unwrap().as_str(), "");
                            code_loop = String::from(new_code.trim());
                        },
                        _ => panic!("An error has occured while parsing the Michelson code (two-param instruction regex)"),
                    }                    
                }
            }
        }
        // checks for condition instructions
        else if if_instr_regex.is_match(&code_loop) {
            match if_instr_regex.captures(&code_loop) {
                None => panic!("An error has occured while parsing the Michelson code (if instruction regex capture)"),
                Some (m) => {
                    match (m.get(1), m.get(2), m.get(3)) {
                        (Some (instr), Some (param1), Some(param2)) => {
                            // parse subfields
                            let sub_instr1 = parse(String::from(param1.as_str()));
                            let sub_instr2 = parse(String::from(param2.as_str()));
                            let instrs: (Vec<ParsedCode>, Vec<ParsedCode>) = 
                                match (sub_instr1, sub_instr2) {
                                    (Ok(set1), Ok(set2)) => (set1, set2),
                                    (Ok(_), Err(err)) => panic!("{}", err.0),
                                    (Err(err), Ok(_)) => panic!("{}", err.0),
                                    (Err(err1), Err(err2)) => panic!("{}/{}", err1.0, err2.0)
                                };

                            instructions.push(ParsedCode {
                                value: String::from(instr.as_str()),
                                val_type: ValType::Condition,
                                args: Some(vec!(
                                    instrs.0, 
                                    instrs.1
                                ))
                            });
                            // updates the code loop
                            let new_code = code_loop.replace(m.get(0).unwrap().as_str(), "");
                            code_loop = String::from(new_code.trim());
                        },
                        _ => panic!("An error has occured while parsing the Michelson code (if instruction regex parsing)"),
                    }                    
                }
            }
        } else {
            error = (true, String::from("The code couldn't match against the patterns"));
            break;
        }
    }

    if error.0 {
        Err((error.1, instructions))
    } else {
        Ok(instructions)
    }
}

/// Parses AST into JSON
pub fn to_json(ast: &Vec<ParsedCode>) -> Result<String, String> {
    if ast.len() > 0 {
        // loops through the different instructions
        let mut json: String = String::from("[");

        for (index, instruction) in ast.iter().enumerate() {
            match instruction.val_type {
                ValType::Simple => {
                    json.push_str(format!(r#"{{"prim": "{}"}}"#, instruction.value).as_str());
                    // adds a comma separator
                    if index != ast.len() - 1 {
                        json.push_str(", ")
                    }
                },
                ValType::OneParam => {
                    let args: &ParsedCode = 
                        match &instruction.args {
                            None => panic!("Unexpected None value for args"),
                            Some (args_) =>
                                if args_.len() != 1 {
                                    panic!("Unexpected length of vector for instruction arguments")
                                } else {
                                    &args_[0][0]
                                }
                        };
                    let formatted_args: String =
                        match args.value.parse::<f64>() {
                            Ok (v) => format!(r#"{{"int": "{}"}}"#, v),
                            Err (_) => format!(r#"{{"prim": "{}"}}"#, args.value)
                        };
                    json.push_str(format!(
                        r#"{{"prim": "{}", "args": [{}]}}"#, 
                        instruction.value, 
                        formatted_args).as_str());
                    // adds a comma separator
                    if index != ast.len() - 1 {
                        json.push_str(", ")
                    }
                },
                ValType::TwoParam => {
                    let args: (&ParsedCode, &ParsedCode) = 
                        match &instruction.args {
                            None => panic!("Unexpected None value for args"),
                            Some (args_) =>
                                if args_.len() != 2 {
                                    panic!("Unexpected length of vector for instruction arguments")
                                } else {
                                    (&args_[0][0], &args_[1][0])
                                }
                        };
                    let val_type = 
                        match args.1.value.parse::<f64>() {
                            Ok (_) => String::from("int"),
                            Err (_) => String::from("string")
                        };
                    json.push_str(format!(
                        r#"{{"prim": "{}", "args": [{{"prim": "{}"}}, {{"{}": "{}"}} ]}}"#, 
                        instruction.value, 
                        args.0.value,
                        val_type,
                        args.1.value
                    ).as_str());
                    // adds a comma separator
                    if index != ast.len() - 1 {
                        json.push_str(", ")
                    }
                },
                ValType::Condition => {
                    let args: (String, String) = 
                        match &instruction.args {
                            None => panic!("Unexpected None value for args"),
                            Some (args_) =>
                                if args_.len() != 2 {
                                    panic!("Unexpected length of vector for instruction arguments")
                                } else {
                                    let args_left = &args_[0];
                                    let args_right = &args_[1];
                                    match (to_json(args_left), to_json(args_right)) {
                                        (Ok(json1), Ok(json2)) => (json1, json2),
                                        (Ok(_), Err(err)) => panic!("{}", err),
                                        (Err(err), Ok(_)) => panic!("{}", err),
                                        (Err(err1), Err(err2)) => panic!("{}/{}", err1, err2),
                                    }
                                }
                        };
                    json.push_str(format!(
                        r#"{{"prim": "{}", "args": [{}, {}]}}"#,
                        instruction.value,
                        args.0,
                        args.1
                    ).as_str());
                    // adds a comma separator
                    if index != ast.len() - 1 {
                        json.push_str(", ")
                    }
                },
                ValType::Unknown => ()
            }
        }

        json.push_str("]");

        Ok(json)
    } else {
        Err(String::from("Vector is empty"))
    }
}

/// runs JSON Michelson code provided a parameter value and a storage
pub fn run(json: &str, mut stack: Stack, mut stack_snapshots: StackSnapshots) -> Result<(Stack, StackSnapshots), String> {    
    // sets default options
    let options = RunOptions {
        context: RunOptionsContext {
            amount: 0,
            sender: String::from("test_sender"),
            source: String::from("test_source"),
        }, 
        pos: 0
    };
    // loops through the JSON value
    let json_array: Value = 
        match serde_json::from_str(json) {
            Ok (val) => val,
            Err (err) => panic!("{:?}", err)
        };
    if json_array.is_array() {
        //println!("{:#?}", json_array);
        for val in json_array.as_array().unwrap() {
            let prim = &val["prim"].to_string();
            let instruction = 
                match Instruction::from_str(prim) {
                    Ok (i) => i,
                    Err (err) => panic!("{}", err)
                };
            println!("{:?}", instruction);
            let args = val["args"].as_array();
            // println!("snapshot: {:?}", stack_snapshots);
            (stack, stack_snapshots) = instruction.run(args, stack, stack_snapshots, &options);
        }

        Ok((stack, stack_snapshots))
    } else {
        Err(String::from("Unexpected type output for JSON value, expected an array"))
    }

}