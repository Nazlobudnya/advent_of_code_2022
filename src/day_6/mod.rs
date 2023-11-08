use std::time::Instant;

fn solve(input: &String, packet_size: usize) -> usize {
    let now: Instant = Instant::now();
    let bytes = input.as_bytes();

    let mut ans = 0usize;
    for i in 0..bytes.len() {
        let seq = bytes[i..i + packet_size].iter().fold(0u32, |acc, x| {
            let place = x - 'a' as u8;
            acc ^ 1u32 << place
        });

        if seq.count_ones() == packet_size as u32 {
            ans = i + packet_size;
            break;
        }
    }

    println!(
        "[{ans}] ver1 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );
    ans
}

fn solve_version_two(input: &String, packet_size: usize) -> usize {
    let now = Instant::now();

    let bytes = input.as_bytes();

    let mut ans = 0usize;
    for (idx, window) in bytes.windows(packet_size as usize).enumerate() {
        for w in window {
            let place = w - 'a' as u8;
            let num = 1usize << place;

            if ans & num == num {
                break;
            }

            ans = ans ^ num;
        }

        if ans.count_ones() == packet_size as u32 {
            ans = idx + packet_size;
            break;
        }

        ans = 0;
    }

    println!(
        "[{ans}] ver2 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );
    ans
}

fn solve_version_three(input: &String, packet_size: usize) -> usize {
    let now = Instant::now();

    let bytes = input.as_bytes();

    let ans = bytes
        .windows(packet_size)
        .position(|s| !(1..s.len()).any(|i| s[i..].contains(&s[i - 1])))
        .unwrap()
        + packet_size;

    println!(
        "[{ans}] ver3 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );

    ans
}

fn solve_version_four(input: &String, packet_size: usize) -> usize {
    let now = Instant::now();

    let bytes = input.as_bytes();

    let mut ans = 0usize;

    for i in 0..bytes.len() {
        let mut curr = 0u32;
        for ch in bytes[i..i + packet_size].iter() {
            let place = ch - 'a' as u8;
            let num = 1u32 << place;

            if curr & num == num {
                break;
            }

            curr = curr ^ num;
        }

        if curr.count_ones() == packet_size as u32 {
            ans = i + packet_size;
            break;
        }
    }

    println!(
        "[{ans}] ver4 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );
    ans
}

fn solve_version_five(i: &String, packet_size: usize) -> usize {
    let now = Instant::now();

    let ans = i
        .as_bytes()
        .windows(14)
        .position(|w| {
            let mut vec = Vec::with_capacity(packet_size);
            for x in w {
                if vec.contains(x) {
                    return false;
                }

                vec.push(*x);
            }
            return true;
        })
        .map(|x| x + packet_size)
        .unwrap();

    println!(
        "[{ans}] ver5 for packet_size: [{packet_size: >2}] | time: [{: >12?}]",
        now.elapsed()
    );
    ans
}

pub fn solution(input: String) -> (usize, usize) {
    solve(&input, 4);
    solve_version_two(&input, 4);
    solve_version_three(&input, 4);
    solve_version_four(&input, 4);
    solve_version_five(&input, 4);

    println!("");
    solve(&input, 14);
    solve_version_two(&input, 14);
    solve_version_three(&input, 14);
    solve_version_four(&input, 14);
    solve_version_five(&input, 14);

    println!("");

    (
        solve_version_four(&input, 4),
        solve_version_four(&input, 14),
    )
}
