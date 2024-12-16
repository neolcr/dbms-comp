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
    
    // Iterar el contenido y extraer los tokens
    let mut buffer = String::new(); 
    for ch in content.chars() {
        println!("{}", ch);
        buffer.push(ch);
        println!("Buffer: {}", buffer);
        for token in &tokens {
            if let Token::Keyword(value) = token {
                if value.contains(&buffer.to_ascii_uppercase()) {
                    println!("Keyword {} contains {}", value, &buffer.to_ascii_uppercase());
                }
            }
        }
    }
    
    
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
































