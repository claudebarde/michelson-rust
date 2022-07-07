use regex::Regex;
use serde_json::{Value};
use crate::stack::{ StackElement, Stack, StackSnapshots };
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

#[derive(Debug)]
struct ParsedCondition {
    left_branch_code: String,
    right_branch_code: String,
    new_code: String
}

#[derive(Debug)]
pub struct RunResult {
    pub stack: Stack,
    pub stack_snapshots: Vec<Stack>,
    pub has_failed: bool,
}

fn parse_conditions(code: &str) -> Result<ParsedCondition, String> {
    // iterates through the string to find curly braces
    let mut opening_curly_braces = 0;
    let mut closing_curly_braces = 0;
    let mut in_first_branch = true;
    let mut condition_first_branch = String::new();
    let mut condition_second_branch = String::new();
    let mut char_count = 0;
    for c in code.chars() {
        // breaks the loop when the number of curly braces is the same
        if opening_curly_braces == closing_curly_braces && opening_curly_braces > 0 && in_first_branch == false {
            break;
        }
        // keeps track of opening curly braces
        if c == '{' {
            opening_curly_braces += 1;
        }
        // keeps track of closing curly braces
        if c == '}' {
            closing_curly_braces += 1;
        }
        // copies the string into the appropriate condition branch
        if opening_curly_braces > 0 && in_first_branch == true {
            condition_first_branch.push(c);
        } else if opening_curly_braces > 0 && in_first_branch == false {
            condition_second_branch.push(c);
        }
        // switch to the second branch
        if opening_curly_braces == closing_curly_braces && opening_curly_braces > 0 && in_first_branch == true {
            in_first_branch = false;
            opening_curly_braces = 0;
            closing_curly_braces = 0;
        }

        char_count += 1;
    };
    // println!("`{}` / `{}`", condition_first_branch, condition_second_branch);
    // TODO: add some safeguards to avoid curly braces that are not closed
    if condition_first_branch.len() > 0 && condition_second_branch.len() > 0 {
        // trims and removes leading and trailing curly braces, spaces and semi-colons
        let front_cleanup_regex = Regex::new(r"^(\s|;|\{|\})+").unwrap();
        let back_cleanup_regex = Regex::new(r"(\s|;|\{|\})+$").unwrap();
        // first branch
        let first_branch = front_cleanup_regex.replace(condition_first_branch.as_str(), "");
        let first_branch = back_cleanup_regex.replace(&first_branch, "");
        // second branch
        let second_branch = front_cleanup_regex.replace(condition_second_branch.as_str(), "");
        let second_branch = back_cleanup_regex.replace(&second_branch, "");
        // code left without the condition
        let new_code = &code[char_count..];
        let new_code = front_cleanup_regex.replace(&new_code, "");
        let new_code = back_cleanup_regex.replace(&new_code, "");

        Ok(ParsedCondition {
            left_branch_code: first_branch.to_string(),
            right_branch_code: second_branch.to_string(),
            new_code: new_code.to_string()
        })
        /*
        // first branch
        let mut first_branch = condition_first_branch.trim();
        if &first_branch.chars().nth(0).unwrap() == &'{' {
            first_branch = &first_branch[1..];
        };
        if &first_branch.chars().nth(first_branch.len() - 1).unwrap() == &'}' {
            first_branch = &first_branch[..first_branch.len() - 1];
        };
        first_branch = first_branch.trim();

        // second branch
        let mut second_branch = condition_second_branch.trim();
        if &second_branch.chars().nth(0).unwrap() == &'{' {
            second_branch = &second_branch[1..];
        };
        if &second_branch.chars().nth(second_branch.len() - 1).unwrap() == &'}' {
            second_branch = &second_branch[..second_branch.len() - 1];
        };
        second_branch = second_branch.trim();

        Ok(ParsedCondition {
            left_branch_code: first_branch.to_string(),
            right_branch_code: second_branch.to_string(),
            char_count
        })*/
    } else {
        Err(String::from("An error has occurred while parsing a condition block"))
    }
}

