#[allow(unused_imports)]
use std::{env, fs};

mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod day_six;

#[allow(dead_code)]
fn do_day_one() {
    println!("===DAY 1===\n");

    let contents = fs::read_to_string("./src/1.in").expect("File is not there or unable to read");

    let contents: Vec<&str> = contents.split("\n").collect();

    let day_one_solution = day_one::solution(contents);

    println!(
        "Elf with max calories: [{}] \nTop three elfs total: [{}]\n",
        day_one_solution.max_calories, day_one_solution.top_three_elfs
    );
}

#[allow(dead_code)]
fn do_day_two() {
    println!("===DAY 2===\n");

    let contents = fs::read_to_string("./src/2.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (day_two_solution_one, day_two_solution_two) = day_two::solution(contents);

    println!(
        "Play score first strat: [{}]. \nPlayer score second strat: [{}]\n",
        day_two_solution_one, day_two_solution_two
    )
}

#[allow(dead_code)]
fn do_day_three() {
    println!("===DAY 3===\n");

    let contents = fs::read_to_string("./src/3.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_three::solution(contents);

    println!(
        "Sum of prios: [{}]. \nPrios of elf groups of 3: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_four() {
    println!("===DAY 4===\n");

    let contents = fs::read_to_string("./src/4.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_four::solution(contents);

    println!(
        "Fully in range: [{}]. \nFully or partially in range: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_five() {
    unimplemented!()
}

fn do_day_six() {
    println!("===DAY 6===\n");
    let contents = fs::read_to_string("./src/6.in").expect("File is not there or unable to read");

    let (ans_one, ans_two) = day_six::solution(contents);

    println!("Message packet size 4: [{}]. \nMessage packet size 14: [{}]", ans_one, ans_two);
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     panic!("Not enough args");
    // }

    // let what = &args[1];


    
    do_day_one();

    do_day_two();

    do_day_three();

    do_day_four();

    do_day_six();

}
