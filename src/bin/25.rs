advent_of_code::solution!(25);

#[derive(Debug, Eq, PartialEq)]
enum Orientation {
    Up,
    Down,
}

#[derive(Debug)]
struct Pice {
    orientation: Orientation,
    values: Vec<u32>,
}

impl Pice {
    fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<&str>>();
        let orientation = match lines[0] {
            "#####" => Orientation::Up,
            "....." => Orientation::Down,
            _ => panic!("Unexpected input"),
        };
        let values = lines.into_iter()
            .map(|line| line.chars()
                .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Unexpected input"),
                })
            )
            .fold(vec![0; 5], |acc, line| {
                acc.iter().zip(line).map(|(a, b)| a + b).collect()
            });
        Self { orientation, values }
    }

    fn can_match(&self, other: &Self) -> bool {
        if self.orientation == other.orientation { return false; }
        self.values.iter()
            .zip(other.values.iter())
            .all(|(a, b)| a + b <= 7)
    }
}

fn parse(input: &str) -> Vec<Pice> {
    input.split("\n\n")
         .map(|pice| Pice::new(pice))
         .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let pices = parse(input);
    let mut result = 0;
    for i in 0..pices.len() {
        for j in i+1..pices.len() {
            if pices[i].can_match(&pices[j]) {
                result += 1;
            }
        }
    }
    Some(result)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
