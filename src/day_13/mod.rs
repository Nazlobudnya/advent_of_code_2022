#![allow(dead_code)]

use core::fmt;
use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum PacketValue {
    Single(usize),
    List(Vec<PacketValue>),
}

impl fmt::Debug for PacketValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl PacketValue {
    fn get_vec(&self) -> Vec<PacketValue> {
        match self {
            Self::List(n) => n.clone(),
            Self::Single(n) => vec![Self::Single(*n)],
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Single(a), Self::Single(b)) => a.partial_cmp(&b),
            (left, right) => {
                let l_vec = left.get_vec();
                let r_vec = right.get_vec();

                Some(l_vec.cmp(&r_vec))
                // Some(
                //     l_vec
                //         .iter()
                //         .zip(r_vec.iter())
                //         .map(|(a, b)| a.cmp(b))
                //         .find(|&x| x != Ordering::Equal)
                //         .unwrap_or_else(|| l_vec.len().cmp(&r_vec.len())),
                // )
            }
        }
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solution(input: String) -> (usize, usize) {
    let mut ans = 0usize;
    for (idx, groups) in input.split("\n\n").enumerate() {
        let mut packets = groups
            .lines()
            .map(|line| serde_json::from_str::<PacketValue>(line).unwrap());

        let left = packets.next().unwrap();
        let right = packets.next().unwrap();

        let is_correct = left < right;

        if is_correct {
            ans += idx + 1;
        }
    }

    let dividers = vec![
        PacketValue::List(vec![PacketValue::Single(2)]),
        PacketValue::List(vec![PacketValue::Single(6)]),
    ];

    let mut packets: Vec<PacketValue> = input
        .lines()
        .filter(|&s| !s.is_empty())
        .map(|item| serde_json::from_str::<PacketValue>(item).unwrap())
        .chain(dividers.clone())
        .collect();

    packets.sort();

    let div_idx_product = dividers
        .iter()
        .map(|div| packets.iter().position(|el| el == div).unwrap() + 1)
        .product::<usize>();

    (ans, div_idx_product)
}
