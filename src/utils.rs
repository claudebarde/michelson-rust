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
                    r"^\({{0,1}}({})\s*({})*\){{0,1}}$",
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
                            r"^\({{0,1}}({})\s+({})*(.*)$",
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
                            // checks if the string is a complex type with 2 parameters
                            Err(String::from("Provided string is not 0 param type"))
                        }
                    }
                }
            }
        } else {
            Err(String::from(
                "Provided string doesn't seem to be a valid Michelson type",
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_micheline_to_json() {
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
}
