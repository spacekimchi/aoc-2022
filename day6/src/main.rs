use std::collections::HashMap;
use std::collections::VecDeque;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = std::fs::read_to_string("./input/input.txt")?;

    part1(&mut input, 4)?;
    part1(&mut input, 14)?;
    
    Ok(())
}

fn part1(input: &str, tar_len: usize) -> Result<()> {
    let mut q: VecDeque<char> = VecDeque::new();
    let mut hm: HashMap<char, u32> = HashMap::new();
    for (i, c) in input.chars().enumerate() {
        let count = hm.entry(c).or_insert(0);
        *count += 1;
        q.push_back(c);
        if q.len() > tar_len {
            let c = q.pop_front().unwrap();
            let count = hm.entry(c).or_insert(0);
            *count -= 1;
            if count <= &mut 0 {
                hm.remove(&c);
            }
        }
        if hm.len() == tar_len {
            println!("i: {}", i + 1);
            break;
        }
    }

    Ok(())
}