/// Parses Micheline code into AST
pub fn parse(code: String) -> Result<Vec<ParsedCode>, (String, Vec<ParsedCode>)> {
    // trim the code
    let code_ = code.trim();
    // removes all the new lines
    let re = Regex::new(r"\s+").unwrap();
    let formatted_code = re.replace_all(code_, " ");
    let mut code_loop: String = formatted_code.to_string();
    // REGEX instruction patterns
    let simple_instr_regex = Regex::new(r"^([A-Z_]+)\s*(;|$)").unwrap();
    let one_param_instr_regex = Regex::new(r"^([A-Z_]+)\s+([^\s]+)\s*(;|$)").unwrap();
    let two_param_instr_regex = Regex::new(r#"^([A-Z_]+)\s+([^\s{};]+)\s+"?([^;\s]+?)"?\s*(;|$)"#).unwrap();
    // let if_instr_regex = Regex::new(r"^(IF.*?)\s*\{(.*)\}\s*\{(.*)\}\s*(;|$)").unwrap();
    let if_instr_regex = Regex::new(r#"^(IF_LEFT|IF_SOME|IF_NONE|IF_CONS|IF)[\s|\{]"#).unwrap();

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
            // gets the current instruction
            let instruction = match if_instr_regex.captures(&code_loop) {
                None => panic!("An error has occured while parsing the Michelson code (if instruction regex capture)"),
                Some (m) => m.get(1).unwrap().as_str()
            };
            // parses the code in the condition block
            println!("\ncode loop: {}", code_loop);
            match parse_conditions(&code_loop) {
                Err(err) => panic!("{}", err),
                Ok(res) => {
                    let sub_instr1 = parse(res.left_branch_code);
                    let sub_instr2 = parse(res.right_branch_code);
                    let instrs: (Vec<ParsedCode>, Vec<ParsedCode>) = 
                        match (sub_instr1, sub_instr2) {
                            (Ok(set1), Ok(set2)) => (set1, set2),
                            (Ok(_), Err(err)) => panic!("{}", err.0),
                            (Err(err), Ok(_)) => panic!("{}", err.0),
                            (Err(err1), Err(err2)) => panic!("{}/{}", err1.0, err2.0)
                        };

                    instructions.push(ParsedCode {
                        value: String::from(instruction),
                        val_type: ValType::Condition,
                        args: Some(vec!(
                            instrs.0, 
                            instrs.1
                        ))
                    });
                    // cleans up the string
                    println!("\n parsed code: `{}` \n new code: `{}`", code_loop, res.new_code);
                    code_loop = String::from(res.new_code.trim());
                }
            }
        }
        /* else if if_instr_regex.is_match(&code_loop) {
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
        } */
        else {
            error = (true, String::from("The code didn't match against the patterns"));
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
pub fn run(json: &str, mut stack: Stack, mut stack_snapshots: StackSnapshots) -> Result<RunResult, String> {    
    // sets default options
    // TODO: it may be better to pass these options as a parameter
    let options = RunOptions {
        context: RunOptionsContext {
            amount: 0,
            sender: String::from("tz1Me1MGhK7taay748h4gPnX2cXvbgL6xsYL"),
            source: String::from("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"),
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
        let mut failed = false;
        //println!("{:#?}", json_array);
        'instructions_loop:
        for val in json_array.as_array().unwrap() {
            let prim = &val["prim"].to_string();
            let instruction = 
                match Instruction::from_str(prim) {
                    Ok (i) => i,
                    Err (err) => panic!("{}", err)
                };
            println!("{:?}", instruction);
            // TODO: add support for macros
            match &instruction {
                &Instruction::FAILWITH => {
                    // aborts the execution of the contract
                    // gets the value on top of the stack
                    let failwith_error = stack[0].value.clone();
                    // updates the stack snapshots
                    stack_snapshots.push(stack.clone());
                    // creates a new stack with one value to return
                    stack = vec![StackElement::new(failwith_error, Instruction::FAILWITH)];
                    // breaks from the contract execution loop
                    failed = true;
                    break 'instructions_loop;
                }
                _ => {
                    let args = val["args"].as_array();
                    // println!("snapshot: {:?}", stack_snapshots);
                    (stack, stack_snapshots) = instruction.run(args, stack, stack_snapshots, &options);
                }
            }
        }

        Ok(RunResult {
            stack,
            stack_snapshots,
            has_failed: failed
        })
    } else {
        Err(String::from("Unexpected type output for JSON value, expected an array"))
    }

}