use regex::Regex;

const TYPES_0_PARAM: [&str; 21] = [
    "unit",
    "never",
    "bool",
    "int",
    "nat",
    "string",
    "chain_id",
    "bytes",
    "mutez",
    "key_hash",
    "key",
    "signature",
    "timestamp",
    "address",
    "tx_rollup_l2_address",
    "operation",
    "bls12_381_g1",
    "bls12_381_g2",
    "bls12_381_fr",
    "chest",
    "chest_key",
];

const TYPES_1_PARAM: [&str; 7] = [
    "option",
    "list",
    "set",
    "contract",
    "ticket",
    "sapling_transaction",
    "sapling_state",
];

const TYPES_2_PARAMS: [&str; 5] = ["pair", "or", "map", "big_map", "lambda"];

const ANNOT_PATTERN: &str = r"@%|@%%|%@|[@:%][_0-9a-zA-Z][_0-9a-zA-Z\.%@]*";

pub fn split_params(
    params: &str,
    micheline: &str,
    main_type: &str,
    annot: String,
) -> Result<String, String> {
    let split_params: Vec<&str> = params.split_whitespace().collect();

    // println!("split params: {} / {:?}", params, split_params);

    if split_params.len() == 2 {
        match (
            micheline_to_json(split_params[0].to_string()),
            micheline_to_json(split_params[1].to_string()),
        ) {
            (Ok(res1), Ok(res2)) => Ok(format!(
                r#"{{"prim":"{}","args":[{},{}]{}}}"#,
                main_type, res1, res2, annot
            )),
            _ => Err(format!(
                "Couldn't parse this params: `{}` for this Micheline input: `{}`",
                params, micheline
            )),
        }
    } else {
        Err(format!(
            "Error while splitting these params: `{}` for this Micheline input: `{}`",
            params, micheline
        ))
    }
}

/// Removes surrounding parens if any
fn remove_outer_parens(str: &str) -> &str {
    let adjacent_parens = Regex::new(r"\)\s*\(").unwrap();

    if str.len() > 0
        && str.chars().nth(0).unwrap() == '('
        && str.chars().nth(str.len() - 1).unwrap() == ')'
        && adjacent_parens.find(str).is_none()
    {
        &str[1..(str.len() - 1)]
    } else {
        // checks if 2 param type enclosed in parens
        let complex_type_parens =
            Regex::new(format!(r"^\({{1}}({})\s+(.*)\){{1}}$", TYPES_2_PARAMS.join("|")).as_str())
                .unwrap();

        if complex_type_parens.is_match(str) {
            &str[1..(str.len() - 1)]
        } else {
            str
        }
    }
}

