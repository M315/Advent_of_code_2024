use std::{cmp::Ordering, collections::{HashMap, HashSet}};

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
        let (graph, pages) = input.split_once("\n\n").unwrap();
        (
            graph.lines()
                .map(|pair| pair.split_once('|').unwrap())
                .map(|(a, b) | (a.trim().parse::<u32>().unwrap(), b.trim().parse::<u32>().unwrap()))
                .fold(HashMap::new(), |mut acc, (a, b)| {
                    acc.entry(a).or_insert(HashSet::new()).insert(b);
                    acc
                }),
            pages.lines()
                .map(|line| line.split(',').into_iter().map(|c| c.parse::<u32>().unwrap()).collect())
                .collect()
        )
}

fn custom_ordering(a: &u32, b: &u32, graph: &HashMap<u32, HashSet<u32>>) -> Ordering {
    if let Some(s) = graph.get(a) {
        for c in s.iter() {
            if *c == *b {
                return Ordering::Less;
            }
        }
    }
    if let Some(s) = graph.get(b) {
        for c in s.iter() {
            if *c == *a {
                return Ordering::Greater;
            }
        }
    }
    Ordering::Equal
}

fn mid_page(p: Vec<u32>, graph: &HashMap<u32, HashSet<u32>>, correct: bool) -> Option<u32> {
    let mut ordered_p = p.clone();
    ordered_p.sort_by(|a, b| custom_ordering(a, b, graph));
    if p == ordered_p {
        match correct {
            true  => Some(ordered_p[ordered_p.len() / 2]),
            false => None,
        }
    } else {
        match correct {
            false => Some(ordered_p[ordered_p.len() / 2]),
            true  => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, pages) = parse(input);
    Some(pages.into_iter()
        .filter_map(|p| mid_page(p, &graph, true))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, pages) = parse(input);
    Some(pages.into_iter()
        .filter_map(|p| mid_page(p, &graph, false))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
