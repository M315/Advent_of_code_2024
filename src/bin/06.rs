use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Direction::Up   => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right=> Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

impl Guard {
    fn new(pos: (usize, usize), c: char) -> Option<Self> {
        let dir: Direction = match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => { return None; }
        };
        Some(Guard{ pos, dir })
    }

    fn next_pos(self, map: &Vec<Vec<bool>>) -> Option<Self> {
        let next_pos: (i32, i32) = match self.dir {
            Direction::Up   => ( self.pos.0 as i32 - 1, self.pos.1 as i32 ),
            Direction::Down => ( self.pos.0 as i32 + 1, self.pos.1 as i32 ),
            Direction::Right=> ( self.pos.0 as i32, self.pos.1 as i32 + 1 ),
            Direction::Left => ( self.pos.0 as i32, self.pos.1 as i32 - 1 ),
        };
        // Check if we go out of bounce
        if next_pos.0 < 0 || next_pos.0 >= map.len() as i32 ||
           next_pos.1 < 0 || next_pos.1 >= map[0].len() as i32
        {
            return None;
        }

        match map[next_pos.0 as usize][next_pos.1 as usize] {
            true  => Some(Guard { pos: (next_pos.0 as usize, next_pos.1 as usize), dir: self.dir }),
            false => Some(Guard { pos: self.pos, dir: self.dir.rotate() }),
        }
    }
}

fn parse(input: &str) -> (Guard, Vec<Vec<bool>>) {
    let mut guard: Option<Guard> = None;
    let map: Vec<Vec<bool>> = input.lines()
        .enumerate()
        .map(|(i, line)| line.chars()
            .enumerate()
            .map(|(j, c)| match c {
                '#' => false,
                '.' => true,
                _ => { guard = Guard::new((i, j), c); true }
            })
            .collect()
        )
        .collect();
    
    (guard.unwrap(), map)
}

fn sim(guard: &mut Guard, map: &Vec<Vec<bool>>, mut positions: HashSet<Guard>) -> Option<u32> {
    positions.insert(*guard);
    while let Some(next_guard) = guard.next_pos(map) {
        if positions.contains(&next_guard) {
            return None;
        }
        positions.insert(next_guard);
        *guard = next_guard;
    }
    Some(positions.into_iter().map(|g| g.pos).collect::<HashSet<(usize, usize)>>().len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut guard, map) = parse(input);
    sim(&mut guard, &map, HashSet::new())
}

fn extra_obstacle_sim(guard: &mut Guard, map: &mut Vec<Vec<bool>>) -> Option<u32> {
    let original_guard = guard.clone();
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();

    while let Some(next_guard) = guard.next_pos(map) {
        // Add Obstacle
        let front_pos = guard.next_pos(map).unwrap().pos;
        if front_pos != guard.pos && !obstacles.contains(&front_pos) && map[front_pos.0][front_pos.1] {
            map[front_pos.0][front_pos.1] = false;

            // Find if it has a loop
            match sim(&mut original_guard.clone(), &map, HashSet::new()) {
                None => { obstacles.insert(front_pos); },
                Some(_) => {},
            };
            map[front_pos.0][front_pos.1] = true;
        }
        *guard = next_guard;
    }

    Some(obstacles.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut guard, mut map) = parse(input);
    extra_obstacle_sim(&mut guard, &mut map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
