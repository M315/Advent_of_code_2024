use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(8);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn add_antenas(a: (i32, i32), b: (i32, i32), map: &mut Vec<Vec<char>>, recursive: bool, legs: (bool, bool)) {
    let x = a.0 - b.0;
    let y = a.1 - b.1;

    if legs.0 && a.0 + x >= 0  && a.0 + x < map.len() as i32 && a.1 + y >= 0 && a.1 + y < map[0].len() as i32 {
        map[(a.0 + x) as usize][(a.1 + y) as usize] = '#';
        if recursive { add_antenas((a.0 + x, a.1 + y), a, map, recursive, (true, false)); }
    }

    if legs.1 && b.0 - x >= 0  && b.0 - x < map.len() as i32 && b.1 - y >= 0 && b.1 - y < map[0].len() as i32 {
        map[(b.0 - x) as usize][(b.1 - y) as usize] = '#';
        if recursive { add_antenas(b, (b.0 - x, b.1 - y), map, recursive, (false, true)); }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let mut coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '.' {
                coords.entry(map[i][j]).and_modify(|v| v.push((i as i32, j as i32))).or_insert(vec![(i as i32, j as i32)]);
            }
        } 
    }
    for (_, v) in coords {
        for comb in v.into_iter().combinations(2) {
            add_antenas(comb[0], comb[1], &mut map, false, (true, true));
        }
    }
    Some(map.into_iter()
        .map(|row| row.into_iter().filter(|c| *c == '#').count() as u32)
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let mut coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '.' {
                coords.entry(map[i][j]).and_modify(|v| v.push((i as i32, j as i32))).or_insert(vec![(i as i32, j as i32)]);
            }
        } 
    }
    for (_, v) in coords {
        for comb in v.into_iter().combinations(2) {
            add_antenas(comb[0], comb[1], &mut map, true, (true, true));
        }
    }
    Some(map.into_iter()
        .map(|row| row.into_iter().filter(|c| *c != '.').count() as u32)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
