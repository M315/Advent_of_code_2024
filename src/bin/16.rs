use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::vec;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    direction: (i32, i32),
    score: u64,
    path_len: Vec<(usize, usize)>,
}

impl State {
    fn new(position: (usize, usize)) -> Self {
        Self {
            position,
            direction: (0, 1),
            score: 0,
            path_len: vec![position],
        }
    }

    fn map_pos(&self) -> (usize, usize, usize) {
        let dir_dimension = match self.direction {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => panic!("Invalid direction"),
        };
        (dir_dimension, self.position.0, self.position.1)
    }

    fn possible_moves(&self) -> Vec<Self> {
        vec![self.step(), self.rotate_clock(), self.rotate_counterclock()]
    }

    fn step(&self) -> Self {
        let new_position = (
            (self.position.0 as i32 + self.direction.0) as usize,
            (self.position.1 as i32 + self.direction.1) as usize,
        );
        let mut new_path = self.path_len.clone();
        new_path.push(new_position);
        Self {
            position: new_position,
            direction: self.direction,
            score: self.score + 1,
            path_len: new_path,
        }
    }

    fn rotate_clock(&self) -> Self {
        let direction = match self.direction {
            ( 0,  1) => ( 1,  0),
            ( 1,  0) => ( 0, -1),
            ( 0, -1) => (-1,  0),
            (-1,  0) => ( 0,  1),
            _ => panic!("Invalid direction"),
        };
        Self {
            position: self.position,
            direction,
            score: self.score + 1000,
            path_len: self.path_len.clone(),
        }
    }

    fn rotate_counterclock(&self) -> Self {
        let direction = match self.direction {
            ( 0,  1) => (-1,  0),
            (-1,  0) => ( 0, -1),
            ( 0, -1) => ( 1,  0),
            ( 1,  0) => ( 0,  1),
            _ => panic!("Invalid direction"),
        };
        Self {
            position: self.position,
            direction,
            score: self.score + 1000,
            path_len: self.path_len.clone(),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijktra(map: &Vec<Vec<bool>>, start: State, goal: (usize, usize)) -> Vec<State> {
    let mut heap = BinaryHeap::new();
    let mut dist = vec![vec![vec![u64::MAX; map[0].len()]; map.len()]; 4];

    let pos = start.map_pos();
    dist[pos.0][pos.1][pos.2] = 0;
    heap.push(start);

    let mut best = None;
    let mut paths = Vec::new();

    while let Some(state) = heap.pop() {
        if state.position == goal {
            best = Some(state.score);
            paths.push(state);
            continue;
        }

        let pos = state.map_pos();
        if dist[pos.0][pos.1][pos.2] < state.score || best.unwrap_or(u64::MAX) < state.score {
            continue;
        }

        // Move forward
        for next_state in state.possible_moves() {
            if !map[next_state.position.0][next_state.position.1] { continue; }
            
            let pos = next_state.map_pos();
            if next_state.score <= dist[pos.0][pos.1][pos.2] && next_state.score <= best.unwrap_or(u64::MAX) {
                dist[pos.0][pos.1][pos.2] = next_state.score;
                heap.push(next_state);
            }
        }
    }
    paths
}

fn parse(input: &str) -> (State, (usize, usize), Vec<Vec<bool>>) {
    let mut start = None;
    let mut goal = (0, 0);
    let map = input.lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = Some(State::new((i, j)));
                    } else if c == 'E' {
                        goal = (i, j);
                    }
                    if c == '#' {
                        false
                    } else {
                        true
                    }
                }).collect()
        }).collect();
    (start.unwrap(), goal, map)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (state, goal, map) = parse(input);
    dijktra(&map, state, goal).first().map(|s| s.score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (state, goal, map) = parse(input);
    Some(dijktra(&map, state, goal).iter()
        .map(|s| s.path_len.iter())
        .flatten()
        .collect::<HashSet<_>>()
        .len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
