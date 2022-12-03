fn get_char_prio(ch: char) -> usize {
    if 'a' <= ch && ch <= 'z' {
        ch as usize - 'a' as usize + 1
    } else {
        ch as usize - 'A' as usize + 1 + 26
    }
}

fn find_prios_of_same_letters(input: &str) -> usize {
    let mut ans = 0usize;

    let half = input.len() / 2;
    let first = input.chars().take(half);
    let second = input.chars().rev().take(half).collect::<Vec<char>>();

    for ch in first {
        if second.iter().position(|&x| x == ch).is_some() {
            ans += get_char_prio(ch);
            break;
        }
    }

    ans
}

fn find_prios_group_of_three(group: &[&str]) -> usize {
    let (first, second, third) = (
        group[0].chars(),
        group[1].chars().collect::<Vec<char>>(),
        group[2].chars().collect::<Vec<char>>(),
    );

    let mut ans = 0usize;
    for ch in first {
        let is_in_second = second.iter().position(|&x| x == ch).is_some();
        let is_in_third = third.iter().position(|&x| x == ch).is_some();

        if is_in_second && is_in_third {
            ans += get_char_prio(ch);
            break;
        }
    }

    ans
}

pub fn solution(input: Vec<&str>) -> (usize, usize) {
    let mut ans_one = 0usize;
    let mut ans_two = 0usize;

    for sack in &input {
        ans_one += find_prios_of_same_letters(sack);
    }

    for elf_group in input.chunks(3) {
        ans_two += find_prios_group_of_three(elf_group);
    }

    (ans_one, ans_two)
}
