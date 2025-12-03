#![allow(unused_imports)]
use std::cmp::Reverse;

#[test]
fn day_3_lobby() {
    println!("{}", include_str!("../3/input").lines().map(|s|
        (11..=99).rev().find(|&n| s
            .find(((n / 10) as u8 + b'0') as char)
            .zip(s.rfind(((n % 10) as u8 + b'0') as char))
            .is_some_and(|(a, b)| a < b)
        ).unwrap_or(0)).sum::<u32>());
}

#[test]
fn day_3_lobby_part_two() {
    println!("{}", include_str!("../3/input").lines().map(|s| (0..12)
        .fold((s, 0u64), |(slice, acc), i| slice
            .char_indices()
            .take(slice.len() + i - 11)
            .min_by_key(|&(_, c)| Reverse(c))
            .map(|(idx, c)| (&slice[idx + 1..], acc * 10 + (c as u64 & 0xF)))
            .unwrap()).1
    ).sum::<u64>());
}
