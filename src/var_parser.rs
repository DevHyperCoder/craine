use super::error_handler::ErrorType;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

//TODO multiline support later
/**
 * Gets all the variable definitions present in the content.
 *
 * Syntax:
 *  {var_name||var_value}
 *
 * Error:
 *  - If the variable name is empty
 */
pub fn get_variables(content: &[&str]) -> Result<HashMap<String, String>, ErrorType> {
    lazy_static! {
        static ref VARIABLE_RE: Regex =
            Regex::new("\\{([a-zA-Z0-9_-]+)\\|\\|([\\s\\S]+)\\}").unwrap();
    }

    let mut variables = HashMap::new();

    for i in content {
        match VARIABLE_RE.captures(&i) {
            None => {}
            Some(capture) => {
                let name = capture.get(1).map_or("", |m| m.as_str());
                let value = capture.get(2).map_or("", |m| m.as_str());

                if name.is_empty() {
                    return Err(ErrorType::Parse("Empty variable name"));
                }

                variables.insert(name.to_string(), value.to_string());
            }
        }
    }

    Ok(variables)
}

/**
 * Replaces variables in the content string.
 * TODO in the future, should consider to use a regex or somekind of variable not defined check
 *
 * Syntax:
 *  (var_name)
 *
 * TODO perhaps make it with {var_name} for consistency. (DEBUG later becuase sometimes the
 * variable is not replaced correectly. Might have to do with format strings. perhaps using
 * concatenation might help us)
 */
pub fn replace_variables(
    content: &str,
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
