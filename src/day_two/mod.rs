use std::str::FromStr;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum Hand {
    A = 1,
    B = 2,
    C = 3,
}

impl FromStr for Hand {
    type Err = ();

    // X Y Z are for the first problem
    fn from_str(input: &str) -> Result<Hand, Self::Err> {
        match input {
            "A" | "X" => Ok(Hand::A),
            "B" | "Y" => Ok(Hand::B),
            "C" | "Z" => Ok(Hand::C),
            _ => Err(()),
        }
    }
}

enum GameResult {
    X = 0,
    Y = 3,
    Z = 6,
}

impl GameResult {
    fn from_hands(player: Hand, enemy: Hand) -> GameResult {
        if beats(player) == enemy {
            return GameResult::Z;
        }

        if player == enemy {
            return GameResult::Y;
        }

        GameResult::X
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(input: &str) -> Result<GameResult, Self::Err> {
        match input {
            "X" => Ok(GameResult::X),
            "Y" => Ok(GameResult::Y),
            "Z" => Ok(GameResult::Z),
            _ => Err(()),
        }
    }
}

fn beats<'a>(move_hand: Hand) -> Hand {
    return match move_hand {
        Hand::A => Hand::C,
        Hand::B => Hand::A,
        Hand::C => Hand::B,
    };
}

fn loses<'a>(move_hand: Hand) -> Hand {
    return match move_hand {
        Hand::A => Hand::B,
        Hand::B => Hand::C,
        Hand::C => Hand::A,
    };
}

fn get_round_score_one(player_move: Hand, enemy_move: Hand) -> usize {
    let expeted_game_score = GameResult::from_hands(player_move, enemy_move) as usize;

    let hand_score = player_move as usize;

    expeted_game_score + hand_score
}

fn get_round_score_two(player_result: GameResult, enemy_move: Hand) -> usize {
    let hand_score = match player_result {
        GameResult::X => beats(enemy_move) as usize,
        GameResult::Y => enemy_move as usize,
        GameResult::Z => loses(enemy_move) as usize,
    };

    let expeted_game_score = player_result as usize;

    expeted_game_score + hand_score
}

pub fn solution(input: &Vec<&str>) -> (usize, usize) {
    let mut score_one = 0usize;
    let mut score_two = 0usize;
    for play in input {
        let round: Vec<&str> = play.split(" ").collect();
        score_one += get_round_score_one(
            Hand::from_str(round[1]).unwrap(),
            Hand::from_str(round[0]).unwrap(),
        );
        score_two += get_round_score_two(
            GameResult::from_str(round[1]).unwrap(),
            Hand::from_str(round[0]).unwrap(),
        );
    }

    (score_one, score_two)
}
