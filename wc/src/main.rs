use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        println!("Error getting args");
        return ();
    }

    let command = &args[2];
    let file_name = &args[3];

    if command == "-c" {
        let size = fs::metadata(file_name).unwrap().len();

        println!("{} {}", size, file_name);
    }

    if command == "-l" {
        let file = File::open(file_name).unwrap();
        let buffered = BufReader::new(file);

        println!("{:?} {}", buffered.lines().count(), file_name);
    }

    ()
}