/// Returns a JSON string from Micheline input
///
/// # Argument
///
/// * `micheline` - The Micheline string to turn into JSON, can be a type or a value
pub fn micheline_to_json(micheline: String) -> Result<String, String> {
    // formats parameter by removing trailing spaces
    let micheline = micheline.trim();
    // formats line returns and white spaces
    let format_regex = Regex::new(r"\s{2,}").unwrap();
    let micheline_string = format_regex.replace_all(micheline, " ").to_string();
    let micheline = micheline_string.as_str();
    // figures out if the passed string is a type or a value
    let all_types: Vec<&&str> = TYPES_0_PARAM
        .iter()
        .chain(TYPES_1_PARAM.iter())
        .chain(TYPES_2_PARAMS.iter())
        .collect();
    let valid_types_regex = Regex::new(
        format!(
            "{}",
            all_types
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("|")
        )
        .as_str(),
    )
    .unwrap();

    // removes the annotations to get a more precise result
    let annot_regex = Regex::new(ANNOT_PATTERN).unwrap();
    let micheline_without_annots = annot_regex.replace_all(micheline, "").to_string();
    // gets the number of words in the provided string
    let word_count_regex = Regex::new(r"[\w-]+").unwrap();
    let words_count = word_count_regex
        .find_iter(micheline_without_annots.as_str())
        .count();

    // checks if string could be a type
    let may_be_type = {
        // gets the occurrences of types in the string
        let types_count = valid_types_regex
            .find_iter(micheline_without_annots.as_str())
            .count();
        // calculates percentage of types in the string
        let percentage = (types_count as f64 / words_count as f64) * 100.0;
        // estimates if the string includes enough types
        let threshold = 75.0;
        if percentage >= threshold {
            true
        } else {
            false
        }
    };

    if may_be_type {
        // checks if the string is a simple type
        let type_0_param_regex = Regex::new(
            format!(
                r"^\(*({})\s*({})*\)*$",
                TYPES_0_PARAM.join("|"),
                ANNOT_PATTERN
            )
            .as_str(),
        )
        .unwrap();
        match type_0_param_regex.captures(micheline) {
            Some(caps) => {
                let main_type = caps.get(1).unwrap().as_str();
                let annot = match caps.get(2) {
                    None => String::from(""),
                    Some(annot) => format!(r#","annots":["{}"]"#, annot.as_str()),
                };

                Ok(format!(r#"{{"prim":"{}"{}}}"#, main_type, annot))
            }
            None => {
                // checks if the string is a complex type with 1 parameter
                let type_1_param_regex = Regex::new(
                    format!(
                        r"^\(*({})\s+({})*(.*)$",
                        TYPES_1_PARAM.join("|"),
                        ANNOT_PATTERN
                    )
                    .as_str(),
                )
                .unwrap();
                match type_1_param_regex.captures(micheline) {
                    Some(caps) => {
                        let main_type = caps.get(1).unwrap().as_str();
                        let annot = match caps.get(2) {
                            None => String::from(""),
                            Some(annot) => format!(r#","annots":["{}"]"#, annot.as_str()),
                        };
                        let param_type = {
                            let param_cap = caps.get(3).unwrap().as_str();
                            // removes trailing parens if string started with a parens
                            if micheline.chars().nth(0).unwrap() == '('
                                && param_cap.chars().nth(param_cap.len() - 1).unwrap() == ')'
                            {
                                param_cap.to_string().pop();
                            }

                            param_cap
                        };

                        let param_to_json = micheline_to_json(param_type.to_string())?;

                        Ok(format!(
                            r#"{{"prim":"{}","args":[{}]{}}}"#,
                            main_type, param_to_json, annot
                        ))
                    }
                    None => {
                        // removes outer parens
                        let formatted_micheline = remove_outer_parens(micheline);
                        // println!("\nformatted_micheline: {}", formatted_micheline);
                        // checks if the string is a complex type with 2 parameters
                        let type_2_params_regex = Regex::new(
                            format!(
                                r"^({})\s+({})*(.*)$",
                                TYPES_2_PARAMS.join("|"),
                                ANNOT_PATTERN
                            )
                            .as_str(),
                        )
                        .unwrap();

                        match type_2_params_regex.captures(formatted_micheline) {
                            None => Err(String::from(
                                "The provided string is not a valid Michelson type",
                            )),
                            Some(caps) => {
                                let main_type = caps.get(1).unwrap().as_str();
                                let annot = match caps.get(2) {
                                    None => String::from(""),
                                    Some(annot) => {
                                        format!(r#","annots":["{}"]"#, annot.as_str())
                                    }
                                };
                                let params = caps.get(3).unwrap().as_str().trim();

                                // captures the first part of the parameter by reading the parens
                                let open_parens = params
                                    .chars()
                                    .filter(|&c| c == '(')
                                    .collect::<Vec<char>>()
                                    .len();
                                let closed_parens = params
                                    .chars()
                                    .filter(|&c| c == ')')
                                    .collect::<Vec<char>>()
                                    .len();

                                let is_first_char_paren = params.chars().nth(0).unwrap() == '(';
                                let is_last_char_paren =
                                    params.chars().nth(params.len() - 1).unwrap() == ')';
                                let nested_parens =
                                    params.find(") (").is_some() || params.find(")(").is_some();

                                if open_parens == 0 && closed_parens == 0 {
                                    // no parens, 2 simple types
                                    split_params(params, formatted_micheline, main_type, annot)
                                } else if open_parens == 1
                                    && closed_parens == 1
                                    && is_first_char_paren
                                    && is_last_char_paren
                                    && !nested_parens
                                {
                                    split_params(
                                        &params[1..(params.len() - 1)],
                                        formatted_micheline,
                                        main_type,
                                        annot,
                                    )
                                } else {
                                    let params = {
                                        if is_first_char_paren
                                            && is_last_char_paren
                                            && !nested_parens
                                        {
                                            // pattern => (... (...)) || ((...) ...)
                                            &params[1..(params.len() - 1)]
                                        } else {
                                            params
                                        }
                                    };
                                    let mut open_parens_counter = 0;
                                    let mut closed_parens_counter = 0;
                                    let mut left_param = String::from("");

                                    for c in params.chars() {
                                        if c == '('
                                            && left_param.len() > 0
                                            && open_parens_counter == 0
                                        {
                                            // this is the second arg starting
                                            break;
                                        } else if c == '(' {
                                            open_parens_counter += 1;
                                        }

                                        if c == ')' {
                                            closed_parens_counter += 1;
                                        }

                                        left_param.push(c);

                                        // breaks after the first set of parens is closed
                                        if open_parens_counter > 0
                                            && open_parens_counter == closed_parens_counter
                                        {
                                            break;
                                        }
                                    }

                                    // splits the left param from the whole param string
                                    let right_param = params.replacen(left_param.as_str(), "", 1);

                                    // println!(
                                    //     "open_parens_counter: {}\nclosed_parens_counter: {}\nnested_parens: {}\nis_first_char_paren: {}\nis_last_char_paren: {}\nleft param: `{}` \nright param: `{}` \nmicheline: `{}`\n",
                                    //     open_parens_counter, closed_parens_counter, nested_parens, is_first_char_paren, is_last_char_paren, left_param, right_param, formatted_micheline
                                    // );

                                    // checks params with recursive call
                                    match (
                                            micheline_to_json(left_param.clone()),
                                            micheline_to_json(right_param.clone()),
                                        ) {
                                            (Ok(res1), Ok(res2)) => Ok(format!(
                                                r#"{{"prim":"{}","args":[{},{}]{}}}"#,
                                                main_type, res1, res2, annot
                                            )),
                                            (Ok(_), Err(err)) => Err(format!(
                                                "Couldn't parse this input: `{}`, error: {}",
                                                right_param, err
                                            )),
                                            (Err(err), Ok(_)) => Err(format!(
                                                "Couldn't parse this input: `{}`, error: {}",
                                                left_param, err
                                            )),
                                            (Err(err1), Err(err2)) => Err(format!(
                                                "Couldn't parse these inputs: `{} / {}`, errors: {} / {}",
                                                left_param, right_param, err1, err2
                                            )),
                                        }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        // checks if the string may be a Michelson value
        // looking for Unit, True, False, None
        if micheline == "Unit" {
            Ok(format!(r#"{{"prim":"{}"}}"#, micheline))
        } else if micheline == "True" || micheline == "False" {
            Ok(format!(r#"{{"prim":"{}"}}"#, micheline))
        } else if micheline == "None" {
            Ok(format!(r#"{{"prim":"{}"}}"#, micheline))
        } else {
            // looking for numbers, strings or bytes
            let simple_value_regex =
                Regex::new(r#"^([0-9-]+|[a-zA-Z_"\r\n\t\\b\\]*|0x[0-9a-fA-F]+)$"#).unwrap();
            match simple_value_regex.captures(micheline) {
                Some(cap) => {
                    let val = cap.get(1).unwrap().as_str();
                    if val.parse::<i64>().is_ok() {
                        // numeric value
                        Ok(format!(r#"{{"int":"{}"}}"#, micheline))
                    } else if val.len() > 1 && &val[..2] == "0x" {
                        // bytes
                        Ok(format!(r#"{{"bytes":"{}"}}"#, micheline))
                    } else {
                        // string
                        Ok(format!(r#"{{"string":"{}"}}"#, micheline))
                    }
                }
                None => {
                    // looking for Left <value>, Right <value>, Some <value>
                    let value_1_param_regex = Regex::new(r"^(Left|Right|Some)\s+(.*)$").unwrap();
                    match value_1_param_regex.captures(micheline) {
                        Some(caps) => {
                            let main_value = caps.get(1).unwrap().as_str();
                            let param_value = caps.get(2).unwrap().as_str();

                            match micheline_to_json(param_value.to_string()) {
                                Ok(res) => Ok(format!(r#"{{"prim":"{}","args":[{}]}}"#, main_value, res)),
                                Err(err) => Err(format!("Argument for provided value `{}` seems to be wrong: `{}`, error: {}", main_value, param_value, err))
                            }
                        }
                        None => {
                            // looking for Pair <data> <data>
                            let pair_regex = Regex::new(r"Pair\s+(.*)").unwrap();
                            match pair_regex.captures(micheline) {
                                Some(caps) => {
                                    let args = caps.get(1).unwrap().as_str();

                                    if args.len() < 1 {
                                        // unexpected empty string
                                        Err(format!("Unexpected empty argument for Pair value: {}", micheline))
                                    } else if args.find("\"").is_some() {
                                        if args.chars().nth(0).unwrap() == '"' {
                                            // string is first argument
                                            let arg_regex = Regex::new(r#"^"(.+)"\s+(.+)"#).unwrap();
                                            match arg_regex.captures(args) {
                                                None => Err(format!("Error parsing Pair parameters: {}", args)),
                                                Some(caps) => {
                                                    let first_arg = caps.get(1).unwrap().as_str();
                                                    let second_arg = caps.get(2).unwrap().as_str();
                                                    // checks the second argument
                                                    let second_arg = micheline_to_json(second_arg.to_string())?;
                                                    Ok(format!(r#"{{"prim":"Pair","args":[{{"string":"{}"}},{}]}}"#, first_arg, second_arg))
                                                }
                                            }
                                        } else if args.chars().nth(args.len() - 1).unwrap() == '"' {
                                            // string is last argument
                                            let arg_regex = Regex::new(r#"(.*)\s+"(.+)"$"#).unwrap();
                                            match arg_regex.captures(args) {
                                                None => Err(format!("Error parsing Pair parameters: {}", args)),
                                                Some(caps) => {
                                                    let first_arg = caps.get(1).unwrap().as_str();
                                                    let second_arg = caps.get(2).unwrap().as_str();
                                                    // checks the second argument
                                                    let first_arg = micheline_to_json(first_arg.to_string())?;
                                                    Ok(format!(r#"{{"prim":"Pair","args":[{},{{"string":"{}"}}]}}"#, first_arg, second_arg))
                                                }
                                            }
                                        } else {
                                            Err(String::from("test"))
                                        }
                                    } else if args.find("(").is_some() {
                                        Err(String::from("test"))
                                    } else if args.find("{").is_some() {
                                        Err(String::from("test"))
                                    } else {
                                        // numeric or byte values
                                        let vals = args.split_whitespace().into_iter().collect::<Vec<&str>>();
                                        if vals.len() == 2 {
                                            match (micheline_to_json(vals[0].to_string()), micheline_to_json(vals[1].to_string())) {
                                                (Ok(left), Ok(right)) => Ok(format!(r#"{{"prim":"Pair","args":[{},{}]}}"#, left, right)),
                                                (Ok(_), Err(err)) | (Err(err), Ok(_)) => Err(format!("Error parsing Pair value: {}, error: {}", micheline, err)),
                                                _ => Err(format!("Error parsing Pair value: {}", micheline))
                                            }
                                        } else {
                                            Err(format!("Found numeric or byte value for Pair, but couldn't split it: {}", micheline))
                                        }
                                    }
                                }
                                None => Err(String::from(
                                    "Provided string doesn't seem to be a valid Michelson type or value",
                                ))
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn utils_test_micheline_to_json_simple_types() {
        let simple_nat_type = String::from("nat");
        let res = micheline_to_json(simple_nat_type);
        assert!(res == Ok(String::from("{\"prim\":\"nat\"}")));

        let simple_bool_type = String::from("bool");
        let res = micheline_to_json(simple_bool_type);
        assert!(res == Ok(String::from("{\"prim\":\"bool\"}")));

        let simple_int_type_with_annot = String::from("int %counter");
        let res = micheline_to_json(simple_int_type_with_annot);
        assert!(res == Ok(String::from("{\"prim\":\"int\",\"annots\":[\"%counter\"]}")));

        let simple_int_type_with_annot = String::from("(address %owner)");
        let res = micheline_to_json(simple_int_type_with_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"address\",\"annots\":[\"%owner\"]}"
            ))
        );
    }

    #[test]
    pub fn utils_test_micheline_to_json_1_param() {
        let simple_option = String::from("option nat");
        let res = micheline_to_json(simple_option);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"nat\"}]}"
            ))
        );

        let simple_option_with_annot = String::from("option %my_option nat");
        let res = micheline_to_json(simple_option_with_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"nat\"}],\"annots\":[\"%my_option\"]}"
            ))
        );

        let simple_option_with_annot_and_parens = String::from("(option %my_option nat)");
        let res = micheline_to_json(simple_option_with_annot_and_parens);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"nat\"}],\"annots\":[\"%my_option\"]}"
            ))
        );

        let complex_option = String::from("option (list nat)");
        let res = micheline_to_json(complex_option);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}]}]}"
            ))
        );

        let complex_option_with_option_annot = String::from("option %my_option (list nat)");
        let res = micheline_to_json(complex_option_with_option_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}]}],\"annots\":[\"%my_option\"]}"
            ))
        );

        let complex_option_with_args_annot = String::from("option (list %my_list nat)");
        let res = micheline_to_json(complex_option_with_args_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}],\"annots\":[\"%my_list\"]}]}"
            ))
        );

        let complex_option_with_both_annot = String::from("option %my_option (list %my_list nat)");
        let res = micheline_to_json(complex_option_with_both_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}],\"annots\":[\"%my_list\"]}],\"annots\":[\"%my_option\"]}"
            ))
        );
    }

    #[test]
    pub fn utils_test_micheline_to_json_2_params() {
        let simple_pair = String::from("pair int nat");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}]}"
            )),
            "pair int nat failing"
        );

        let simple_pair_with_parens = String::from("(pair int nat)");
        let res = micheline_to_json(simple_pair_with_parens);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}]}"
            )),
            "(pair int nat) failing"
        );

        let simple_pair_with_annot = String::from("pair %simple_pair (int nat)");
        let res = micheline_to_json(simple_pair_with_annot);
        assert!(res == Ok(
            String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}],\"annots\":[\"%simple_pair\"]}"
            )
        ),
        "`pair %simple_pair (int nat)` failed");

        let simple_pair_with_annot = String::from("pair %simple_pair int nat");
        let res = micheline_to_json(simple_pair_with_annot);
        assert!(res == Ok(
            String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}],\"annots\":[\"%simple_pair\"]}"
            )
        ),
        "`pair %simple_pair int nat` failed");

        let simple_pair = String::from("pair (option mutez) string");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"option\",\"args\":[{\"prim\":\"mutez\"}]},{\"prim\":\"string\"}]}"
            ))
        );

        let simple_pair = String::from("pair int (option string)");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}]}"
            )),
            "`pair int (option string)`failed"
        );

        let simple_pair_with_parens = String::from("(pair int (option string))");
        let res = micheline_to_json(simple_pair_with_parens);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}]}"
            )),
            "`pair int (option string)`failed"
        );

        let simple_pair_with_annot = String::from("pair %my_pair (int (option string))");
        let res = micheline_to_json(simple_pair_with_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}],\"annots\":[\"%my_pair\"]}"
            )),
            "`pair %my_pair (int (option string))` failed"
        );

        let simple_pair_with_field_annots = String::from("pair (int %owner) (nat %amount)");
        let res = micheline_to_json(simple_pair_with_field_annots);
        // println!("{:?}", res);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\",\"annots\":[\"%owner\"]},{\"prim\":\"nat\",\"annots\":[\"%amount\"]}]}"
            ))
        );

        let nested_pairs = String::from("pair (pair string int) (pair nat mutez)");
        let res = micheline_to_json(nested_pairs);
        assert!(res == Ok(String::from(
            "{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"string\"},{\"prim\":\"int\"}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"mutez\"}]}]}"
        )));

        let complex_pair = String::from("pair (pair int (option string)) (or nat (option int))");
        let res = micheline_to_json(complex_pair);
        assert!(res == Ok(String::from(
            "{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}]},{\"prim\":\"or\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"int\"}]}]}]}"
            )),
            "`pair (pair int (option string)) (or nat (option int))`failed with error: {:?}",
            res
        );

        let wrong_string = String::from("this is a test with an int");
        let res = micheline_to_json(wrong_string);
        assert!(res.is_err());

        let complex_big_map = String::from(
            "(big_map %balances address (pair (map %approvals address nat) (nat %balance)))",
        );
        let res = micheline_to_json(complex_big_map);
        assert!(res == Ok(String::from(
            "{\"prim\":\"big_map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"pair\",\"args\":[{\"prim\":\"map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"nat\"}],\"annots\":[\"%approvals\"]},{\"prim\":\"nat\",\"annots\":[\"%balance\"]}]}],\"annots\":[\"%balances\"]}"
            )),
            "`(big_map %balances address (pair (map %approvals address nat) (nat %balance)))`failed with error: {:?}",
            res
        );

        let kusd_storage_type = String::from(
            "pair
            (pair
            (pair (address %administrator)
                    (big_map %balances address
                                    (pair (map %approvals address nat) (nat %balance))))
            (pair (nat %debtCeiling) (address %governorContractAddress)))
            (pair (pair (big_map %metadata string bytes) (bool %paused))
                (pair (big_map %token_metadata nat (pair nat (map string bytes)))
                        (nat %totalSupply)))",
        );
        let res = micheline_to_json(kusd_storage_type);
        // println!("\n{:?}\n", res);
        assert!(res.is_ok());
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"address\",\"annots\":[\"%administrator\"]},{\"prim\":\"big_map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"pair\",\"args\":[{\"prim\":\"map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"nat\"}],\"annots\":[\"%approvals\"]},{\"prim\":\"nat\",\"annots\":[\"%balance\"]}]}],\"annots\":[\"%balances\"]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"nat\",\"annots\":[\"%debtCeiling\"]},{\"prim\":\"address\",\"annots\":[\"%governorContractAddress\"]}]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"big_map\",\"args\":[{\"prim\":\"string\"},{\"prim\":\"bytes\"}],\"annots\":[\"%metadata\"]},{\"prim\":\"bool\",\"annots\":[\"%paused\"]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"big_map\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"pair\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"map\",\"args\":[{\"prim\":\"string\"},{\"prim\":\"bytes\"}]}]}],\"annots\":[\"%token_metadata\"]},{\"prim\":\"nat\",\"annots\":[\"%totalSupply\"]}]}]}]}"
            ))
        );
    }

    #[test]
    pub fn utils_test_micheline_to_json_simple_values() {
        let simple_nat_value = String::from("3");
        let res = micheline_to_json(simple_nat_value);
        assert!(res == Ok(String::from("{\"int\":\"3\"}")));

        let simple_int_value = String::from("-69");
        let res = micheline_to_json(simple_int_value);
        assert!(res == Ok(String::from("{\"int\":\"-69\"}")));

        let simple_string_value = String::from("tezos");
        let res = micheline_to_json(simple_string_value);
        assert!(res == Ok(String::from("{\"string\":\"tezos\"}")));

        let empty_string_value = String::from("");
        let res = micheline_to_json(empty_string_value);
        assert!(res == Ok(String::from("{\"string\":\"\"}")));

        let simple_bytes_value = String::from("0x7461717569746f");
        let res = micheline_to_json(simple_bytes_value);
        assert!(res == Ok(String::from("{\"bytes\":\"0x7461717569746f\"}")));

        let unit_value = String::from("Unit");
        let res = micheline_to_json(unit_value);
        assert!(res == Ok(String::from("{\"prim\":\"Unit\"}")));

        let none_value = String::from("None");
        let res = micheline_to_json(none_value);
        assert!(res == Ok(String::from("{\"prim\":\"None\"}")));

        let true_value = String::from("True");
        let res = micheline_to_json(true_value);
        assert!(res == Ok(String::from("{\"prim\":\"True\"}")));

        let false_value = String::from("False");
        let res = micheline_to_json(false_value);
        assert!(res == Ok(String::from("{\"prim\":\"False\"}")));
    }

    pub fn utils_test_micheline_to_json_1_param_values() {
        let some_value = String::from("Some 69");
        let res = micheline_to_json(some_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Some\",\"args\":[{\"int\":\"69\"}]}"
            ))
        );

        let left_value = String::from("Left \"tezos\"");
        let res = micheline_to_json(left_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Left\",\"args\":[{\"string\":\"tezos\"}]}"
            ))
        );

        let right_value = String::from("Right \"True\"");
        let res = micheline_to_json(right_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Right\",\"args\":[{\"prim\":\"True\"}]}"
            ))
        );
    }

    #[test]
    pub fn utils_test_micheline_to_json_pair_values() {
        let simple_pair_value = String::from("Pair 5 6");
        let res = micheline_to_json(simple_pair_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Pair\",\"args\":[{\"int\":\"5\"},{\"int\":\"6\"}]}"
            ))
        );

        let simple_pair_value = String::from("Pair \"tezos\" 45");
        let res = micheline_to_json(simple_pair_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Pair\",\"args\":[{\"string\":\"tezos\"},{\"int\":\"45\"}]}"
            ))
        );

        let simple_pair_value = String::from("Pair 45 \"tezos\"");
        let res = micheline_to_json(simple_pair_value);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Pair\",\"args\":[{\"int\":\"45\"},{\"string\":\"tezos\"}]}"
            ))
        );

        let simple_nested_pair_value = String::from("Pair (Pair 45 50) \"tezos\"");
        let res = micheline_to_json(simple_nested_pair_value);
        println!("{:?}", res);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"Pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"int\":\"45\"},{\"int\":\"50\"}]},{\"string\":\"tezos\"}]}"
            ))
        );
    }
}
