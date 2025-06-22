use regex::Regex;
use std::collections::HashMap;


/*

 Enum to define all the standar library posible types in VIG,
 helps to make the code more readable and easier to make operations.

 */

#[allow(dead_code)]
#[derive(Debug)]
enum Types<'a> {
    Int(i32),
    Str(String),
    Float(f32),
    Operator(&'a str),
    Keyword(&'a str),
    Name(&'a str),
}

/*

This function gets a string and a regex and returns a vector of the tokens.
The tokens are every type of the enum, divided by any character established on the regex.

eg:
input = "a = 7 + 2"
output = ["a", "=", "7", "+", "2"]

this function will be used whenever we need to get tokens so then we can match it with their type.

*/
fn get_tokens<'a>(code: &'a str, re: &Regex) -> Vec<&'a str> {
    re.find_iter(code).map(|m| m.as_str()).collect()
}



/*

This function should parse every token into an enum type,
so then it's easier to work with it. This function gets a
vector of the tokens and returns an option of a vector of the types.

Why an option? because if there any syntax error it will return an Err(e),
so we make sure that we don't have any errors on the execution time.


eg:
input = "a = 7 + 2"

output = [
    Name("a"),
    Operator("="),
    Int(7),
    Operator("+"),
    Int(2)
]

*/
fn get_data_types(tokens: Vec<&str>) -> Result<Vec<Types>, String> {
    let mut values: Vec<Types> = vec![];

    for val in tokens {
        
        // checking if it is a number and assiging it to an i32
        if val.chars().all(|c| c.is_numeric()) {
            let value = val.parse::<i32>().unwrap();
            
            {   
                values.push(Types::Int(value));
            }
        }
        
    }
    Ok(values)
}


/*
    This function gets a vector of the data types and return the total of some operation,
    to sum up this function will return the result of value of the right-hand side (RHS).
    

    eg:
    input = "a = 7 + 2"
    
    output = 9

    this function will keep all the tokens after the equal operator, and resolve the operation.
    this function will be used whenever we need to allocate any value.

 */
#[allow(dead_code, unused_variables)]
fn simplify_expression(values: Vec<Types>) -> i32 {
       

    0
}

fn main() {
    
    let mut variables: HashMap<String, Types> = HashMap::new();

    loop {
        let mut string = String::new();

        println!("ingrese su codigo en VIG");

        std::io::stdin()
        .read_line(&mut string)
        .expect("Error al leer la entrada");
        
        string = string.trim().to_string();

        if string == "exit()" {
            break;
        }
        
        // regex which filters special characters
        let re = Regex::new(r"[^ =()+\-*/{}]+|[=()+\-*/{}]").unwrap();
        
        let tokens = get_tokens(string.as_str(), &re);

        dbg!(&tokens);

        if tokens.len() == 0 {
            println!("Error al leer la entrada");
            continue;
        }
        
        for token in tokens.iter() {
            match *token {
                "if" => {}
                "for" => {}
                "while" => {}
                _ => {
                    if tokens.len() < 3 {
                        println!("Error al leer la entrada");
                        continue;
                    }

                    // we check if the second token is =
                    // if it is we assign the value to the variable
                    if tokens[1] == "=" {
                        // name of variable
                        let var: &str = tokens[0];
                        // value of variable
                        let value = tokens[2];
                        
                        let types = get_data_types(vec![value]);

                        let total = 0;


                        variables.insert((*var).to_string().clone(), Types::Int(total));

                        dbg!(&types);
                        break
                        
                    }
                }
            }

        }
         
    }

    dbg!(&variables);
}
