# JSONParser

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