use std::collections::HashMap;

advent_of_code::solution!(22);

const MOD: u128 = 16777216;

fn parse(input: &str) -> Vec<u128> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn next_num(num: &mut u128) {
    *num = ((*num * 64) ^ *num).rem_euclid(MOD);
    *num = ((*num / 32) ^ *num).rem_euclid(MOD);
    *num = ((*num * 2048) ^ *num).rem_euclid(MOD);
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut nums = parse(input);
    for _ in 0..2000 {
        nums.iter_mut().for_each(|num| next_num(num));
    }
    Some(nums.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = parse(input);
    
    // Get the secuences
    let mut seq: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); 2000]; nums.len()];
    let mut curr: Vec<i32> = nums.iter().map(|num| num.rem_euclid(10) as i32).collect();
    for i in 0..2000 {
        nums.iter_mut().for_each(|num| next_num(num));
        for (j, num) in nums.iter().enumerate() {
            let n = num.rem_euclid(10) as i32;
            seq[j][i] = (n - curr[j], n);
            curr[j] = n;
        }
    }

    // Map
    let mut map: HashMap<Vec<i32>, Vec<Option<i32>>> = HashMap::new();
    seq.into_iter()
        .enumerate()
        .for_each(|(i, s)| {
            s.windows(4)
                .for_each(|w| {
                    let (k, v): (Vec<i32>, Vec<i32>) = w.iter()
                        .cloned()
                        .unzip();
                    map.entry(k)
                        .and_modify(|e| { 
                            if e[i].is_none() { e[i] = Some(*v.last().unwrap()) }
                         })
                        .or_insert({
                            let mut new_v = vec![None; nums.len()];
                            new_v[i] = Some(*v.last().unwrap());
                            new_v
                        });
                });
        });

    map.values()
        .map(|v| v.into_iter().map(|x| x.unwrap_or(0)).sum::<i32>() as u64)
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
