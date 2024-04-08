use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_name: &str;

    if args.len() < 4 {
        file_name = &args[2];

        let size = get_file_total_bytes(&file_name.to_string());
        let lines_count = get_file_lines_count(&file_name.to_string());
        let words_count = get_file_words_count(&file_name.to_string());

        println!("{} {} {} {}", size, lines_count, words_count, file_name);

        return ();
    }

    file_name = &args[3];
    let command = &args[2];

    if command == "-c" {
        let size = get_file_total_bytes(&file_name.to_string());

        println!("{} {}", size, file_name);
    }

    if command == "-l" {
        let lines_count = get_file_lines_count(&file_name.to_string());

        println!("{:?} {}", lines_count, file_name);
    }

    if command == "-w" {
        let words_count: usize = get_file_words_count(&file_name.to_string());

        println!("{} {}", words_count, file_name);
    }

    if command == "-m" {
        let chars_count = get_file_chars_count(&file_name.to_string());

        println!("{} {}", chars_count, file_name);
    }

    ()
}

fn get_file_total_bytes(file_name: &String) -> u64 {
    fs::metadata(&file_name).unwrap().len()
}

fn get_file_words_count(file_name: &String) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().split_whitespace().count())
        .sum()
}

fn get_file_lines_count(file_name: &String) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines().count()
}

fn get_file_chars_count(file_name: &String) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().count())
        .sum()
}
