use std::str::FromStr;

const FULL_ROTATION: u64 = 100;
const DIAL_START_POSITION: u64 = 50;

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    position: u64,
}

impl Dial {
    fn new() -> Dial {
        Dial { position: DIAL_START_POSITION }
    }

    fn at_zero(&self) -> bool {
        self.position == 0
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

        let amount = match s_iterator.as_str().parse::<u64>() {
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
        let turn_right_amount = match rotation.direction {
            RotationDirection::LEFT     => FULL_ROTATION - (rotation.amount % FULL_ROTATION),
            RotationDirection::RIGHT    => rotation.amount,
        };

        self.position = (self.position + turn_right_amount) % FULL_ROTATION;
    }
}

pub fn part1(input: String) -> Result<u64, String> {
    let mut dial = Dial::new();
    let mut number_if_zeroes: u64 = 0;

    for line in input.lines() {
        let rotation = match line.trim().parse::<Rotation>() {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        dial.turn(&rotation);
        if dial.at_zero() {
            number_if_zeroes += 1;
        }
    }

    return Ok(number_if_zeroes);
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
        assert_eq!(part1(EXAMPLE_INPUT.to_owned()), Ok(3));
    }
}
