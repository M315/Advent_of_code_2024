use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(23);

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    input.lines()
        .fold(HashMap::new(), |mut graph, line| {
            let (a, b) = line.split_once("-").unwrap();
            graph.entry(a.to_string()).or_insert_with(HashSet::new).insert(b.to_string());
            graph.entry(b.to_string()).or_insert_with(HashSet::new).insert(a.to_string());
            graph
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input);

    let mut ans = 0;
    let mut used = HashSet::new();
    for computer in graph.keys() {
        if !computer.starts_with("t") { continue; }
        let conections = graph.get(computer).unwrap();
        for con in conections.iter().combinations(2) {
            if used.contains(&vec![computer, con[0], con[1]]) { continue; }
            if graph.get(con[0]).unwrap().contains(con[1]) { 
                used.insert(vec![computer, con[0], con[1]]);
                used.insert(vec![computer, con[1], con[0]]);
                used.insert(vec![con[0], computer, con[1]]);
                used.insert(vec![con[1], computer, con[0]]);
                used.insert(vec![con[0], con[1], computer]);
                used.insert(vec![con[1], con[0], computer]);
                ans += 1;
            }
        }
    }
    Some(ans)
}

fn are_connected(graph: &HashMap<String, HashSet<String>>, mut con: Vec<&String>) -> bool {
    while let Some(c1) = con.pop() {
        for c2 in con.iter() {
            if !graph.get(c1).unwrap().contains(*c2) { return false; }
        }
    }
    true
}

fn find_lan(graph: &HashMap<String, HashSet<String>>, k: usize) -> Option<Vec<String>> {
    for computer in graph.keys() {
        let conections = graph.get(computer).unwrap();
        for con in conections.iter().combinations(k) {
            if are_connected(graph, con.clone()) { 
                let mut ans = vec![computer.to_string()];
                let mut extention = con.iter().map(|s| s.to_string()).collect();
                ans.append(&mut extention);
                return Some(ans);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input);
    let mut left = 1;
    let mut right = graph.keys().len();
    while left < right {
        let mid = (left + right) / 2;
        let sim = find_lan(&graph, mid);
        match sim {
            Some(_) => left = mid + 1,
            None => right = mid,
        }
    }
    let mut ans = find_lan(&graph, left - 1).unwrap();
    ans.sort_unstable();
    println!("{}", ans.join(","));
    Some(ans.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
