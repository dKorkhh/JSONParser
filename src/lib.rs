use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JsonGrammar;

/// Represents a JSON value
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

/// Errors that can occur during JSON parsing
#[derive(Debug, Error)]
pub enum ParseError {
    /// Syntax error produced by Pest
    #[error("Syntax error: {0}")]
    Syntax(String),

    /// Number parsing error
    #[error("Invalid number: {0}")]
    InvalidNumber(String),

    /// Internal parsing mismatch
    #[error("Unexpected rule encountered: {0}")]
    UnexpectedRule(String),
}

/// Validates if the input string is a valid JSON
///
/// Returns true if valid, false otherwise
pub fn validate_json(input: &str) -> bool {
    JsonGrammar::parse(Rule::json, input).is_ok()
}

/// Fully parses JSON
pub fn parse_json(input: &str) -> Result<JsonValue, ParseError> {
    let mut pairs =
        JsonGrammar::parse(Rule::json, input).map_err(|e| ParseError::Syntax(e.to_string()))?;

    let first = pairs.next().unwrap();
    let tmp = first.into_inner().next().unwrap();
    parse_value(tmp)
}

/// Converts a pair into a JsonValue
fn parse_value(pair: pest::iterators::Pair<Rule>) -> Result<JsonValue, ParseError> {
    let mut current = pair;

    loop {
        let rule = current.as_rule();

        if rule == Rule::value {
            let mut tmp = current.into_inner();
            current = tmp.next().unwrap();
            continue;
        }

        if rule == Rule::object {
            return parse_object(current);
        }

        if rule == Rule::array {
            return parse_array(current);
        }

        if rule == Rule::string {
            return Ok(parse_string(current));
        }

        if rule == Rule::number {
            return parse_number(current);
        }

        if rule == Rule::boolean {
            return Ok(parse_bool(current));
        }

        if rule == Rule::null {
            return Ok(JsonValue::Null);
        }

        if rule == Rule::key_value || rule == Rule::pair_key_value || rule == Rule::element_list {
            let mut tmp = current.into_inner();
            current = tmp.next().unwrap();
            continue;
        }

        if rule == Rule::WHITESPACE {
            let mut tmp = current.into_inner();
            current = tmp.next().unwrap();
            continue;
        }

        return Err(ParseError::UnexpectedRule(format!("{:?}", rule)));
    }
}

/// Parses a JSON object into Object
fn parse_object(pair: pest::iterators::Pair<Rule>) -> Result<JsonValue, ParseError> {
    let mut map = HashMap::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::key_value {
            let mut tmp = inner.into_inner();
            let key = parse_string(tmp.next().unwrap());
            let val = parse_value(tmp.next().unwrap())?;

            if let JsonValue::String(k) = key {
                map.insert(k, val);
            }
        }
    }

    Ok(JsonValue::Object(map))
}

/// Parses JSON arrays
fn parse_array(pair: pest::iterators::Pair<Rule>) -> Result<JsonValue, ParseError> {
    let mut arr = Vec::new();

    for tmp in pair.into_inner() {
        if tmp.as_rule() == Rule::value {
            arr.push(parse_value(tmp)?);
        }
    }

    Ok(JsonValue::Array(arr))
}

/// Parses JSON strings
fn parse_string(pair: pest::iterators::Pair<Rule>) -> JsonValue {
    let mut s = pair.as_str().to_string();
    s.remove(0);
    s.pop(); 
    JsonValue::String(s)
}

/// Parses JSON numbers
fn parse_number(pair: pest::iterators::Pair<Rule>) -> Result<JsonValue, ParseError> {
    let text = pair.as_str();

    match text.parse::<f64>() {
        Ok(n) => Ok(JsonValue::Number(n)),
        Err(_) => Err(ParseError::InvalidNumber(text.into())),
    }
}

/// Parses booleans
fn parse_bool(pair: pest::iterators::Pair<Rule>) -> JsonValue {
    JsonValue::Bool(pair.as_str() == "true")
}

/// Extracts a field from a JSON object
pub fn get_field<'a>(value: &'a JsonValue, key: &str) -> Option<&'a JsonValue> {
    if let JsonValue::Object(map) = value {
        return map.get(key);
    }
    None
}
