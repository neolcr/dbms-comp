use core::panic;
use std::fs;
use std::env;
use regex::Regex;
use log::{debug, error, info};
use env_logger;

fn main() {
    env_logger::init(); 
    env::set_var("RUST_LOG", "debug");


    match env::current_dir() {
        Ok(path) => debug!("Current directory: {}", path.display()),
        Err(error) => debug!("Error: {}", error),
    }

    match env::current_exe() {
        Ok(exe) => debug!("Current directory exe: {}", exe.display()),
        Err(error) => debug!("Error: {}", error),
    }

    let file_path = "src/sql/query.sql";
        
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|e| {
            debug!("Error reading file : {}", e);
            panic!("ERROR TRYING TO READ THE FILE");
        });
    
    let lista_fase1 = analisis_lexico_fase1(&content);
    info!("Lista fase 1 : {:?}", lista_fase1);
    let lista_final = analisis_lexico_fase2(lista_fase1);
    info!("{:?}", lista_final);

}

#[allow(dead_code)]
fn analisis_lexico_fase0(content: &String) -> String {
    // reemplazar todos los espacios consecutivos por un solo espacio 
    let mut result = String::new();
    let mut anterior_espacio: bool = false;

    for ch in content.chars() {
        if !anterior_espacio && ch.is_whitespace() {
            result.push(' ');
            anterior_espacio = true;
        } else if !ch.is_whitespace() {
            anterior_espacio = false;
            result.push(ch);
        }         

    }
    result

}

fn analisis_lexico_fase1(content: &String) -> String {
    let mut result = String::new();
    let valid_symbols = "*()<>=,;.";
    for ch in content.chars() {
        if valid_symbols.contains(ch) {
            result.push(' ');
            result.push(ch);
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    result
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

    let valid_symbols = "*()<>=,.";
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
        debug!("Buffer: {}", buffer);
        j = j + 1;
        some_ch = content.chars().nth(j);

        ch = match some_ch {
            Some(c) => c, 
            None => ' ',
        };
        debug!("{}", buffer);
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
    debug!("Retornar string: {}", buffer);
    (j, buffer)
}


fn analisis_lexico_fase2(content: String) -> Vec<String>  {
    debug!("El contenido: {}", content); 
//    let id_regex = Regex::new(r"^[a-zA-Z]+\.[a-zA-Z0-9]*$").unwrap();

    let id_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
    debug!("{:?}", id_regex);

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
            Tipo::FIN => debug!("Se alcanza el fin"),
            Tipo::PuntoComa => {
                debug!("Punto y coma");
                final_tokens_list.push(ch.to_string());
            }
            Tipo::Espacio =>  {
                debug!("Es un espacio");
            }
            Tipo::InicioString => {
                debug!("Inicio de string");
                let (j, seg) = extraer_string(i, &content);
                i = j;
                final_tokens_list.push(seg);

            }
            Tipo::InicioKeyword =>{
                debug!("Inicio de keyword");
                let (j, seg) = extraer_keyword(i, &content);
                i = j;
                final_tokens_list.push(seg);
                let next = get_next(i, &content);
                debug!("next: {}", next);
                if next == ';' {
                    final_tokens_list.push(next.to_string());
                }
            }
            Tipo::SimboloValido => {
                debug!("Simbolo valido");
                final_tokens_list.push(ch.to_string());
            }
            Tipo::SimboloInvalido => {
                error!("Simbolo invalido");
                panic!("SIMBOLO INVALIDO:  '{}'", ch);
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
    fn analisis_lexico_fase2_test() {
        let test_cases = vec![
            (String::from("SELECT * FROM TABLA1;"), vec!["SELECT","*","FROM","TABLA1",";"]),
            (String::from("select * from tabla1    ;"), vec!["SELECT","*","FROM","TABLA1", ";"]),
            (String::from("select * from tabla1 inner join tabla2;"), vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", ";"]),
            (String::from("select * from tabla1 inner join tabla2 ;"),vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", ";"]),
            (String::from("select * from tabla1 inner join tabla2 where name = 'fulano' ;"),vec!["SELECT","*","FROM","TABLA1","INNER", "JOIN", "TABLA2", "WHERE", "NAME","=", "'fulano'", ";"]),
        ];
        for case in test_cases {
            assert_eq!(analisis_lexico_fase2(case.0), case.1);
        }
    }
}

