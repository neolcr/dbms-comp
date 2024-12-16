use std::fs;
use std::env;
use regex::Regex;

fn main() {
    
    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(error) => eprintln!("Error: {}", error),
    }

    match env::current_exe() {
        Ok(exe) => println!("Current directory exe: {}", exe.display()),
        Err(error) => eprintln!("Error: {}", error),
    }

    let file_path = "src/sql/query.sql";
        
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file : {}", e);
            panic!("ERROR TRYING TO READ THE FILE");
        });

    lexic_analysis(content);

}

#[allow(unused_variables)]
fn lexic_analysis(content: String) {
    println!("The content: {}", content); 

    let id_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    println!("{:?}", id_regex);

    let tokens = vec![
        Token::Keyword("SELECT".to_string()),
        Token::Symbol('*'),
        Token::Keyword("FROM".to_string()),
        Token::Identifier(id_regex),
        Token::Symbol(';'),

    ];
    
    let mut final_tokens_list: Vec<String> = Vec::new(); 
    let mut buffer = String::new(); 
    let mut found_keyword: bool = false;
    let mut found_symbol: bool = false;
    let mut found_whitespace: bool = false;

    for ch in content.chars() {
        println!("{}", ch);
        buffer.push(ch);
        println!("Buffer: {}", buffer);
        for token in &tokens {
            if let Token::Keyword(value) = token {
                if contains_ignore_case(value,&buffer) {
                    //println!("Keyword {} contains {}", value, &buffer.to_ascii_uppercase());
                }
                if value.eq_ignore_ascii_case(&buffer){
                    println!("Found the whole word: {}", value.to_ascii_uppercase());
                    final_tokens_list.push(buffer.to_string());
                    buffer = String::new();
                }
                found_keyword = true;
            }
            if let Token::Symbol(value) = token {
                 if ch.is_whitespace() {
                    println!("Found whitespace");
                    buffer = String::new();
                    found_whitespace = true;
                 }
                 else if ch == *value {
                    println!("Found symbol: {}", ch);
                    final_tokens_list.push(ch.to_string());
                    buffer = String::new();
                    found_symbol = true;
                 }
            }
            if !found_symbol && !found_whitespace && !found_keyword {

                if let Token::Identifier(regex) = token {
                   if regex.is_match(&buffer) {
                        println!("{} is a valid identifier", &buffer);
                   } else if !&buffer.is_empty() {
                        final_tokens_list.push(buffer.to_string());
                        println!("{} is NOT a valid identifier", &buffer);
                        buffer = String::new(); 
                   }
                }

            }
            found_whitespace = false;
            found_keyword = false;
            found_symbol = false;

        }
    }
    
    println!("Show final list of tokens"); 
    for tok in &final_tokens_list {
        println!("Final token: {}", tok);
    }
    
    
}


fn contains_ignore_case(main: &str, sub: &str) -> bool {
    main.to_ascii_uppercase().contains(&sub.to_ascii_uppercase())

}


#[derive(Debug)]
enum Token {
    Identifier(Regex),
    Number(i32),
    Operator(String),
    Keyword(String),
    Symbol(char),
    Whitespace
}
































