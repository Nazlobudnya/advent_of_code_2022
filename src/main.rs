#[allow(unused_imports)]
use std::{env, fs};

mod day_four;
mod day_one;
mod day_three;
mod day_two;

fn do_day_one() {
    let contents = fs::read_to_string("./src/1.in").expect("File is not there or unable to read");

    let contents: Vec<&str> = contents.split("\n").collect();

    let day_one_solution = day_one::solution(contents);

    println!(
        "DAY ONE \nElf with max calories: [{}] \nTop three elfs total: [{}]\n",
        day_one_solution.max_calories, day_one_solution.top_three_elfs
    );
}

fn do_day_two() {
    let contents = fs::read_to_string("./src/2.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (day_two_solution_one, day_two_solution_two) = day_two::solution(contents);

    println!(
        "DAY TWO \nPlay score first strat: [{}]. \nPlayer score second strat: [{}]\n",
        day_two_solution_one, day_two_solution_two
    )
}

fn do_day_three() {
    let contents = fs::read_to_string("./src/3.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_three::solution(contents);

    println!(
        "DAY THREE \nSum of prios: [{}]. \nPrios of elf groups of 3: [{}]\n",
        ans_one, ans_two
    )
}

fn do_day_four() {
    let contents = fs::read_to_string("./src/4.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_four::solution(contents);

    println!(
        "DAY FOUR \nFully in range: [{}]. \nFully or partially in range: [{}]\n",
        ans_one, ans_two
    )
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     panic!("Not enough args");
    // }

    // let file_path = &args[1];

    do_day_one();

    do_day_two();

    do_day_three();

    do_day_four();
}
