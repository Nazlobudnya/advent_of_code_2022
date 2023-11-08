pub struct DayOneSolution {
    pub max_calories: usize,
    pub top_three_elfs: usize,
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    let mut count = Vec::<usize>::new();
    let mut temp_sum = 0usize;
    for i in input {
        if *i == "" {
            count.push(temp_sum);
            temp_sum = 0;
        } else {
            temp_sum += i.parse::<usize>().unwrap();
        }
    }

    count
}

fn max_calories(input: &Vec<&str>) -> usize {
    let vec = self::parse_input(input);

    *vec.iter().max().unwrap()
}

fn max_three_calories(input: &mut Vec<&str>) -> usize {
    let mut vec = self::parse_input(input);
    vec.sort();

    vec[vec.len() - 3..].iter().sum()
}

pub fn solution(mut input: Vec<&str>) -> DayOneSolution {
    let max_calories = max_calories(&input);

    let top_three_elfs = max_three_calories(&mut input);

    DayOneSolution {
        max_calories,
        top_three_elfs,
    }
}
