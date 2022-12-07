type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = std::fs::read_to_string("./input/input.txt")?;
    part1(&mut input)?;
    part2(&mut input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let (header, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];
    let mut lines = header.lines().rev();
    lines.next();
    for line in lines {
         for (i, c) in line.chars().enumerate() {
             if c.is_alphabetic() {
                stacks[i/4].push(c);
             }
         }
    }

    let moves: Vec<Vec<&str>> = instructions
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .filter(|word| {
                    word.chars().nth(0).unwrap().is_numeric()
                }).collect()
        }).collect();
    for m in moves {
        let amount: usize = m[0].parse().unwrap();
        let source: usize = m[1].parse().unwrap();
        let dest: usize = m[2].parse().unwrap();

        for _ in 0..amount {
            let val = stacks[source-1].pop().unwrap();
            stacks[dest-1].push(val);
        }
    }

    println!("{:?}", stacks);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (header, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ];
    let mut lines = header.lines().rev();
    lines.next();
    for line in lines {
         for (i, c) in line.chars().enumerate() {
             if c.is_alphabetic() {
                stacks[i/4].push(c);
             }
         }
    }

    let moves: Vec<Vec<&str>> = instructions
        .lines()
        .map(|line| {
            line
                .split_whitespace()
                .filter(|word| {
                    word.chars().nth(0).unwrap().is_numeric()
                }).collect()
        }).collect();
    for m in moves {
        let amount: usize = m[0].parse().unwrap();
        let source: usize = m[1].parse().unwrap();
        let dest: usize = m[2].parse().unwrap();

        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..amount {
            let val = stacks[source-1].pop().unwrap();
            
            tmp.push(val);
        }
        while !tmp.is_empty() {
            let val = tmp.pop().unwrap();
            stacks[dest-1].push(val);
        }
    }

    println!("{:?}", stacks);
    Ok(())
}
