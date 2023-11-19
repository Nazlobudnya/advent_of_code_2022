use std::collections::HashSet;

fn advance_matrix_pos(pos: (isize, isize), direction: &char) -> (isize, isize) {
    let (x, y);

    match direction {
        'U' => {
            x = pos.0;
            y = pos.1 - 1;
        }
        'R' => {
            x = pos.0 + 1;
            y = pos.1;
        }
        'D' => {
            x = pos.0;
            y = pos.1 + 1;
        }
        'L' => {
            x = pos.0 - 1;
            y = pos.1;
        }
        _ => panic!("Direction could only be U R D L"),
    };

    (x, y)
}

#[derive(PartialEq)]
enum MOVES {
    NoMove,
    Move,
}

fn should_move(head_pos: (isize, isize), tail_pos: (isize, isize)) -> MOVES {
    let x_diff = head_pos.0.abs_diff(tail_pos.0);
    let y_diff = head_pos.1.abs_diff(tail_pos.1);

    if x_diff + y_diff > 2 || (x_diff > 1 || y_diff > 1) {
        return MOVES::Move;
    }

    MOVES::NoMove
}

pub fn solution(input: String) -> usize {
    let mut hs: HashSet<(isize, isize)> = HashSet::new();

    let base_pos = (11isize, 15isize);
    let mut tails = [base_pos; 9];

    let mut head_pos = base_pos;

    hs.insert(base_pos);

    let steps: Vec<(char, usize)> = input
        .split('\n')
        .map(|item| {
            (
                item.as_bytes()[0] as char,
                (&item[2..]).parse::<usize>().unwrap(),
            )
        })
        .collect();

    for &(direction, num_moves) in steps.iter() {
        for _ in 0..num_moves {
            let new_head_pos = advance_matrix_pos(head_pos, &direction);
            head_pos = new_head_pos;

            let mut new_tail_pos = new_head_pos;

            for curr_tail_pos in tails.iter_mut() {
                let move_type = should_move(new_tail_pos, curr_tail_pos.clone());

                match move_type {
                    MOVES::NoMove => {
                        break;
                    }
                    MOVES::Move => {
                        if new_tail_pos.0 > curr_tail_pos.0 {
                            curr_tail_pos.0 += 1;
                        }

                        if new_tail_pos.0 < curr_tail_pos.0 {
                            curr_tail_pos.0 -= 1;
                        }

                        if new_tail_pos.1 > curr_tail_pos.1 {
                            curr_tail_pos.1 += 1;
                        }

                        if new_tail_pos.1 < curr_tail_pos.1 {
                            curr_tail_pos.1 -= 1;
                        }
                    }
                }

                new_tail_pos = curr_tail_pos.clone();
            }

            hs.insert(tails.last().unwrap().clone());
        }
    }

    hs.len()
}

#[allow(dead_code)]
pub fn solution_a(input: String) -> usize {
    let mut hs: HashSet<(isize, isize)> = HashSet::new();

    let (mut tail_x, mut tail_y) = (0isize, 0isize);
    let (mut head_x, mut head_y) = (0isize, 0isize);
    hs.insert((tail_x, tail_y));

    let steps = input.split('\n');

    for step in steps {
        let (direction, num_moves): (char, usize) = (
            step.as_bytes()[0] as char,
            (&step[2..]).parse::<usize>().unwrap(),
        );

        for _ in 0..num_moves {
            let (new_head_x, new_head_y) = advance_matrix_pos((head_x, head_y), &direction);
            if new_head_x.abs_diff(tail_x) > 1 || new_head_y.abs_diff(tail_y) > 1 {
                tail_x = head_x;
                tail_y = head_y;

                hs.insert((tail_x, tail_y));
            }

            head_x = new_head_x;
            head_y = new_head_y;
        }
    }

    hs.len()
}
