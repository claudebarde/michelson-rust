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

/// Checks if the string includes an annotation
/// Extracts the annotation and returns it along with the parameter type
/// Returns (annot, param without annot)
fn as_annot(str: &str) -> (String, String) {
    let mut types_array: Vec<&str> = vec![];
    types_array.extend(TYPES_0_PARAM.iter());
    types_array.extend(TYPES_1_PARAM.iter());
    types_array.extend(TYPES_2_PARAMS.iter());
    // checks if the annotation is at the head
    let head_annot_regex = Regex::new(
        format!(
            "^(%|:|@)([_0-9a-zA-Z\\.%@]+)\\s+(\\(?({}).*)",
            types_array.join("|")
        )
        .as_str(),
    )
    .unwrap();
    match head_annot_regex.captures(str) {
        None => {
            // checks if the annotation is at the tail
            let tail_annot_regex = Regex::new(
                format!(
                    "^({})\\s+(%|:|@)([_0-9a-zA-Z\\.%@]+)",
                    types_array.join("|")
                )
                .as_str(),
            )
            .unwrap();
            match tail_annot_regex.captures(str) {
                None => (String::from(""), str.to_string()),
                Some(caps) => {
                    let param = caps.get(1).unwrap().as_str();
                    let annot_symbol = caps.get(2).unwrap().as_str();
                    let annot_name = caps.get(3).unwrap().as_str();

                    (format!("{}{}", annot_symbol, annot_name), param.to_string())
                }
            }
        }
        Some(caps) => {
            let annot_symbol = caps.get(1).unwrap().as_str();
            let annot_name = caps.get(2).unwrap().as_str();
            let param = caps.get(3).unwrap().as_str();

            (format!("{}{}", annot_symbol, annot_name), param.to_string())
        }
    }
}

/// Removes surrounding parens if any
fn remove_outer_parens(str: &str) -> &str {
    if str.len() > 0
        && str.chars().nth(0).unwrap() == '('
        && str.chars().nth(str.len() - 1).unwrap() == ')'
    {
        &str[1..(str.len() - 1)]
    } else {
        str
    }
}

