use regex::Regex;

advent_of_code::solution!(13);

fn parse(input: &str) -> Vec<Puzzle> {
    input.split("\n\n")
         .map(|p| Puzzle::parse(p))
         .collect()
}

fn gcd<T>(mut a: T, mut b: T) -> T
where T: Copy + PartialOrd + std::ops::Sub<Output = T> {
    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    a
}

#[derive(Debug)]
struct Puzzle {
    a: (i128, i128),
    b: (i128, i128),
    goal: (i128, i128),
}

impl Puzzle {
    fn parse(input: &str) -> Self {
        fn capture_line(re: &Regex, line: &str) -> (i128, i128) {
            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [a, b])| (a.parse::<i128>().unwrap(), b.parse::<i128>().unwrap()))
                .next().unwrap()
        }
        let re: Regex = Regex::new(r"[\+=](\d*),.*[\+=](\d*)").unwrap();
        let lines: Vec<&str> = input.lines().collect();
        Self {
            a: capture_line(&re, lines[0]),
            b: capture_line(&re, lines[1]),
            goal: capture_line(&re, lines[2]),
        }
    }

    /*
        (a b) (x) = (g_1)
        (c d) (y) = (g_2)
        
        (a b)           (x) = (g_1)
        (0 d - c b / a) (y) = (g_2 - c g_1 / a)
            -> y = (g_2 - c g_1 / a) / (d - c b / a)
     */
    fn solve2(&self) -> Option<u128> {
        let (a, c) = self.a;
        let (b, d) = self.b;
        let (g_1, g_2) = self.goal;

        // Normalize
        let gcd = gcd(a, c);
        let b = b * c / gcd;
        let d = d *  a / gcd;
        let g_1 = g_1 * c / gcd;
        let g_2 = g_2 * a / gcd;
        let a = a * c / gcd;

        // Solve
        let d = d - b;
        let g_2 = g_2 - g_1;
        if d == 0 || g_2.rem_euclid(d) != 0 { return None; }
        let y = g_2 / d;
        if y < 0 || (g_1 - b * y).rem_euclid(a) != 0 { return None; }
        let x = (g_1 - b * y) / a;
        if x < 0 { return None; }

        Some((x * 3 + y) as u128)
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let p = parse(input);
    Some(p.iter().map(|p| p.solve2().unwrap_or(0)).sum())
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut p = parse(input);
    p.iter_mut().for_each(|p| {
        p.goal.0 += 10000000000000;
        p.goal.1 += 10000000000000;
    });
    Some(p.iter().map(|p| p.solve2().unwrap_or(0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
