use std::io::{self, Read, Write};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (opp, me) = line.split_once(" ").unwrap();
        if me == "X" {
            score += 1;
            score += match opp {
                "A" => 3,
                "C" => 6,
                _ => 0
            };
        } else if me == "Y" {
            score += 2;
            score += match opp {
                "B" => 3,
                "A" => 6,
                _ => 0
            };
        } else if me == "Z" {
            score += 3;
            score += match opp {
                "C" => 3,
                "B" => 6,
                _ => 0
            };
        }
    }

    println!("{}", score);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut score: u32 = 0;
    for line in input.lines() {
        let (opp, me) = line.split_once(" ").unwrap();
        if me == "X" {
            score += match opp {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0
            };
        } else if me == "Y" {
            score += 3;
            score += match opp {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 0
            };
        } else if me == "Z" {
            score += 6;
            score += match opp {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => 0
            };
        }
    }

    println!("{}", score);
    Ok(())
}
