use anyhow::{Context, Result};
use std::{io, str::FromStr};

#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    set: Vec<Cubes>,
}

impl Game {
    fn possible(&self, cubes: &Cubes) -> bool {
        self.set.iter().all(|c| cubes.contains(c))
    }
    
    fn min(&self) -> Cubes {
        Cubes{
            red: self.set.iter().map(|c| c.red).max().unwrap_or(0),
            green: self.set.iter().map(|c| c.green).max().unwrap_or(0),
            blue: self.set.iter().map(|c| c.blue).max().unwrap_or(0),
        }
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tokens = s.split(":");
        let id = tokens.next().unwrap()
            .split_whitespace().nth(1).unwrap()
            .parse::<u32>()?;
        let mut cubes = Cubes::default();
        let set = tokens.next().unwrap()
            .split(';').map(|c| {
                let mut cubes = Cubes::default();
                for p in c.split(","){
                    let mut pair = p.split_whitespace();
                    let count = pair.next().unwrap().parse::<u32>()?;
                    let color = pair.next().unwrap();
                    match color {
                        "red" => cubes.red = count,
                        "green" => cubes.green = count,
                        "blue" => cubes.blue = count,
                        _ => return Err(anyhow::anyhow!("Invalid color: {}", color)),
                    }
                }
                Ok(cubes)
            }).collect::<Result<Vec<_>>>()?;
        Ok(Self { id, set })
    }
}

fn part1(games: &[Game]) -> u32 {
    let mut cubes = Cubes::new(12, 13, 14);
    let mut possable_games = games.iter().filter(|g| g.possible(&cubes));
    possable_games.map(|g| g.id).sum()
}

fn part2(games: &[Game]) -> u32 {
    games.iter().map(|g| g.min().power()).sum()
}

pub fn run() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    let games = input
        .lines()
        .map(|l| l.parse::<Game>())
        .collect::<Result<Vec<_>>>()?;
    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));
    Ok(())
}
