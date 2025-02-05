advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_zeros(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    map.iter()
       .enumerate()
       .map(|(i, row)| {
            row.iter()
               .enumerate()
               .filter_map(|(j, val)| match val {
                    0 => Some((i, j)),
                    _ => None,
               }).collect::<Vec<(usize, usize)>>()
       })
       .flatten()
       .collect()
}

fn score(pos: (usize, usize), map: &Vec<Vec<u32>>, allow_duplicate_ends: bool) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut stack = vec![(pos, 0)];
    let mut trails: u32 = 0;
    
    while let Some((pos, score)) = stack.pop() {
        let (i, j) = pos;

        if visited[i][j] && !allow_duplicate_ends { continue; }
        visited[i][j] = true;

        if map[i][j] == 9 { 
            trails += 1;
            continue;
        }
        
        if i > 0 && map[i][j] + 1 == map[i - 1][j] {
            stack.push(((i - 1, j), score + 1));
        }
        if i < map.len() - 1 && map[i][j] + 1 == map[i + 1][j] {
            stack.push(((i + 1, j), score + 1));
        }
        if j > 0 && map[i][j] + 1 == map[i][j - 1] {
            stack.push(((i, j - 1), score + 1));
        }
        if j < map[0].len() - 1 && map[i][j] + 1 == map[i][j + 1] {
            stack.push(((i, j + 1), score + 1));
        }
    }
    
    trails
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let start_positions = find_zeros(&map);

    Some(start_positions.into_iter()
        .map(|pos| score(pos, &map, false))
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let start_positions = find_zeros(&map);

    Some(start_positions.into_iter()
        .map(|pos| score(pos, &map, true))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
