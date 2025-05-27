advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Directions {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<char> for Directions {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '^' => Ok(Self::Up),
            'v' => Ok(Self::Down),
            _   => Err("Invalid direction"),
        }
    }
}

impl From<Directions> for (isize, isize) {
    fn from(value: Directions) -> Self{
        match value {
            Directions::Left => (0, -1),
            Directions::Right => (0, 1),
            Directions::Up => (-1, 0),
            Directions::Down => (1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '@' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            _   => Err(format!("Invalid cell {value}")),
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Cell>>, (usize, usize), Vec<Directions>) {
    let mut start = (0, 0);
    let (map, sequence) = input.split_once("\n\n").expect("Input should contain a map and a sequence");
    let map = map.lines()
        .enumerate()
        .map(|(i, line)| line.chars().enumerate().map(|(j, c)| {
            if c == '@' { start = (i, j); }
            Cell::try_from(c).unwrap()
        }).collect())
        .collect();
    let moves = sequence.lines()
        .flat_map(|line| line.chars().map(|c| Directions::try_from(c).unwrap()))
        .collect();
    (map, start, moves)
}

fn simulate(map: &mut Vec<Vec<Cell>>, pos: (usize, usize), dir: Directions, current_cell: Cell) -> Result<(usize, usize), (usize, usize)> {
    let (dx, dy) = dir.into();
    let (new_x, new_y) = ((pos.0 as isize + dx) as usize, (pos.1 as isize + dy) as usize);

    match map[new_x][new_y] {
        Cell::Wall => Err(pos),
        Cell::Empty => {
            map[new_x][new_y] = current_cell;
            Ok((new_x, new_y))
        },
        Cell::Box => {
            let mut new_map = map.clone();
            new_map[new_x][new_y] = current_cell;
            match simulate(&mut new_map, (new_x, new_y), dir, Cell::Box) {
                Ok(_) => {
                    *map = new_map;
                    Ok((new_x, new_y))
                },
                Err(_) => Err(pos),
            }
        },
        Cell::LeftBox => {
            let mut new_map = map.clone();
            match current_cell {
                Cell::Empty => {
                    new_map[new_x][new_y] = Cell::Empty;
                    new_map[new_x][new_y + 1] = Cell::Empty;
                },
                Cell::LeftBox => {
                    new_map[new_x][new_y] = Cell::LeftBox;
                    new_map[new_x][new_y + 1] = Cell::Empty;
                },
                Cell::RightBox => {
                    new_map[new_x][new_y] = Cell::RightBox;
                    new_map[new_x][new_y + 1] = Cell::Empty;
                },
                _ => unreachable!(),
            }
            match (
                simulate(&mut new_map, (new_x, new_y), dir, Cell::LeftBox),
                simulate(&mut new_map, (new_x, new_y + 1), dir, Cell::RightBox),
            ) {
                (Ok(_), Ok(_)) => {
                    *map = new_map;
                    Ok((new_x, new_y))
                },
                _ => Err(pos),
            }
        },
        Cell::RightBox => {
            let mut new_map = map.clone();
            match current_cell {
                Cell::Empty => {
                    new_map[new_x][new_y - 1] = Cell::Empty;
                    new_map[new_x][new_y] = Cell::Empty;
                },
                Cell::LeftBox => {
                    new_map[new_x][new_y - 1] = Cell::Empty;
                    new_map[new_x][new_y] = Cell::LeftBox;
                },
                Cell::RightBox => {
                    new_map[new_x][new_y - 1] = Cell::Empty;
                    new_map[new_x][new_y] = Cell::RightBox;
                },
                _ => unreachable!(),
            }
            match (
                simulate(&mut new_map, (new_x, new_y - 1), dir, Cell::LeftBox),
                simulate(&mut new_map, (new_x, new_y), dir, Cell::RightBox),
            ) {
                (Ok(_), Ok(_)) => {
                    *map = new_map;
                    Ok((new_x, new_y))
                },
                _ => Err(pos),
            }
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Cell>>, pos: (usize, usize)) {
    let mut char_map = map.iter()
        .map(|row| row.iter().map(|cell| match cell {
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::Empty => '.',
            Cell::LeftBox => '[',
            Cell::RightBox => ']',
        }).collect()
        ).collect::<Vec<Vec<char>>>();
    char_map[pos.0][pos.1] = '@';
    for row in char_map {
        println!("{}", row.iter().collect::<String>());
    }
    println!("\n\n");
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut map, mut pos, moves) = parse(input);

    // print_map(&map, pos);
    for dir in moves {
        pos = match simulate(&mut map, pos, dir, Cell::Empty) {
            Ok(new_pos) => new_pos,
            Err(_) => pos,
        };
    }
    // print_map(&map, pos);

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Cell::Box {
                ans += i as u64 * 100 + j as u64;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut map, mut pos, moves) = parse(input);

    // Double map
    map = map.into_iter()
        .map(|row| row.into_iter().flat_map(|cell| match cell {
            Cell::Box => vec![Cell::LeftBox, Cell::RightBox],
            _ => vec![cell, cell]
        }).collect())
        .collect();
    pos = (pos.0, pos.1 * 2);

    // print_map(&map, pos);
    for dir in moves {
        pos = match simulate(&mut map, pos, dir, Cell::Empty) {
            Ok(new_pos) => new_pos,
            Err(_) => pos,
        };
    }
    // print_map(&map, pos);

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Cell::LeftBox {
                ans += i as u64 * 100 + j as u64;
            }
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
