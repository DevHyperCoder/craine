use super::error_handler::ErrorType;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

//TODO multiline support later
//TODO remove the mut cuz its not put anyways
//TODO add better docs
/**
 * get var using regex
 */
pub fn get_variables(content: &mut Vec<&str>) -> Result<HashMap<String, String>, ErrorType> {
    //TODO lazy static later

    lazy_static! {
        static ref VARIABLE_RE: Regex =
            Regex::new("\\{([a-zA-Z0-9_-]+)\\|\\|([\\s\\S]+)\\}").unwrap();
    }

    let mut variables = HashMap::new();
    let mut var_line = vec![];
    // Two iterations ; TODO refactor

    for (index, i) in content.iter().enumerate() {
        match VARIABLE_RE.captures(&i) {
            None => {}
            Some(capture) => {
                let name = capture.get(1).map_or("", |m| m.as_str());
                let value = capture.get(2).map_or("", |m| m.as_str());
                var_line.push(index);

                if name.is_empty() {
                    return Err(ErrorType::Parse("Empty variable name"));
                }

                variables.insert(name.to_string(), value.to_string());
            }
        }
    }

    for i in var_line {
        content[i] = "";
    }

    Ok(variables)
}

// TODO docs
/**
 * replace var using regex
 */
pub fn replace_variables(
    content: &String,
    variables: HashMap<String, String>,
) -> Result<String, ErrorType> {
    let mut c = content.to_string();
    for key in variables.keys().into_iter() {
        let var_name = format!("({})", key);
        let value = variables.get(key).unwrap();
        c = c.replace(var_name.as_str(), value.as_str());
    }

    Ok(c)
}
