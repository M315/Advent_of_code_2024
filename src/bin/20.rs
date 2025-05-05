use std::collections::HashMap;

advent_of_code::solution!(20);

static DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(input: &str) -> (Vec<Vec<bool>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input.lines()
        .enumerate()
        .map(|(i, line)| line.chars().enumerate().map(|(j, c)| match c {
            '#' => false,
            '.' => true,
            'S' => {
                start = (i, j);
                true
            },
            'E' => {
                end = (i, j);
                true
            }
            _ => panic!("Invalid character: {}", c),
        }).collect())
        .collect();
    (map, start, end)
}

fn bfs(map: &Vec<Vec<bool>>, start: (usize, usize)) -> Vec<Vec<u64>> {
    let mut queue = vec![start];
    let mut dp = vec![vec![u64::MAX; map[0].len()]; map.len()];
    dp[start.0][start.1] = 0;
    let mut distance = 1;

    while !queue.is_empty() {
        let mut next_queue = Vec::new();
        for (x, y) in queue {
            for (dx, dy) in DIRECTIONS.iter() {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                if new_x < map.len() && new_y < map[0].len() && map[new_x][new_y] && dp[new_x][new_y] > distance {
                    dp[new_x][new_y] = distance;
                    next_queue.push((new_x, new_y));
                }
            }
        }
        queue = next_queue;
        distance += 1;
    }
    dp
}

fn cheat_directions(size: isize, x: isize, y: isize) -> Vec<((isize, isize), u64)> {
    let mut distance = 0;
    let mut curr = HashMap::new();
    curr.insert((x, y), 0);

    for _ in 0..size {
        distance += 1;
        let mut new_curr = curr.clone();
        for (x, y) in curr.keys() {
            for (dx, dy) in DIRECTIONS.iter() {
                new_curr.entry((x + dx, y + dy)).and_modify(|v| *v = (*v).min(distance)).or_insert(distance);
            }
        }
        curr = new_curr.clone();
    }

    curr.into_iter().collect()
}

fn cheats(forward_dist: &Vec<Vec<u64>>, backward_dist: &Vec<Vec<u64>>, best: u64, size: isize) -> Option<u64> {
    let mut count = 0;
    let directions = cheat_directions(size, 0, 0);
    for x in 0..backward_dist.len() {
        for y in 0..backward_dist[0].len() {
            for &((dx, dy), cheat_size) in directions.iter() {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if new_x >= backward_dist.len() || new_y >= backward_dist[0].len() {
                    continue;
                }
                if backward_dist[x][y] as u128 + forward_dist[new_x][new_y] as u128 + cheat_size as u128 <= best as u128 - 2 { // 100 {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}


pub fn part_one(input: &str) -> Option<u64> {
    let (map, start, end) = parse(input);
    let forward_dist = bfs(&map, end);
    let backward_dist = bfs(&map, start);
    let best = forward_dist[start.0][start.1];
    cheats(&forward_dist, &backward_dist, best, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (map, start, end) = parse(input);
    let forward_dist = bfs(&map, end);
    let backward_dist = bfs(&map, start);
    let best = forward_dist[start.0][start.1];
    cheats(&forward_dist, &backward_dist, best, 20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3081));
    }
}
