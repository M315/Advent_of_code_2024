use regex::Regex;

advent_of_code::solution!(3);

fn parse_one(input: &str, re: Regex) -> Vec<(u32, u32)> {
        input.lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|m| m.extract())
                .map(|(_, [a, b])| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .collect::<Vec<(u32, u32)>>()
        })
        .flatten()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(parse_one(input, re).into_iter()
        .map(|(a, b)| a * b)
        .sum())
}

fn parse_two(input: &str, re: Regex) -> Vec<&str> {
        input.lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|m| m.extract())
                .map(|(_, [s])| s)
                .collect::<Vec<&str>>()
        })
        .flatten()
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re: Regex = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d+,\d+)\)").unwrap();
    let mut enabled: bool = true;
    Some(parse_two(input, re).into_iter()
        .filter_map(|s| {
            match s {
                "do()"    => { enabled = true; None },
                "don't()" => { enabled = false; None },
                _ => {
                    if enabled {
                        let (a, b) = s.split_once(",").unwrap();
                        Some(a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
