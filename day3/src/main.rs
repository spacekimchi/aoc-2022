use std::collections::HashSet;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = std::fs::read_to_string("./input/input.txt")?;
    
    part1(&mut input)?;
    part2(&mut input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let lines = input.lines();
    let mut ans: u32 = 0;
    for line in lines {
        let n = line.len();
        let (f, s) = line.split_at(n/2);
        let mut char_set: HashSet<char> = HashSet::new();
        for fc in f.chars() {
            char_set.insert(fc);
        }
        for sc in s.chars() {
            if char_set.contains(&sc) {
                let cap = sc.is_uppercase();
                ans += sc as u32 - if cap { 'A' as u32 } else { 'a' as u32 } + 1;
                ans += if cap { 26 } else { 0 };
                char_set.remove(&sc);
            }
        }
    }

    println!("{}", ans);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut ans: u32 = 0;
    let arr: Vec<&str> = input.split('\n').collect();
    let mut it = arr.chunks(3);

    while let Some(chunk) = it.next() {
        let mut count: [u32; 53] = [0; 53];
        for line in chunk {
            let mut char_set: HashSet<char> = HashSet::new();
            for c in line.chars() {
                if !char_set.contains(&c) {
                    let cap = c.is_uppercase();
                    let mut num = c as u32 - if cap { 'A' as u32 } else { 'a' as u32 } + 1;
                    num += if cap { 26 } else { 0 };
                    count[num as usize] += 1;
                    if count[num as usize] == 3 {
                        ans += num;
                    }
                    char_set.insert(c);
                }
            }
        }
    }

    println!("{}", ans);
    Ok(())
}
