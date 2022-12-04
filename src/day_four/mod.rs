macro_rules! intersect_full {
    ($start1:expr,$end1:expr,$start2:expr,$end2:expr) => {{
        ($start1 <= $start2 && $end1 >= $end2) || ($start2 <= $start1 && $end2 >= $end1)
    }};
}

// you have to satisfy all arms
macro_rules! intersect_partial {
    ($start1:expr,$end1:expr,$start2:expr,$end2:expr) => {{
        if ($start2 <= $start1 && $start1 <= $end2) || ($start2 <= $end1 && $end1 <= $end2) {
            true
        } else if ($start1 <= $start2 && $start2 <= $end1) || ($start1 <= $end2 && $end2 <= $end1) {
            true
        } else {
            false
        }
    }};
}

pub fn solution(input: Vec<&str>) -> (usize, usize) {
    let mut ans_one = 0usize;
    let mut ans_two = 0usize;

    for group in input {
        let split_group: Vec<&str> = group.split(",").collect();

        let r1: Vec<usize> = split_group[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let r2: Vec<usize> = split_group[1]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        // macros just because I wanted to try
        if intersect_partial!(r1[0], r1[1], r2[0], r2[1]) {
            ans_two += 1;
        }

        if intersect_full!(r1[0], r1[1], r2[0], r2[1]) {
            ans_one += 1;
        }
    }

    (ans_one, ans_two)
}
