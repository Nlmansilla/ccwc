use std::env;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Arguments {
    filename: String,
    options: String
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut opts = "-clm".to_string();

    if args[1].chars().nth(0).unwrap() == '-' {
        opts = args[1].clone()
    }

    let arguments: Arguments = Arguments {
        filename: args[ args.len() -1 ].clone(),
        options: opts
    };
    
    let mut answer: String = "".to_owned();

    

    for caracter in arguments.options.chars().skip(1) {
        match caracter {
            'c' =>  answer.push_str(&get_bytes_count(&arguments.filename).to_string()),
            'l' => answer.push_str(&(" ".to_owned() + &get_lines_count(&arguments.filename).to_string())),
            'w' => answer.push_str(&(" ".to_owned() + &get_words_count(&arguments.filename).to_string())),
            'm' => answer.push_str(&(" ".to_owned() + &get_characters_count(&arguments.filename).to_string())),
            _ => ()
        }
    }

    answer.push_str( &(" ".to_owned() + &arguments.filename) );

    println!("{:?}", answer);
}

fn get_bytes_count(filename: &String) -> usize{
    let file = File::open("./".to_owned() + filename).unwrap();
    let mut reader = BufReader::with_capacity(512, file);

    let mut total_bytes_in_file = 0;
    loop {
        let buffer = reader.fill_buf().unwrap();
        let buffer_length: usize = buffer.len();

        // No hay más bytes para leer.
        if buffer_length == 0 {
            break;
        }

        // Procesa los bytes leídos aquí.
        //println!("{:?}",buffer_length);
        total_bytes_in_file += buffer_length;

        // Consumir todos los bytes del buffer.
        reader.consume(buffer_length);
    }

    total_bytes_in_file
}

fn get_lines_count(filename: &String) -> usize {
    let file = File::open("./".to_owned() + filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().count()
}

fn get_words_count(filename: &String) -> usize {
    let file = File::open("./".to_owned() + filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_words: usize = 0;

    for line in reader.lines() {
        total_words += line.unwrap().split_whitespace().count();
    }

    total_words
}

fn get_characters_count(filename: &String) -> usize {

    let total_characters: usize = fs::read_to_string(
        "./".to_owned() + filename
    ).unwrap().chars().count();

    total_characters
}