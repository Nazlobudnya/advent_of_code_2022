use std::{env, fs};

mod day_one;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough args");
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("File is not there or unable to read");

    let mut contents: Vec<&str> = contents.split("\n").collect();

    let day_one_solution = day_one::solution(&mut contents);

    println!(
        "Elf with max calories: [{}] \nTop three elfs total: [{}]",
        day_one_solution.max_calories, day_one_solution.top_three_elfs
    );
}
