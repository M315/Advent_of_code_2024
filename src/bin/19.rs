use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse(input: &str) -> (Vec<&str>, Vec<String>) {
    let (dict, messages) = input.split_once("\n\n").unwrap();
    let dict = dict.split(",")
        .map(|word| word.trim())
        .collect();
    let messages = messages.lines()
        .map(|line| line.trim().to_string())
        .collect();
    (dict, messages)
}

fn valid(dict: &Vec<&str>, memo: &mut HashMap<String, Option<u64>>, message: String) -> Option<u64> {
    if message.is_empty() { return Some(1); }
    if let Some(result) = memo.get(&message) { return *result; }
    let mut ans = 0;
    for word in dict {
        if message.ends_with(word) {
            let new_message = message[..message.len() - word.len()].to_string();
            if let Some(ways) = valid(dict, memo, new_message) { ans += ways; }
        }
    }
    let ans = match ans {
        0 => None,
        _ => Some(ans),
    };
    memo.insert(message, ans);
    ans
}

pub fn part_one(input: &str) -> Option<u64> {
    let (dict, messages) = parse(input);
    Some(messages.into_iter()
        .filter_map(|message| valid(&dict, &mut HashMap::new(), message))
        .count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (dict, messages) = parse(input);
    Some(messages.into_iter()
        .map(|message| valid(&dict, &mut HashMap::new(), message).unwrap_or(0))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
