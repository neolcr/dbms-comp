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
    let mut start_buffer = false;
    let mut end_buffer = false;
    
    for ch in content.chars() {
        println!("Char: {}", ch);        
        if valid_symbols.contains(ch) {
            println!("El simbolo {} se guarda en el buffer y continue", ch.to_string());
            final_tokens_list.push(ch.to_string());
            buffer = String::new();
            continue;
        }
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
                if valid_tokens.contains(&buffer.to_ascii_uppercase()) || id_regex.is_match(&buffer) {
                    println!("Insertando valor: {}", &buffer);
                    final_tokens_list.push(buffer.to_string());
                    buffer = String::new();
                    start_buffer = false;
                    end_buffer = false;
                    continue;
                }
                else {
                    let error_message = format!("KEYWORD O PALABRA INVALIDA: {}", buffer);
                    panic!("{}", error_message);
                }
            }
        }
        if !valid_symbols.contains(ch) && ch.is_ascii_punctuation() {
             let error_message = format!("SIMBOLO INVALIDO: {}", ch);
             panic!("{}", error_message);
         
        }
        // descartados simbolos puedo meter en el buffer

        if ch.is_alphabetic() || ch.is_numeric() {
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
        
    }
    println!("######## MOSTRAR LA LISTA FINAL DE TOKENS #######"); 
    for tok in &final_tokens_list {
        println!("Token: {}", tok);
    }
}

#[allow(dead_code)]
fn contains_ignore_case(main: &str, sub: &str) -> bool {
    main.to_ascii_uppercase().contains(&sub.to_ascii_uppercase())
}





























