use itertools::Itertools;
use std::collections::HashMap;

#[allow(unused_imports)]
use std::{env, fs};

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

mod day_2;
mod day_3;
mod day_4;
mod day_6;
mod day_9;

fn get_input_file_name(day: usize, is_test: bool) -> String {
    format!("./src/inputs/{day}.{}", if is_test { "test" } else { "in" })
}

#[allow(dead_code)]
fn do_day_one(input: String) {
    let input: Vec<&str> = input.split("\n").collect();

    let day_one_solution = day_1::solution(input);

    println!(
        "Elf with max calories: [{}] \nTop three elfs total: [{}]\n",
        day_one_solution.max_calories, day_one_solution.top_three_elfs
    );
}

#[allow(dead_code)]
fn do_day_two(input: String) {
    let input: Vec<&str> = input.split("\n").collect();

    let (day_two_solution_one, day_two_solution_two) = day_2::solution(input);

    println!(
        "Play score first strat: [{}]. \nPlayer score second strat: [{}]\n",
        day_two_solution_one, day_two_solution_two
    )
}

#[allow(dead_code)]
fn do_day_three(input: String) {
    let input: Vec<&str> = input.split("\n").collect();

    let (ans_one, ans_two) = day_3::solution(input);

    println!(
        "Sum of prios: [{}]. \nPrios of elf groups of 3: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_four(input: String) {
    let input: Vec<&str> = input.split("\n").collect();

    let (ans_one, ans_two) = day_4::solution(input);

    println!(
        "Fully in range: [{}]. \nFully or partially in range: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_six(input: String) {
    let (ans_one, ans_two) = day_6::solution(input);

    println!(
        "Message packet size [ 4]: [{}]. \nMessage packet size [14]: [{}]",
        ans_one, ans_two
    );
}

#[allow(dead_code)]
fn do_day_nine(input: String) {
    let ans = day_9::solution(input);

    println!("Tail visited: [{ans}] unique positions",);
}

#[allow(dead_code)]
fn do_day_ten(input: String) {
    let ans = day_10::solution(input);

    println!("\nSum of singal strenghts: [{ans}]",);
}

fn do_day_eleven(input: String) {
    let ans = day_11::solution(input);

    println!("\n Monkey: [{ans}]",);
}

fn do_day_twelve(input: String) {
    let ans = day_12::solution(input);

    println!("\n Shortest path: [{ans}]",);
}

fn do_day_thirteen(input: String) {
    let (sum_of_correct, product_of_div_idx) = day_13::solution(input);

    println!("\n Sum of correct packet indexes: [{sum_of_correct}]. Product of divider idexes [{product_of_div_idx}]",);
}

fn do_day_fourteen(input: String) {
    let ans = day_14::solution(input);

    println!("\n Sand stops at {ans}",);
}

fn main() {
    let mut hm: HashMap<usize, fn(input: String) -> ()> = HashMap::new();
    hm.insert(1, do_day_one);
    hm.insert(2, do_day_two);
    hm.insert(3, do_day_three);
    hm.insert(4, do_day_four);
    hm.insert(6, do_day_six);
    hm.insert(9, do_day_nine);
    hm.insert(10, do_day_ten);
    hm.insert(11, do_day_eleven);
    hm.insert(12, do_day_twelve);
    hm.insert(13, do_day_thirteen);
    hm.insert(14, do_day_fourteen);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough args");
    }

    let what = &args.get(1);

    if what.is_none() {
        eprintln!("Day is not supplied. Possible values: [all, {{day_number}}]");
    }

    let is_test = args.get(2);

    let is_test = if is_test.is_none() {
        eprintln!("Running personal input");
        false
    } else {
        match is_test.unwrap().as_str() {
            "t" => true,
            "test" => true,
            _ => false,
        }
    };

    let what = what.unwrap();

    if what == "all" {
        for (&day_num, &func) in hm.iter().sorted() {
            println!("===DAY {day_num}===\n");

            let contents = fs::read_to_string(get_input_file_name(day_num, is_test))
                .expect("File is not there or unable to read");

            func(contents);
        }
    } else {
        let day_num = what.parse::<usize>().unwrap();
        if let Some(func) = hm.get(&day_num) {
            println!("===DAY {day_num}===\n");

            let contents = fs::read_to_string(get_input_file_name(day_num, is_test))
                .expect("File is not there or unable to read");

            func(contents);
        } else {
            unimplemented!("Nothing for day ${what}");
        }
    }
}
