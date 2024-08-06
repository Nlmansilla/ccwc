use std::env;
use std::fmt::Write;
use std::fs;
use std::io;

const FMT_DISPLAY_WIDTH: usize = 6;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum CliOptions {
    CountBytes,
    CountCharacters,
    CountLines,
    CountWords,
    MaxLineLength,
}

fn get_default_options() -> Vec<CliOptions> {
    vec![
        CliOptions::CountLines,
        CliOptions::CountWords,
        CliOptions::CountBytes,
    ]
}

fn main() {
    let mut files: Vec<String> = vec![];
    let mut read_stdin = false;
    let mut options: Vec<CliOptions> = vec![];

    for arg in env::args().skip(1).into_iter() {
        match arg.as_str() {
            "-c" | "--bytes" => add_option(CliOptions::CountBytes, &mut options),
            "-m" | "--chars" => add_option(CliOptions::CountCharacters, &mut options),
            "-l" | "--lines" => add_option(CliOptions::CountLines, &mut options),
            "-L" | "--max_line_length" => add_option(CliOptions::MaxLineLength, &mut options),
            "-w" | "--words" => add_option(CliOptions::CountWords, &mut options),
            "-" => read_stdin = true,
            "--help" => help(),
            "--version" => version(),
            s if s.starts_with("--files0-from=") => add_input_files(s, &mut files),
            file => files.push(file.to_string()),
        }
    }

    options = if options.is_empty() {
        get_default_options()
    } else {
        options
    };

    handle_stdin_or_empty_file(read_stdin, &files, &options);

    let mut total: Vec<usize> = vec![];

    for file in files.iter() {
        if let Ok(metadata) = fs::metadata(file) {
            let contents = if metadata.is_file() {
                fs::read_to_string(file).expect(&format!("unable to read {}", file))
            } else {
                println!("ccwc {} is a directory", file);
                "".to_string()
            };

            let counts = process_wc_options(&contents, &options);
            println!("{}{:>FMT_DISPLAY_WIDTH$}", wc_format_output(&counts), file);

            if total.is_empty() {
                total = counts;
            } else {
                total = total.iter().zip(&counts).map(|(&t, &c)| t + c).collect();
            }
        } else {
            println!("ccwc: {}: No such file or directory", file);
        }
    }

    print_total(&files, &total);
}

fn print_total(files: &Vec<String>, total: &Vec<usize>) {
    if files.len() > 1 {
        println!("{}total", wc_format_output(&total));
    }
}

fn handle_stdin_or_empty_file(read_stdin: bool, files: &Vec<String>, options: &Vec<CliOptions>) {
    if read_stdin || files.is_empty() {
        let file = if read_stdin { "-" } else { "" };
        let contents = io::read_to_string(io::stdin()).expect("Unable to read from stdin");
        println!(
            "{}{:>FMT_DISPLAY_WIDTH$}",
            wc_format_output(&process_wc_options(&contents, &options)),
            file
        );
    }
}

fn process_wc_options(contents: &String, options: &Vec<CliOptions>) -> Vec<usize> {
    let mut counts: Vec<usize> = vec![];

    for option in options {
        match option {
            CliOptions::CountBytes => counts.push(contents.bytes().len()),
            CliOptions::CountCharacters => counts.push(contents.chars().count()),
            CliOptions::CountLines => counts.push(contents.lines().count()),
            CliOptions::CountWords => counts.push(
                contents
                    .lines()
                    .map(|line| line.split_whitespace().count())
                    .sum(),
            ),
            CliOptions::MaxLineLength => {
                counts.push(contents.lines().map(|line| line.len()).max().unwrap_or(0))
            }
        }
    }

    counts
}

fn add_input_files(option: &str, files: &mut Vec<String>) {
    let (_, input) = option.split_at("--files0-from=".len());
    let contents = if input == "-" {
        io::read_to_string(io::stdin()).expect("Failed to read from stdin")
    } else {
        fs::read_to_string(input).expect(&format!("Failed to read from {}", input))
    };
    files.extend(contents.split('\0').map(String::from));
}

fn add_option(option: CliOptions, options: &mut Vec<CliOptions>) {
    options.push(option);
    options.dedup();
}

fn wc_format_output(counts: &Vec<usize>) -> String {
    let mut fmt = "".to_string();
    let width = if counts.len() > 1 {
        FMT_DISPLAY_WIDTH
    } else {
        0
    };
    for count in counts {
        write!(fmt, "{:>width$} ", count).expect("Failed to write bytes");
    }
    fmt
}

fn version() {
    println!(
        r#"ccwc 0.1.0
Copyright (C) 2024 <nlmansilla89@gmail.com>
License MIT: The MIT License <https://opensource.org/license/mit>
This is free software: you are free to change and redistribute it.
The software is provided “as is”, without warranty of any kind.

Written by Nicolas Mansilla"#
    );
}

fn help() {
    println!(
        r#"ccwc - print newline, word, and byte counts for each file

Usage: ccwc [OPTIONS]... [FILE]...
   or: ccwc [OPTIONS]... --file0-from=F

Description:

Print newline, word and byte counts for each FILE, and total line
if more than one FILE is specified. A word is a non-zero-length sequence
of characters delimited by white space.

With no FILE or when FILE is - read standard input

The options below may be used to select which counts are printed, always 
in the following order: newline, word, character, byte, maximum line length.

Options: 
    -c, --bytes             Print the byte counts
    -m, --chars             Print the character counts
    -l, --lines             Print the newline counts
        --files0-from=F     Read input from the files specified by
                              NUL-terminated names in file F;
                              If F is - then read names from standard input
    -L, --max-line-length   Print the maximum display width
    -w, --words             Print the word counts
        --help              Display this help and exit 
        --version           Output version information and exit"#
    )
}
