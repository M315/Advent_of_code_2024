use regex::Regex;

advent_of_code::solution!(14);

static N: i32 = 101; // 11;
static M: i32 = 103; //  7;

#[derive(Debug, Copy, Clone)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    fn parse(line: &str) -> Self {
        Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)")
            .unwrap()
            .captures_iter(line)
            .map(|c| c.extract())
            .map(|(_, [p1, p2, v1, v2])| Robot {
                p: (p1.parse::<i32>().unwrap(), p2.parse::<i32>().unwrap()),
                v: (v1.parse::<i32>().unwrap(), v2.parse::<i32>().unwrap()),
            })
            .next().unwrap()
    }

    fn step(&mut self) {
        self.p.0 = (self.p.0 + self.v.0).rem_euclid(N);
        self.p.1 = (self.p.1 + self.v.1).rem_euclid(M);
    }

    fn get_quadrant(&self) -> Option<usize> {
        let (i, j) = self.p;
        if i < N / 2 && j < M / 2 {
            Some(0)
        } else if i > N / 2 && j < M / 2 {
            Some(1)
        } else if i < N / 2 && j > M / 2 {
            Some(2)
        } else if i > N / 2 && j > M / 2 {
            Some(3)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines()
        .map(|line| Robot::parse(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut robots = parse(input);


    robots.iter_mut()
        .for_each(|robot| {
            for _ in 1..=100 {
                robot.step();
            }
        });

    let mut quadrants = vec![0; 4];
    for robot in robots {
        if let Some(quadrant) = robot.get_quadrant() {
            quadrants[quadrant] += 1;
        }
    }

    Some(quadrants.iter().product::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut robots = parse(input);
    let mut best = (0, u64::MAX);

    for k in 1..10000 {
        robots.iter_mut()
            .for_each(|robot| {
                robot.step();
            });

        let mut quadrants = vec![0; 4];
        for robot in &robots {
            if let Some(quadrant) = robot.get_quadrant() {
                quadrants[quadrant] += 1;
            }
        }
        let score = quadrants.iter().product::<usize>() as u64;
        if score < best.1 {
            best = (k, score);
        }

        // if k == 7138 {
        //     let mut map = vec![vec![0; M as usize]; N as usize];
        //     for robot in &robots {
        //         map[robot.p.0 as usize][robot.p.1 as usize] += 1;
        //     }
        //     for row in &map {
        //         println!("{}", row.iter().map(|x| if *x > 0 { '#' } else { '.' }).collect::<String>());
        //     }
        // }
    }

    Some(best.0 as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
