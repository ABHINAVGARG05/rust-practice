use std::fs;
use std::io::{self, Write};

fn main() {
    println!("Text Analyzer CLI");
    println!("-----------------");

    loop {
        print_menu();
        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let text = read_input("Enter a sentence: ");
                print_report(&text);
            }
            "2" => {
                let path = read_input("Enter file path: ");
                match fs::read_to_string(&path) {
                    Ok(content) => print_report(&content),
                    Err(err) => println!("Could not read file: {err}"),
                }
            }
            "3" => print_help(),
            "4" => {
                println!("Goodbye");
                break;
            }
            _ => println!("Invalid option. Pick 1, 2, 3, or 4."),
        }

        println!();
    }
}

fn print_menu() {
    println!("1) Analyze typed text");
    println!("2) Analyze text from file");
    println!("3) Help");
    println!("4) Exit");
}

fn print_help() {
    println!("This CLI reports:");
    println!("- first word");
    println!("- total word count");
    println!("- longest word");
    println!("- average word length");
}

fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn print_report(text: &str) {
    let word_count = count_words(text);

    if word_count == 0 {
        println!("No words found.");
        return;
    }

    let first = first_word(text).unwrap_or("(none)");
    let longest = longest_word(text).unwrap_or("(none)");
    let avg = average_word_length(text);

    println!("\nReport");
    println!("------");
    println!("First word: {first}");
    println!("Word count: {word_count}");
    println!("Longest word: {longest}");
    println!("Average word length: {avg:.2}");
}

fn first_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().next()
}

fn count_words(sentence: &str) -> usize {
    sentence.split_whitespace().count()
}

fn longest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace()
        .max_by_key(|word| word.chars().count())
}

fn average_word_length(sentence: &str) -> f64 {
    let words: Vec<&str> = sentence.split_whitespace().collect();

    if words.is_empty() {
        return 0.0;
    }

    let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();

    total_chars as f64 / words.len() as f64
}