/// Returns a JSON string from Michelsine input
///
/// # Argument
///
/// * `micheline` - The Micheline string to turn into JSON, can be a type or a value
pub fn micheline_to_json(micheline: String) -> Result<String, String> {
    if micheline.len() < 3 {
        Err(format!(
            "Expected Micheline string must be at least 3 character long, got `{}`",
            micheline
        ))
    } else {
        // formats parameter by removing trailing spaces
        let micheline = micheline.trim();
        // formats parameter by removing start and end parens if any
        let micheline = remove_outer_parens(micheline);
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

        let valid_values: [&str; 8] = [
            "Pair", "None", "Some", "Left", "Right", "True", "False", "Elt",
        ];
        let valid_values_regex =
            Regex::new(format!("{}", valid_values.join("|")).as_str()).unwrap();
        // removes annotations for more accurate results
        let annot_regex = Regex::new(r"@%|@%%|%@|[@:%][_0-9a-zA-Z][_0-9a-zA-Z\.%@]*").unwrap();
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

        // parses the string into JSON
        if may_be_type {
            // 1st case, simple type
            let type_0_param_regex = Regex::new(
                format!(
                    r"^({})\s*(@%|@%%|%@|[@:%][_0-9a-zA-Z][_0-9a-zA-Z\.%@]*)*$",
                    TYPES_0_PARAM.join("|")
                )
                .as_str(),
            )
            .unwrap();

            let (is_match, res) = match type_0_param_regex.captures(micheline) {
                None => (
                    false,
                    format!("No match found for Michelson type (0 param): {}", micheline),
                ),
                Some(caps) => {
                    let main_type = caps.get(1).unwrap().as_str();
                    let annot = match caps.get(2) {
                        None => String::from(""),
                        Some(annot) => format!(r#","annots":["{}"]"#, annot.as_str()),
                    };

                    (true, format!(r#"{{"prim":"{}"{}}}"#, main_type, annot))
                }
            };

            if is_match {
                Ok(res)
            } else {
                // 2nd case, simple type with argument
                let type_1_param_regex =
                    Regex::new(format!(r"^({})\s+(.*)$", TYPES_1_PARAM.join("|")).as_str())
                        .unwrap();

                let (is_match, res) = match type_1_param_regex.captures(micheline) {
                    None => (
                        false,
                        format!("No match found for Michelson type (1 param): {}", micheline),
                    ),
                    Some(caps) => {
                        let main_type = caps.get(1).unwrap().as_str();
                        let param_type = caps.get(2).unwrap().as_str();

                        // checks if annotation is present
                        let (annot, param_type) = as_annot(param_type);

                        match micheline_to_json(param_type.to_string()) {
                            Ok(arg) => {
                                if annot.len() == 0 {
                                    // no annotation
                                    (
                                        true,
                                        format!(r#"{{"prim":"{}","args":[{}]}}"#, main_type, arg),
                                    )
                                } else {
                                    // annotation
                                    (
                                        true,
                                        format!(
                                            r#"{{"prim":"{}","args":[{}],"annots":["{}"]}}"#,
                                            main_type, arg, annot
                                        ),
                                    )
                                }
                            }
                            Err(_) => (false, String::from("")),
                        }
                    }
                };

                if is_match {
                    Ok(res)
                } else {
                    // 3rd case, complex type
                    let type_2_param_regex = Regex::new(
                        format!(r"^({})\s+([a-z0-9_()% ]+)", TYPES_2_PARAMS.join("|")).as_str(),
                    )
                    .unwrap();

                    let (is_match, res) = match type_2_param_regex.captures(micheline) {
                        None => (
                            false,
                            format!(
                                "No match found for Michelson type (2 params): {}",
                                micheline
                            ),
                        ),
                        Some(caps) => {
                            let main_type = caps.get(1).unwrap().as_str().trim();
                            let params = caps.get(2).unwrap().as_str().trim();
                            let (annot, params) = as_annot(params);

                            // strips surrounding parens if any
                            // let params = remove_outer_parens(params.as_str());

                            // counts the number of open and close parens in the string
                            let has_surrounding_parens = params.chars().nth(0).unwrap() == '('
                                && params.chars().nth(params.len() - 1).unwrap() == ')';

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

                            if (open_parens == 0 && closed_parens == 0)
                                || (open_parens == 1
                                    && closed_parens == 1
                                    && has_surrounding_parens == true)
                            {
                                // no parens or only surrounding parens
                                let params = remove_outer_parens(params.as_str());
                                let split_params: Vec<&str> = params.split_whitespace().collect();
                                if split_params.len() == 2 {
                                    match (
                                        micheline_to_json(split_params[0].to_string()),
                                        micheline_to_json(split_params[1].to_string()),
                                    ) {
                                        (Ok(res1), Ok(res2)) => {
                                            if annot.len() > 0 {
                                                (
                                                    true,
                                                    format!(
                                                        r#"{{"prim":"{}","args":[{},{}],"annots":["{}"]}}"#,
                                                        main_type, res1, res2, annot
                                                    ),
                                                )
                                            } else {
                                                (
                                                    true,
                                                    format!(
                                                        r#"{{"prim":"{}","args":[{},{}]}}"#,
                                                        main_type, res1, res2
                                                    ),
                                                )
                                            }
                                        }
                                        _ => (
                                            false,
                                            format!("Couldn't parse this input: `{}`", params),
                                        ),
                                    }
                                } else {
                                    (
                                        false,
                                        format!(
                                            "This input: `{}` seems to have more than 2 subtypes",
                                            params
                                        ),
                                    )
                                }
                            } else if open_parens == 1
                                && closed_parens == 1
                                && has_surrounding_parens == false
                            {
                                // one of the 2 params is a complex type with parens
                                enum CurrentType {
                                    Left,
                                    Right,
                                }
                                let mut param1 = String::from("");
                                let mut param2 = String::from("");
                                let mut param_switch = CurrentType::Left;

                                for c in params.chars() {
                                    if (c == '(' && param1.len() > 0) || c == ')' {
                                        // some of the string has been parsed at this point
                                        param_switch = CurrentType::Right;
                                    }

                                    if c != '(' && c != ')' {
                                        match param_switch {
                                            CurrentType::Left => param1.push(c),
                                            CurrentType::Right => param2.push(c),
                                        }
                                    }
                                }

                                param1 = param1.trim().to_string();
                                param2 = param2.trim().to_string();

                                match (micheline_to_json(param1), micheline_to_json(param2)) {
                                    (Ok(json_left), Ok(json_right)) => {
                                        // builds the output JSON string
                                        if annot.len() > 0 {
                                            (
                                                true,
                                                format!(
                                                    r#"{{"prim":"{}","args":[{},{}],"annots":["{}"]}}"#,
                                                    main_type, json_left, json_right, annot
                                                ),
                                            )
                                        } else {
                                            (
                                                true,
                                                format!(
                                                    r#"{{"prim":"{}","args":[{},{}]}}"#,
                                                    main_type, json_left, json_right
                                                ),
                                            )
                                        }
                                    }
                                    (Err(err), Ok(_)) | (Ok(_), Err(err)) => (false, err),
                                    (Err(err1), Err(err2)) => (
                                        false,
                                        format!("2 errors occurred: `{}` / `{}`", err1, err2),
                                    ),
                                }
                            } else {
                                // more than 1 set of parens
                                // the parens were stripped before but must be set back to parse the string correctly
                                // let params = format!("({})", params);

                                // separates param1 from param2 by counting parens
                                let mut parens_counter: Option<usize> = None;
                                let mut loop_counter = 0;
                                let mut param1 = String::from("");
                                let mut param2 = String::from("");

                                for c in params.chars() {
                                    if loop_counter == 0 {
                                        param1.push(c);
                                    }

                                    if loop_counter == 1 {
                                        param2.push(c);
                                    }

                                    if c == '(' {
                                        parens_counter = match parens_counter {
                                            None => Some(1),
                                            Some(counter) => Some(counter + 1),
                                        }
                                    }

                                    if c == ')' {
                                        parens_counter = match parens_counter {
                                            None => Some(0),
                                            Some(counter) => Some(counter - 1),
                                        }
                                    }

                                    if let Some(0) = parens_counter {
                                        if loop_counter == 0 {
                                            // first field was processed
                                            loop_counter = 1;
                                            parens_counter = None;
                                        } else if loop_counter == 1 {
                                            // second field was processed
                                            break;
                                        }
                                    }
                                }

                                // makes recursive call to process param1 and param2
                                match (micheline_to_json(param1), micheline_to_json(param2)) {
                                    (Ok(json_left), Ok(json_right)) => {
                                        // builds the output JSON string
                                        if annot.len() > 0 {
                                            (
                                                true,
                                                format!(
                                                    r#"{{"prim":"{}","args":[{},{}],"annots":["{}"]}}"#,
                                                    main_type, json_left, json_right, annot
                                                ),
                                            )
                                        } else {
                                            (
                                                true,
                                                format!(
                                                    r#"{{"prim":"{}","args":[{},{}]}}"#,
                                                    main_type, json_left, json_right
                                                ),
                                            )
                                        }
                                    }
                                    (Err(err), Ok(_)) | (Ok(_), Err(err)) => (false, err),
                                    (Err(err1), Err(err2)) => (
                                        false,
                                        format!("2 errors occurred: `{}` / `{}`", err1, err2),
                                    ),
                                }
                            }
                        }
                    };

                    if is_match {
                        Ok(res)
                    } else {
                        Err(res)
                    }
                }
            }
        } else {
            // if the string is not considered a Michelson type, checks if it may be a value
            let may_be_value = {
                // gets the occurrences of types in the string
                let values_count = valid_values_regex
                    .find_iter(micheline_without_annots.as_str())
                    .count();
                // calculates percentage of types in the string
                let percentage = (values_count / words_count) * 100;
                // estimates if the string includes enough types
                let threshold = 75;
                if percentage >= threshold {
                    true
                } else {
                    false
                }
            };

            if may_be_value {
                // parses the string into JSON

                Ok(String::from("true"))
            } else {
                Err(format!(
                    "The provided string doesn't seem to be a valid Michelson type or value: {}",
                    micheline
                ))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_micheline_to_json_legacy() {
        let simple_type = String::from("nat");
        let res = micheline_to_json(simple_type);
        assert!(res == Ok(String::from("{\"prim\":\"nat\"}")));

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

        let complex_option = String::from("option (list nat)");
        let res = micheline_to_json(complex_option);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}]}]}"
            ))
        );

        let complex_option_with_annot = String::from("option %test (list nat)");
        let res = micheline_to_json(complex_option_with_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"option\",\"args\":[{\"prim\":\"list\",\"args\":[{\"prim\":\"nat\"}]}],\"annots\":[\"%test\"]}"
            ))
        );

        let simple_pair = String::from("pair int nat");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}]}"
            ))
        );

        let simple_pair = String::from("pair int (option string)");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}]}"
            ))
        );

        let simple_pair = String::from("pair (option mutez) string");
        let res = micheline_to_json(simple_pair);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"option\",\"args\":[{\"prim\":\"mutez\"}]},{\"prim\":\"string\"}]}"
            ))
        );

        let simple_pair_with_annot = String::from("pair %simple_pair (int nat)");
        let res = micheline_to_json(simple_pair_with_annot);
        assert!(res == Ok(
            String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"nat\"}],\"annots\":[\"%simple_pair\"]}"
            )
        ));

        let simple_pair_with_field_annots = String::from("pair (int %owner) (nat %amount)");
        let res = micheline_to_json(simple_pair_with_field_annots);
        // println!("{:?}", res);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\",\"annots\":[\"%owner\"]},{\"prim\":\"nat\",\"annots\":[\"%amount\"]}]}"
            ))
        );

        let simple_pair_with_annot = String::from("pair %simple_pair_test int (option string)");
        let res = micheline_to_json(simple_pair_with_annot);
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"int\"},{\"prim\":\"option\",\"args\":[{\"prim\":\"string\"}]}],\"annots\":[\"%simple_pair_test\"]}"
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
        )));

        let wrong_string = String::from("this is a test with an int");
        let res = micheline_to_json(wrong_string);
        assert!(res.is_err());

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
        // println!("{:?}", res);
        assert!(res.is_ok());
        assert!(
            res == Ok(String::from(
                "{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"address\",\"annots\":[\"%administrator\"]},{\"prim\":\"big_map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"pair\",\"args\":[{\"prim\":\"map\",\"args\":[{\"prim\":\"address\"},{\"prim\":\"nat\"}],\"annots\":[\"%approvals\"]},{\"prim\":\"nat\",\"annots\":[\"%balance\"]}]}],\"annots\":[\"%balances\"]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"nat\",\"annots\":[\"%debtCeiling\"]},{\"prim\":\"address\",\"annots\":[\"%governorContractAddress\"]}]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"pair\",\"args\":[{\"prim\":\"big_map\",\"args\":[{\"prim\":\"string\"},{\"prim\":\"bytes\"}],\"annots\":[\"%metadata\"]},{\"prim\":\"bool\",\"annots\":[\"%paused\"]}]},{\"prim\":\"pair\",\"args\":[{\"prim\":\"big_map\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"pair\",\"args\":[{\"prim\":\"nat\"},{\"prim\":\"map\",\"args\":[{\"prim\":\"string\"},{\"prim\":\"bytes\"}]}]}],\"annots\":[\"%token_metadata\"]},{\"prim\":\"nat\",\"annots\":[\"%totalSupply\"]}]}]}]}"
            ))
        );
    }
}
