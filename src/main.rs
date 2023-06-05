use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate regex;
use regex::Regex;

struct Argument {
    arg_type: String,
    arg_name: String,
}

struct Function {
    object: String,
    name: String,
    largs: Vec<Argument>,
}

fn parse_cpp_function_syntax(input: &str) -> Option<Function> {
    let function_pattern = r"\w+::(\w+)\(([^)]*)\)";

    let arg_pattern = r"([\w:]+(?:\s*\*|&)?)(?:\s*\*)?\s*(\w+)\s*";


    let function_regex = Regex::new(function_pattern).unwrap();

    let arg_regex = Regex::new(arg_pattern).unwrap();

    let captures = function_regex.captures(input)?;
    // println!("Captures: {:?}", captures);

    let object_name = captures.get(1)?.as_str().to_string();
    let function_name = captures.get(0)?.as_str().to_string();

    let arg_list = captures.get(2)?.as_str();
    let mut largs = Vec::new();
    for captures in arg_regex.captures_iter(arg_list) {
        let arg_type = captures.get(1)?.as_str().replace(" ", "").to_string();
        let arg_name = captures.get(2)?.as_str().to_string();

        largs.push(Argument {
            arg_type,
            arg_name,
        });
    }

    Some(Function {
        object: object_name,
        name: function_name,
        largs,
    })
}

fn main() {
    let file = File::open("input.cpp").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let input: String = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    if let Some(function) = parse_cpp_function_syntax(&input) {
        let mut args_json = String::new();
        for arg in function.largs {
            let arg_json = format!(
                "{{ \"type\": \"{}\", \"arg\": \"{}\" }},",
                arg.arg_type, arg.arg_name
            );
            args_json.push_str(&arg_json);
        }

        if !args_json.is_empty() {
            args_json.pop();
        }

        let json_output = format!(
            "{{ \"data\": {{\n    \"object\": \"function\",\n    \"name\": \"{}::{}\",\n    \"largs\": [{}]\n}} }}",
            function.object, function.name, args_json
        );
        println!("{}", json_output);
    }
}
