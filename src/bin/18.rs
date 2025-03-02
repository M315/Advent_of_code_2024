use std::collections::BinaryHeap;

advent_of_code::solution!(18);

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn parse(input: &str) -> Vec<(usize, usize)> {
    input.lines()
         .map(|line| line.split_once(',').unwrap())
         .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
         .collect()
}

fn dijkstra(grid: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> Option<u64> {
    let mut dist = vec![vec![i32::MIN; grid.len()]; grid.len()];
    let mut heap = BinaryHeap::new();

    dist[start.0][start.1] = 0;
    heap.push((0, start));

    while let Some((cost, (i, j))) = heap.pop() {
        // println!("{} {:?}", cost, (i, j));
        if grid[i][j] == -1 { continue; }
        if (i, j) == end { return Some(-cost as u64); }

        if cost < dist[i][j] { continue; }

        for (di, dj) in DIRECTIONS {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid.len() as i32 { continue; }
            let (ni, nj) = (ni as usize, nj as usize);
            if cost - 1 > dist[ni][nj] {
                dist[ni][nj] = cost - 1;
                heap.push((cost - 1, (ni, nj)));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let pos = parse(input);
    let size = *pos.iter().map(|(a, b)| a.max(b)).max().unwrap();
    let mut grid = vec![vec![i32::MAX; size + 1]; size + 1];
    for (i, j) in pos.into_iter().take(12) {
        grid[j][i] = -1;
    }
    dijkstra(&grid, (0, 0), (size, size))
}

fn simulate(pos: &Vec<(usize, usize)>, time: usize) -> Option<u64> {
    let size = *pos.iter().map(|(a, b)| a.max(b)).max().unwrap();
    let mut grid = vec![vec![i32::MAX; size + 1]; size + 1];
    for &(i, j) in pos.iter().take(time) {
        grid[j][i] = -1;
    }
    dijkstra(&grid, (0, 0), (size, size))
}

pub fn part_two(input: &str) -> Option<u64> {
    let pos = parse(input);
    let mut left = 0;
    let mut right = pos.len();
    while left < right {
        let mid = (left + right) / 2;
        let sim = simulate(&pos, mid);
        match sim {
            Some(_) => left = mid + 1,
            None => right = mid,
        }
    }
    println!("{:?}", pos[left - 1]);
    Some(left as u64 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
