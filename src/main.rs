use core::panic;
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
fn lexic_analysis(mut content: String) {
    content.insert_str(0, " ");
    println!("El contenido: {}", content); 

    let id_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    println!("{:?}", id_regex);

    let valid_tokens = vec![
        "SELECT".to_string(),"FROM".to_string(),"INNER".to_string(),"JOIN".to_string()
    ];
    
    let valid_symbols = ";*()<>=,.";

    let mut final_tokens_list: Vec<String> = Vec::new(); 
    let mut buffer = String::new(); 
    let mut found_keyword: bool = false;
    let mut found_symbol: bool = false;
    let mut found_whitespace: bool = false;
    let mut found_identifier: bool = false;
    let mut start_buffer = false;
    let mut end_buffer = false;
 
    
    for ch in content.chars() {
        println!("Char: {}", ch);        
        if ch.is_ascii_whitespace() {
            println!("Detectado espacio en blanco");
            if !start_buffer {
                println!("Empiezo a llenar el buffer");
                start_buffer = true;
                end_buffer = true;
                buffer = String::new();
                continue;
            }
            if end_buffer {
                println!("Hora de mirar el buffer {}", buffer.to_string());
                let is_match = id_regex.is_match(&buffer);
                println!("is match {}", is_match.to_string());
                if valid_tokens.contains(&buffer.to_ascii_uppercase()) || id_regex.is_match(&buffer) {
                    println!("Insertando valor: {}", &buffer);
                    final_tokens_list.push(buffer.to_string());
                    buffer = String::new();
                    start_buffer = false;
                    end_buffer = false;
                    continue;
                }
                else {
                    //panic!("KEYWORD INVALIDO");

                }
            }
        }
        if !valid_symbols.contains(ch) && ch.is_ascii_punctuation() {
             let error_message = format!("SIMBOLO INVALIDO: {}", ch);
             panic!("{}", error_message);
         
        }
        // descartados simbolos puedo meter en el buffer

        if ch.is_alphabetic() {
           if !start_buffer {
                println!("Detecto nuevo caracter valido: empiezo a llenar el buffer");
                start_buffer = true;
                end_buffer = true;
                buffer = String::new();
            }

        }
        
        if start_buffer {
            buffer.push(ch);
            println!("Buffer: {}", buffer);


        }
        if valid_symbols.contains(ch) {
            println!("El simbolo {} se guarda en el buffer y continue", ch.to_string());
            final_tokens_list.push(ch.to_string());
            buffer = String::new();
            continue;
        }
        
    }
    println!("######## MOSTRAR LA LISTA FINAL DE TOKENS #######"); 
    for tok in &final_tokens_list {
        println!("Token: {}", tok);
    }
}



#[allow(unused_variables)]
#[allow(dead_code)]
fn lexic_analysis2(content: String) {
    println!("El contenido: {}", content); 

    let id_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    println!("{:?}", id_regex);

    let tokens = vec![
        Token::Keyword("SELECT".to_string()),
        Token::Keyword("FROM".to_string()),
        Token::Keyword("INNER".to_string()),
        Token::Keyword("JOIN".to_string()),
        Token::Identifier(id_regex),
    ];
    
    let mut final_tokens_list: Vec<String> = Vec::new(); 
    let mut buffer = String::new(); 
    let mut found_keyword: bool = false;
    let mut found_symbol: bool = false;
    let mut found_whitespace: bool = false;
    let mut found_identifier: bool = false;
    let valid_symbols = ";*()<>=,.";
    
    for ch in content.chars() {
        println!("{}", ch);
        buffer.push(ch);
        println!("Buffer: {}", buffer);
        for token in &tokens {
            if !found_identifier {
                if let Token::Keyword(value) = token {
                    if contains_ignore_case(value,&buffer) {
                        //println!("Keyword {} contains {}", value, &buffer.to_ascii_uppercase());
                    }
                    if value.eq_ignore_ascii_case(&buffer){
                        println!("Se encontro el keyword: {}", value.to_ascii_uppercase());
                        final_tokens_list.push(buffer.to_string());
                        buffer = String::new();
                    }
                    found_keyword = true;
                }
                if valid_symbols.contains(ch) {
                    println!("Se encontro el simbolo: {}", ch);
                    final_tokens_list.push(ch.to_string());
                    buffer = String::new();
                    found_symbol = true;
                }
                if ch.is_whitespace() {
                   buffer = String::new();
                   found_whitespace = true;
                }
                if !valid_symbols.contains(ch) && ch.is_ascii_punctuation() {
                     let error_message = format!("SIMBOLO INVALIDO: {}", ch);
                     panic!("{}", error_message);
                 
                }
                /*if let Token::Symbol(value) = token {
                     if ch == *value {
                        println!("Se encontro el simbolo: {}", ch);
                        final_tokens_list.push(ch.to_string());
                        buffer = String::new();
                        found_symbol = true;
                     }
                     else if ch.is_whitespace() {
                        //println!("Found whitespace");
                        buffer = String::new();
                        found_whitespace = true;
                     }
                     else {
                         let error_message = format!("SIMBOLO INVALIDO: {}", ch);
                         panic!("{}", error_message);
                     }
                }*/
            }
            
            if !found_symbol && !found_whitespace && !found_keyword {

                if let Token::Identifier(regex) = token {
                   if regex.is_match(&buffer) {
                        println!("{} es un identifier valido", &buffer);
                        found_identifier = true;
                   } else if !&buffer.is_empty() {
                        final_tokens_list.push(buffer.to_string());
                        println!("{} NO es un identifier valido", &buffer);
                        buffer = String::new(); 
                        found_identifier = false;
                   }
                }

            }
            found_whitespace = false;
            found_keyword = false;
            found_symbol = false;

        }
    }
  
    println!("######## MOSTRAR LA LISTA FINAL DE TOKENS #######"); 
    for tok in &final_tokens_list {
        println!("Token: {}", tok);
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
































