use std::io::{self, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    highest_n_calories(&input, 1)?;
    highest_n_calories(&input, 3)?;

    Ok(())
}

fn highest_n_calories(input: &str, n: usize) -> Result<()> {
    let mut sum: u32 = 0;
    let mut sums: Vec<u32> = Vec::new();
    for line in input.lines() {
        if !line.trim().is_empty() {
            sum += line.trim().parse::<u32>().unwrap();
        } else {
            sums.push(sum);
            sums.sort_by(|a, b| b.cmp(a));
            sum = 0;
        }
        while sums.len() > n {
            sums.pop();
        }
    }
    writeln!(io::stdout(), "{}", sums.iter().sum::<u32>())?;
    Ok(())
}
