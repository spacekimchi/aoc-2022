type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = std::fs::read_to_string("./input/input.txt")?;
    part1(&mut input)?;
    part2(&mut input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines = input.split('\n').map(|line| {
        if line.is_empty() { return 0 }
        let (e1, e2) = line.split_once(',').unwrap();
        if contains(&e1, &e2) { 1 } else { 0 }
    });

    println!("{}", lines.sum::<u32>());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let lines = input.split('\n').map(|line| {
        if line.is_empty() { return 0 }
        let (e1, e2) = line.split_once(',').unwrap();
        if contains(&e1, &e2) || overlaps(&e1, &e2) || overlaps(&e2, &e1) { 1 } else { 0 }
    });

    println!("{}", lines.sum::<u32>());

    Ok(())

}

fn contains(c1: &str, c2: &str) -> bool {
    let t1: (u32, u32) = c1.split_once('-').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();
    let t2: (u32, u32) = c2.split_once('-').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();

    (t1.0 >= t2.0 && t1.1 <= t2.1) || (t2.0 >= t1.0 && t2.1 <= t1.1)
}

fn overlaps(c1: &str, c2: &str) -> bool {
    let t1: (u32, u32) = c1.split_once('-').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();
    let t2: (u32, u32) = c2.split_once('-').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();
    
    (t1.0 <= t2.0 && t1.1 >= t2.0) || (t1.0 <= t2.1 && t1.1 >= t2.0)
}
