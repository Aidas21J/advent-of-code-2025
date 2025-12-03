fn parse_bank(bank_str: &str) -> Result<Vec<u64>, String> {
    bank_str
        .chars()
        .map(|char| {
            match char.to_digit(10) {
                Some(digit) => Ok(digit as u64),
                None        => Err(format!("Invalid joltage: {char}")),
            }
        })
        .collect::<Result<Vec<u64>, String>>()
}

fn max_bank_joltage(bank: Vec<u64>) -> u64 {
    let digits: (Option<u64>, Option<u64>) = bank
        .iter()
        .rev()
        .skip(1)
        .rev()
        .fold((None, None), |digits, joltage| {
            match digits {
                (None,          second      ) => (Some(*joltage), second),
                (Some(first),   None        ) => {
                    if first < *joltage {
                        (Some(*joltage), None)
                    } else {
                        (Some(first), Some(*joltage))
                    }
                }
                (Some(first),   Some(second)) => {
                    if first < *joltage {
                        (Some(*joltage), None)
                    } else if second < *joltage {
                        (Some(first), Some(*joltage))
                    } else {
                        (Some(first), Some(second))
                    }
                },
            }
        });

    match (digits.0, digits.1, bank.last()) {
        (None,      _,          None        ) => 0,
        (None,      _,          Some(last)  ) => *last,
        (Some(d1),  None,       None        ) => d1,
        (Some(d1),  None,       Some(last)  ) => 10 * d1 + *last,
        (Some(d1),  Some(d2),   None        ) => 10 * d1 + d2,
        (Some(d1),  Some(d2),   Some(last)  ) => 10 * d1 + d2.max(*last),
    }
}

pub fn part1(input: &str) -> Result<u64, String> {
    input
        .lines()
        .map(|line| parse_bank(line.trim()))
        .map(|bank| match bank {
            Ok(bank) => Ok(max_bank_joltage(bank)),
            Err(err) => Err(err),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), Ok(357));
    }
}
