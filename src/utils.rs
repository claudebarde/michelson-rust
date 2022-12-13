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
fn as_annot(str: &str) -> (String, String) {
    let annot_regex = Regex::new(r"^(%|:)([a-z0-9_]+)\s+(.+)").unwrap();
    match annot_regex.captures(str) {
        None => (String::from(""), str.to_string()),
        Some(caps) => {
            let annot_symbol = caps.get(1).unwrap().as_str();
            let annot_name = caps.get(2).unwrap().as_str();
            let param = caps.get(3).unwrap().as_str();

            (format!("{}{}", annot_symbol, annot_name), param.to_string())
        }
    }
}

/// Returns a JSON string from Michelsine input
///
/// # Argument
///
/// * `micheline` - The Micheline string to turn into JSON, can be a type or a value
pub fn micheline_to_json(micheline: String) -> Result<String, String> {
    // formats parameter by removing trailing spaces
    let micheline = micheline.trim();
    // formats parameter by removing start and end parens if any
    let micheline = micheline.trim_matches(|ch| ch == '(' || ch == ')');
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
    let valid_values_regex = Regex::new(format!("{}", valid_values.join("|")).as_str()).unwrap();

    let word_count_regex = Regex::new(r"[\w-]+").unwrap();
    // gets the number of words in the provided string
    let words_count = word_count_regex.find_iter(micheline).count();

    // checks if string could be a type
    let may_be_type = {
        // gets the occurrences of types in the string
        let types_count = valid_types_regex.find_iter(micheline).count();
        // calculates percentage of types in the string
        let percentage = (types_count / words_count) * 100;
        // estimates if the string includes enough types
        let threshold = 75;
        if percentage >= threshold {
            true
        } else {
            false
        }
    };

    if may_be_type {
        // parses the string into JSON
        // 1st case, simple type
        let type_0_param_regex =
            Regex::new(format!(r"^({})$", TYPES_0_PARAM.join("|")).as_str()).unwrap();

        if type_0_param_regex.is_match(micheline) {
            Ok(format!(r#"{{"prim":"{}"}}"#, micheline))
        } else {
            // 2nd case, simple type with argument
            let type_1_param_regex =
                Regex::new(format!(r"^({})\s+(.*)$", TYPES_1_PARAM.join("|")).as_str()).unwrap();

            let (is_match, res) = match type_1_param_regex.captures(micheline) {
                None => (false, String::from("")),
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
                Ok(String::from("true"))
            }
        }
    } else {
        // if the string is not considered a Michelson type, checks if it may be a value
        let may_be_value = {
            // gets the occurrences of types in the string
            let values_count = valid_values_regex.find_iter(micheline).count();
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
            Err(String::from(
                "The provided string doesn't seem to be a valid Michelson type or value",
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_micheline_to_json() {
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

        let simple_pair = String::from("pair int nat");
        let res = micheline_to_json(simple_pair);
        assert!(res == Ok(String::from("true")));

        let complex_pair = String::from("pair (pair int (option string)) (or nat (option int))");
        let res = micheline_to_json(complex_pair);
        assert!(res == Ok(String::from("true")));

        let wrong_string = String::from("this is a test with an int");
        let res = micheline_to_json(wrong_string);
        assert!(res.is_err());
    }
}
