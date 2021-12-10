use std::str::FromStr;

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => return Ok(Direction::Forward),
            "down" => return Ok(Direction::Down),
            "up" => return Ok(Direction::Up),
            _ => return Err(ParseError),
        }
    }
}

#[derive(Debug)]
struct Instruction(Direction, usize);

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let dir: Direction = words.next().ok_or(ParseError).and_then(|w| w.parse())?;
        let val: usize = words
            .next()
            .and_then(|w| w.parse().ok())
            .ok_or(ParseError)?;

        Ok(Instruction(dir, val))
    }
}

fn calculate_coords(instructions: &[Instruction]) -> usize {
    let mut h: usize = 0;
    let mut v: usize = 0;

    for instruction in instructions {
        match instruction.0 {
            Direction::Down => v += instruction.1,
            Direction::Up => v -= instruction.1,
            Direction::Forward => h += instruction.1,
        }
    }
    return h * v;
}

fn calculate_coords_with_aim(instructions: &[Instruction]) -> usize {
    let mut h: usize = 0;
    let mut v: usize = 0;
    let mut aim: usize = 0;

    for instruction in instructions {
        match instruction.0 {
            Direction::Down => aim += instruction.1,
            Direction::Up => aim -= instruction.1,
            Direction::Forward => {
                h += instruction.1;
                v += instruction.1 * aim
            }
        }
    }
    return h * v;
}

pub fn day02() {
    let input = std::fs::read_to_string("inputs/day02.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();

    println!("Result of Part A is: {}", calculate_coords(&instructions));

    println!("Result of Part A is: {}", calculate_coords_with_aim(&instructions));
}
