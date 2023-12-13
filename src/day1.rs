use anyhow::Result;
use std::io;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let h = l.chars().find_map(|c| c.to_digit(10)).unwrap_or(0);
            let l = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(h);
            h * 10 + l
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    fn get_num(str: &str, rev: bool) -> Option<u32> {
        for (n, &w) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            if !rev {
                if str.starts_with(w) {
                    return Some(n as u32 + 1);
                }
            } else {
                let wr = w.chars().rev().collect::<String>();
                if str.starts_with(&wr) {
                    return Some(n as u32 + 1);
                }
            }
        }
        str.chars().nth(0)?.to_digit(10)
    }
    fn find_num(str: &str, rev: bool) -> u32 {
        if let Some(n) = get_num(str, rev) {
            n
        } else {
            find_num(&str[1..], rev)
        }
    }
    input
        .lines()
        .map(|l| {
            let n = find_num(l, false);
            let n2 = find_num(&l.chars().rev().collect::<String>(), true);
            n * 10 + n2
        })
        .sum()
}

pub fn run() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
