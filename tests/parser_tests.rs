use pest::Parser;
use JSONParser::{Grammar, Rule};

#[test]
fn parses_simple_key_value() {
    let input = "\"key\": \"value\"";
    let result = Grammar::parse(Rule::key_value, input);

    assert!(result.is_ok(), "Parser failed: {:?}", result.err());
    let pairs = result.unwrap();
    let pair = pairs.peek().unwrap();

    assert_eq!(pair.as_rule(), Rule::key_value);
    assert_eq!(pair.as_str(), input);
}
