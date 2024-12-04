advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<Vec<char>> {
        input.lines()
        .map(|line| line.chars().collect() )
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse(input);
    let n: usize = grid.len();
    let m: usize = grid[0].len();

    let key_word: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut rev_word = key_word.clone();
    rev_word.reverse();

    let mut count: u32 = 0;

    // Horizontal
    for line in grid.iter() {
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }

    // Vertical
    for j in 0..m {
        let mut line = Vec::new();
        for i in 0..n {
            line.push(grid[i][j]);
        }
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }
    
    // Diagonal \
    for j in 0..m {
        let mut line = Vec::new();
        for i in 0..n - j {
            line.push(grid[i][j+i]);
        }
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }
    for i in 1..n {
        let mut line = Vec::new();
        for j in 0..m - i {
            line.push(grid[i+j][j]);
        }
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }

    // Diagonal /
    for j in 0..m {
        let mut line = Vec::new();
        for i in 0..n - j {
            line.push(grid[i][m - 1 - j - i]);
        }
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }
    for i in 1..n {
        let mut line = Vec::new();
        for j in 0..m - i {
            line.push(grid[i+j][m - 1 - j]);
        }
        for word in line.windows(key_word.len()) {
            if word == key_word { count += 1; }
            if word == rev_word { count += 1; }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse(input);
    let n: usize = grid.len();
    let m: usize = grid[0].len();

    let mut count: u32 = 0;

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] != 'A' { continue; }
            count += match (grid[i-1][j-1], grid[i+1][j+1], grid[i-1][j+1], grid[i+1][j-1]) {
              ('M', 'S', 'M', 'S') => 1,
              ('M', 'S', 'S', 'M') => 1,
              ('S', 'M', 'M', 'S') => 1,
              ('S', 'M', 'S', 'M') => 1,
              _ => 0,
            };
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
