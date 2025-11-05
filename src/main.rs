use JSONParser::{Grammar, Rule};
use pest::Parser;

fn main() {
    let got = Grammar::parse(Rule::key_value, "\"key\": \"value\"");
    println!("{:?}", got);
}
