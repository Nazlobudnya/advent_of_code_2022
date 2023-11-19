use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, Clone)]
struct Monkey<'a> {
    id: usize,
    items: VecDeque<usize>,
    operation_type: char,
    operation_value: &'a str,
    test_value: usize,
    throw_to: [usize; 2],
    inspected_items: usize,
}

impl<'a> PartialEq for Monkey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> PartialOrd for Monkey<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> Ord for Monkey<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

fn perform_op(old: usize, operand: char, value: &str) -> usize {
    let value = match value {
        "old" => old,
        x => x.parse::<usize>().unwrap(),
    };

    match operand {
        '*' => old * value,
        '+' => old + value,
        _ => panic!("Only {{*, +\\}} are permitted"),
    }
}

pub fn solution(input: String) -> usize {
    let monkeys = input.split("\n\n").collect::<Vec<&str>>();

    let mut monkey_map: HashMap<usize, Monkey> = HashMap::new();

    for (idx, &monkey) in monkeys.iter().enumerate() {
        let monkey_parts = monkey.split("\n").collect::<Vec<&str>>();

        let _id = idx;
        let items: VecDeque<usize> = monkey_parts[1].split(": ").collect::<Vec<&str>>()[1]
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&x| x.parse::<usize>().unwrap())
            .collect();

        let operation_members = monkey_parts[2].split("= ").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>();

        let test_value = monkey_parts[3].split("by ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let if_test_passed = monkey_parts[4].split("monkey ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let if_test_failed = monkey_parts[5].split("monkey ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let throw_to = [if_test_passed, if_test_failed];

        monkey_map.insert(
            idx,
            Monkey {
                id: idx,
                items,
                operation_type: operation_members[1].chars().nth(0).unwrap(),
                operation_value: operation_members[2],
                test_value,
                throw_to,
                inspected_items: 0,
            },
        );
    }

    // for monkey in monkey_map.values().sorted() {
    //     println!("{monkey:?}");
    // }

    let divisor_product: usize = monkey_map.values().map(|x| x.test_value).product();

    for _ in 0..10_000 {
        for i in 0..monkey_map.len() {
            let mc;

            {
                let monkey = monkey_map.get_mut(&i).unwrap();
                mc = monkey.clone();
                monkey.inspected_items += monkey.items.len();
            }

            for mut item in mc.items.iter().copied() {
                item %= divisor_product;
                item = perform_op(item, mc.operation_type, mc.operation_value);
                if item % mc.test_value == 0 {
                    monkey_map
                        .get_mut(&mc.throw_to[0])
                        .unwrap()
                        .items
                        .push_back(item);
                } else {
                    monkey_map
                        .get_mut(&mc.throw_to[1])
                        .unwrap()
                        .items
                        .push_back(item);
                }
            }

            monkey_map.get_mut(&i).unwrap().items.clear()
        }
    }

    // for monkey in monkey_map
    //     .values()
    //     .sorted_unstable_by(|&a, &b| b.inspected_items.cmp(&a.inspected_items))
    // {
    //     println!("{:?}", monkey);
    // }

    monkey_map
        .values()
        .map(|monkey| monkey.inspected_items)
        .sorted_by(|&a, &b| b.cmp(&a))
        .take(2)
        .product::<usize>()
}
