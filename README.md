# json_parser

## Description: 

This will be a parser for a JSON-type structure, which will include parsing of simple fields, arrays, objects, and boolean/string/integer values.

The structure of JSON must follow the rule that it starts with '{' and ends with '}'.

Example: 
```
    {
        "name": "SomeName",
        "value": 1.2,
        "object": {
            "someField": "newValue"
        },
        "array": [
            {
                "value1": "val1" 
            },
            {
                "value1": "val2" 
            }
        ]
    }
```

Some fields can contain null or an empty string "".


## Output

After parsing a JSON file, the program extracts the value of each field and converts it into a corresponding Rust data structure.

## JSON Grammar

```
json = _{ SOI ~ WHITESPACE? ~ value ~ WHITESPACE? ~ EOI }

value = { object | array | string | number | boolean | null }

object = { "{" ~ WHITESPACE? ~ pair_key_value? ~ WHITESPACE? ~ "}" }

array = {  "[" ~ WHITESPACE? ~ element_list? ~ WHITESPACE? ~ "]" }

element_list = _{value ~ (WHITESPACE? ~ "," ~ WHITESPACE? ~ value)*}

number = @{"-"? ~ int ~ double?}

int = { "0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT* }

double = { "." ~ ASCII_DIGIT+ }

boolean = { "true" | "false" }

null = { "null" }

pair_key_value = _{key_value ~ (WHITESPACE? ~ "," ~ WHITESPACE? ~ key_value)*}

key_value = { string ~ WHITESPACE* ~ ":" ~ WHITESPACE* ~ value  }

string = { "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

WHITESPACE = _{ (" " | "\t" | "\r" | "\n")+ }
```

## Grammar Rules Explanation 

**json**  
JSON must start at start-of-input (SOI) and end at end-of-input (EOI). No extra characters before or after the main value.

**value**  
A JSON value - one of: object, array, string, number, boolean (true/false), or null.

**object**  
Object - collection of "key": value pairs enclosed in {}. Pairs separated by commas.

**array**  
Array - ordered sequence of values enclosed in []. Values separated by commas, can be empty.

**pair_key_value**  
Set of "key": value pairs inside an object. Zero or more pairs, separated by commas.

**key_value**  
Single "key": value pair. Key is always a quoted string, followed by :, then any valid JSON value.

**number**  
Number - integer or floating-point, with optional minus sign.

**string**  
String - sequence of characters in double quotes "...".

**boolean**  
Boolean - literals true or false.

**null**  
`null`

---

## Railroad Diagram (Parsing Flow)
