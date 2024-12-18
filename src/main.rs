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

    let lista_final = analisis_lexico(content);
    println!("{:?}", lista_final);

}

enum Tipo {
    Espacio,
    InicioString,
    InicioKeyword,
    SimboloValido,
    SimboloInvalido,
    PuntoComa,
    FIN
}

fn get_tipo(ch: char) -> Tipo {

    let valid_symbols = ";*()<>=,.";
    if ch == ';' {
        return Tipo::PuntoComa;
    }
    if ch.is_whitespace() {
        return Tipo::Espacio
    }
    if ch == '\'' {
        return Tipo::InicioString;
    }
    if ch.is_alphanumeric() || ch.is_numeric() {
        return Tipo::InicioKeyword;
    }
    if valid_symbols.contains(ch) {
        return Tipo::SimboloValido;
    }
    if !valid_symbols.contains(ch) && ch.is_ascii_punctuation() {
        return Tipo::SimboloInvalido;
    }
    Tipo::FIN 
}

fn extraer_keyword(i: usize, content: &String) -> (usize, String) {
    let mut j: usize = i;

    let mut some_ch = content.chars().nth(i);

    let mut ch = match some_ch {
        Some(c) => c, 
        None => ' ',
    };
    let mut buffer = String::new();

    while !ch.is_whitespace() && ch != ';' {
        buffer.push(ch);    
        println!("Buffer: {}", buffer);
        j = j + 1;
        some_ch = content.chars().nth(j);

        ch = match some_ch {
            Some(c) => c, 
            None => ' ',
        };
        println!("{}", buffer);
    }
    (j, buffer.to_uppercase())
}

fn extraer_string(i: usize, content: &String) -> (usize, String) {
    let mut j: usize = i;
    let mut buffer = String::new();
    
    let mut some_ch = content.chars().nth(i);
    let mut ch = match some_ch {
        Some(c) => c, 
        None => ' ',
    };
    buffer.push(ch);

    j = j + 1;
    some_ch = content.chars().nth(j);
    ch = match some_ch {
        Some(c) => c, 
        None => ' ',
    };
    buffer.push(ch);    

    while ch != '\'' {
        j = j + 1;
        if j == content.len() - 1 {
            panic!("NO se encuentra fin de comillas en 'string'");
        }
        some_ch = content.chars().nth(j);

        ch = match some_ch {
            Some(c) => c, 
            None => ' ',
        };
        buffer.push(ch);    
    }
    println!("Retornar string: {}", buffer);
    (j, buffer)
}


fn analisis_lexico(mut content: String) -> Vec<String>  {
    println!("El contenido: {}", content); 

    let id_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    println!("{:?}", id_regex);

    let valid_keyword = vec![
        "SELECT".to_string(),"FROM".to_string(),"INNER".to_string(),"JOIN".to_string()
    ];
    

    let mut final_tokens_list: Vec<String> = Vec::new(); 
    let mut i: usize = 0;
    
    while i <= content.len() + 1 {
        let some_ch = content.chars().nth(i);
        
        let ch = match some_ch {
            Some(c) => c,
            None => ' '  
        };


        let tipo: Tipo = get_tipo(ch);
        match tipo {
            Tipo::FIN => println!("Se alcanza el fin"),
            Tipo::PuntoComa => {
                println!("Punto y coma");
                final_tokens_list.push(ch.to_string());
            }
            Tipo::Espacio =>  {
                println!("Es un espacio");
            }
            Tipo::InicioString => {
                println!("Inicio de string");
                let (j, seg) = extraer_string(i, &content);
                i = j;
                final_tokens_list.push(seg);

            }
            Tipo::InicioKeyword =>{
                println!("Inicio de keyword");
                let (j, seg) = extraer_keyword(i, &content);
                i = j;
                if !valid_keyword.contains(&seg) && !id_regex.is_match(&seg) {
                    panic!("IDENTIFICADOR INVALIDO");
                }
                final_tokens_list.push(seg);
                let next = get_next(i, &content);
                println!("next: {}", next);
                if next == ';' {
                    final_tokens_list.push(next.to_string());
                }
            }
            Tipo::SimboloValido => {
                println!("Simbolo valido");
                final_tokens_list.push(ch.to_string());
            }
            Tipo::SimboloInvalido => {
                println!("Simbolo invalido");
                panic!("SIMBOLO INVALIDO");
            }
        }
        i = i + 1;
    }

    final_tokens_list
}

fn get_next(i: usize, content: &String) -> char {
    let some_ch = content.chars().nth(i);
        
    let ch = match some_ch {
        Some(c) => c,
        None => ' '  
    };
    ch
}

#[allow(dead_code)]
fn contains_ignore_case(main: &str, sub: &str) -> bool {
    main.to_ascii_uppercase().contains(&sub.to_ascii_uppercase())
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analisis_lexico_test() {
        let test_cases = vec![
            (String::from("SELECT * FROM TABLA1;"), vec!["SELECT","*","FROM","TABLA1",";"]),
            (String::from("select * from tabla1    ;"), vec!["SELECT","*","FROM","TABLA1", ";"]),
            (String::from("select * from tabla1 inner join tabla2;"), vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", ";"]),
            (String::from("select * from tabla1 inner join tabla2 ;"),vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", ";"]),
            (String::from("select * from tabla1 inner join tabla2 where name = 'fulano' ;"),vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", "WHERE", "NAME","=", "'fulano'", ";"]),
        ];
        for case in test_cases {
            assert_eq!(analisis_lexico(case.0), case.1);
        }
    }
}

