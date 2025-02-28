advent_of_code::solution!(12);

use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
         .map(|line| line.chars().collect())
         .collect()
}

#[derive(Debug, Clone)]
struct Field {
    area: Vec<(i32, i32)>,
    perimeter: Vec<(i32, i32)>,
}

impl std::ops::Add for Field {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            area: self.area.into_iter().chain(other.area.into_iter()).collect(),
            perimeter: self.perimeter.into_iter().chain(other.perimeter.into_iter()).collect(),
        }
    }
}

impl Field {
    fn new((area, perimeter): (Vec<(i32, i32)>, Vec<(i32, i32)>)) -> Self {
        Self { area, perimeter }
    }

    fn value(&self) -> u64 {
        self.area.len() as u64 * self.perimeter.len() as u64
    }

    fn score(&self) -> u64 {
        self.area.len() as u64 * self.angles()
    }

    fn angles(&self) -> u64 {
        let mut angles = 0;
        let mut used = HashSet::new();
        // Exterior angles
        for &(i, j) in self.perimeter.iter() {
            if !used.contains(&((i, j), (i + 1, j + 1))) && self.perimeter.contains(&(i + 1, j + 1)) 
                && (self.area.contains(&(i + 1, j)) || self.area.contains(&(i, j + 1)))
                && !(self.area.contains(&(i + 1, j)) && self.area.contains(&(i, j + 1))) {
                // println!("{} {} | {} {}", i, j, i + 1, j + 1);
                angles += 1;
                used.insert(((i, j), (i + 1, j + 1)));
            }
            if !used.contains(&((i - 1, j + 1), (i, j))) && self.perimeter.contains(&(i - 1, j + 1))
                && (self.area.contains(&(i - 1, j)) || self.area.contains(&(i, j + 1)))
                && !(self.area.contains(&(i - 1, j)) && self.area.contains(&(i, j + 1))) {
                // println!("{} {} | {} {}", i, j, i - 1, j + 1);
                angles += 1;
                used.insert(((i - 1, j + 1), (i, j)));
            }
            if !used.contains(&((i - 1, j - 1), (i, j))) && self.perimeter.contains(&(i - 1, j - 1))
                && (self.area.contains(&(i - 1, j)) || self.area.contains(&(i, j - 1)))
                && !(self.area.contains(&(i - 1, j)) && self.area.contains(&(i, j - 1))) {
                // println!("{} {} | {} {}", i, j, i - 1, j - 1);
                angles += 1;
                used.insert(((i - 1, j - 1), (i, j)));
            }
            if !used.contains(&((i, j), (i + 1, j - 1))) && self.perimeter.contains(&(i + 1, j - 1))
                && (self.area.contains(&(i + 1, j)) || self.area.contains(&(i, j - 1)))
                && !(self.area.contains(&(i + 1, j)) && self.area.contains(&(i, j - 1))) {
                // println!("{} {} | {} {}", i, j, i + 1, j - 1);
                angles += 1;
                used.insert(((i, j), (i + 1, j - 1)));
            }
        }
        // Interior angles
        let mut used = HashSet::new();
        for &(i, j) in self.perimeter.iter() {
            if self.area.contains(&(i + 1, j)) && self.area.contains(&(i, j + 1)) 
                && !used.contains(&((i, j), (i, j + 1), (i + 1, j))) {
                // println!("{} {} | {} {} | {} {}", i, j, i + 1, j, i, j + 1);
                angles += 1;
                used.insert(((i, j), (i, j + 1), (i + 1, j)));
            }
            if self.area.contains(&(i + 1, j)) && self.area.contains(&(i, j - 1))
                && !used.contains(&((i, j - 1), (i, j), (i + 1, j))) {
                // println!("{} {} | {} {} | {} {}", i, j, i + 1, j, i, j - 1);
                angles += 1;
                used.insert(((i, j - 1), (i, j), (i + 1, j)));
            }
            if self.area.contains(&(i - 1, j)) && self.area.contains(&(i, j + 1))
                && !used.contains(&((i - 1, j), (i, j), (i, j + 1))) {
                // println!("{} {} | {} {} | {} {}", i, j, i - 1, j, i, j + 1);
                angles += 1;
                used.insert(((i - 1, j), (i, j), (i, j + 1)));
            }
            if self.area.contains(&(i - 1, j)) && self.area.contains(&(i, j - 1))
                && !used.contains(&((i - 1, j), (i, j - 1), (i, j))) {
                // println!("{} {} | {} {} | {} {}", i, j, i - 1, j, i, j - 1);
                angles += 1;
                used.insert(((i - 1, j), (i, j - 1), (i, j)));
            }
        }
        angles
    }
}

fn score(grid: &Vec<Vec<char>>, used: &mut Vec<Vec<bool>>, i: usize, j: usize, field: char) -> Field {
    if i < grid.len() && j < grid[i].len() && used[i][j] && grid[i][j] == field { return Field::new((Vec::new(), Vec::new())); }
    if i >= grid.len() || j >= grid[i].len() { return Field::new((Vec::new(), vec![(i as i32, j as i32)])); }

    if grid[i][j] != field { return Field::new((Vec::new(), vec![(i as i32, j as i32)])); }

    used[i][j] = true;
    let mut ans = Field::new((vec![(i as i32, j as i32)], Vec::new()));
    ans = ans + score(grid, used, i + 1, j, field);
    ans = ans + score(grid, used, i, j + 1, field);
    if i > 0 {
        ans = ans + score(grid, used, i - 1, j, field);
    } else {
        ans = ans + Field::new((Vec::new(), vec![(i as i32 - 1, j as i32)]));
    }
    if j > 0 {
        ans = ans + score(grid, used, i, j - 1, field);
    } else {
        ans = ans + Field::new((Vec::new(), vec![(i as i32, j as i32 - 1)]));
    }
    ans
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    let mut used = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            ans += score(&grid, &mut used, i, j, grid[i][j]).value();
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse(input);
    let mut used = vec![vec![false; grid[0].len()]; grid.len()];
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            ans += score(&grid, &mut used, i, j, grid[i][j]).score();
        }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
