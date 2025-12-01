use std::str::FromStr;

const FULL_ROTATION: u64 = 100;
const DIAL_START_POSITION: u64 = 50;

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    position: u64,
    point_at_zero_count: u64,
}

impl Dial {
    fn new() -> Dial {
        Dial { 
            position: DIAL_START_POSITION,
            point_at_zero_count: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RotationDirection {
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq, Eq)]
struct Rotation {
    direction: RotationDirection,
    amount: u64,
}

type ParseRotationError = String;

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_iterator = s.chars();

        let direction = match s_iterator.next() {
            Some('L')   => RotationDirection::LEFT,
            Some('R')   => RotationDirection::RIGHT,
            Some(d)     => return Err(format!("invalid rotation direction: {d}")),
            None        => return Err("no rotation direction provided".to_owned()),
        };

        let amount = match s_iterator.as_str().parse() {
            Ok(a) => a,
            Err(err) => return Err(format!("failed to parse rotation amount: {err}")),
        };

        return Ok(Rotation{
            amount,
            direction,
        });
    }
}

impl Dial {
    fn turn(&mut self, rotation: &Rotation) {
        if rotation.direction == RotationDirection::LEFT {
            self.position = (FULL_ROTATION - self.position) % FULL_ROTATION;
        }

        self.position = (self.position + rotation.amount) % FULL_ROTATION;

        if rotation.direction == RotationDirection::LEFT {
            self.position = (FULL_ROTATION - self.position) % FULL_ROTATION;
        }
        self.point_at_zero_count += (self.position == 0) as u64;
    }

    fn turn_with_intermediate_zero(&mut self, rotation: &Rotation) {
        if rotation.direction == RotationDirection::LEFT {
            self.position = (FULL_ROTATION - self.position) % FULL_ROTATION;
        }

        let sum = self.position + rotation.amount;
        self.position = sum % FULL_ROTATION;
        self.point_at_zero_count += sum / FULL_ROTATION;

        if rotation.direction == RotationDirection::LEFT {
            self.position = (FULL_ROTATION - self.position) % FULL_ROTATION;
        }
    }
}

pub fn part1(input: &String) -> Result<u64, String> {
    let mut dial = Dial::new();

    for line in input.lines() {
        let rotation = match line.trim().parse::<Rotation>() {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        dial.turn(&rotation);
    }

    return Ok(dial.point_at_zero_count);
}

pub fn part2(input: &String) -> Result<u64, String> {
    let mut dial = Dial::new();

    for line in input.lines() {
        let rotation = match line.trim().parse::<Rotation>() {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        dial.turn_with_intermediate_zero(&rotation);
    }

    return Ok(dial.point_at_zero_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE_INPUT.to_owned()), Ok(3));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE_INPUT.to_owned()), Ok(6));
    }
}
