use itertools::Itertools;
use std::collections::HashMap;

#[allow(unused_imports)]
use std::{env, fs};

mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_6;
mod day_9;

#[allow(dead_code)]
fn do_day_one() {
    let contents = fs::read_to_string("./src/1.in").expect("File is not there or unable to read");

    let contents: Vec<&str> = contents.split("\n").collect();

    let day_one_solution = day_1::solution(contents);

    println!(
        "Elf with max calories: [{}] \nTop three elfs total: [{}]\n",
        day_one_solution.max_calories, day_one_solution.top_three_elfs
    );
}

#[allow(dead_code)]
fn do_day_two() {
    let contents = fs::read_to_string("./src/2.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (day_two_solution_one, day_two_solution_two) = day_2::solution(contents);

    println!(
        "Play score first strat: [{}]. \nPlayer score second strat: [{}]\n",
        day_two_solution_one, day_two_solution_two
    )
}

#[allow(dead_code)]
fn do_day_three() {
    let contents = fs::read_to_string("./src/3.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_3::solution(contents);

    println!(
        "Sum of prios: [{}]. \nPrios of elf groups of 3: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_four() {
    let contents = fs::read_to_string("./src/4.in").expect("File is not there or unable to read");
    let contents: Vec<&str> = contents.split("\n").collect();

    let (ans_one, ans_two) = day_4::solution(contents);

    println!(
        "Fully in range: [{}]. \nFully or partially in range: [{}]\n",
        ans_one, ans_two
    )
}

#[allow(dead_code)]
fn do_day_six() {
    let contents = fs::read_to_string("./src/6.in").expect("File is not there or unable to read");

    let (ans_one, ans_two) = day_6::solution(contents);

    println!(
        "Message packet size [ 4]: [{}]. \nMessage packet size [14]: [{}]",
        ans_one, ans_two
    );
}

#[allow(dead_code)]
fn do_day_nine() {
    let contents = fs::read_to_string("./src/9.in").expect("File is not there or unable to read");

    let ans = day_9::solution(contents);

    println!("Tail visited: [{ans}] unique positions",);
}

#[allow(dead_code)]
fn do_day_ten() {
    let contents = fs::read_to_string("./src/10.in").expect("File is not there or unable to read");

    let ans = day_10::solution(contents);

    println!("\nSum of singal strenghts: [{ans}]",);
}

#[allow(dead_code)]
fn do_day_eleven() {
    let contents =
        fs::read_to_string("./src/11.test").expect("File is not there or unable to read");

    let ans = day_11::solution(contents);

    println!("\n Monkey: [{ans}]",);
}

fn main() {
    let mut hm: HashMap<usize, fn() -> ()> = HashMap::new();
    hm.insert(1, do_day_one);
    hm.insert(2, do_day_two);
    hm.insert(3, do_day_three);
    hm.insert(4, do_day_four);
    hm.insert(6, do_day_six);
    hm.insert(9, do_day_nine);
    hm.insert(10, do_day_ten);
    hm.insert(11, do_day_eleven);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough args");
    }

    let what = &args[1];

    if what == "all" {
        for (&day_num, &f) in hm.iter().sorted() {
            println!("===DAY {day_num}===\n");

            f();
        }
    } else {
        if let Some(elem) = hm.get(&what.parse::<usize>().unwrap()) {
            println!("===DAY {what}===\n");
            elem();
        } else {
            unimplemented!("Nothing for day ${what}");
        }
    }
}
