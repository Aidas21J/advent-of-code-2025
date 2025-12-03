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

fn best_bank_joltage(bank: &[u64], battery_count: usize, result: u64) -> Option<u64> {
    if battery_count == 0 {
        return Some(result);
    } else if !(0 < battery_count && battery_count <= bank.len()) {
        return None;
    }

    let next_digit = match bank
        .iter()
        .rev()
        .skip(battery_count - 1)
        .rev()
        .max()
    {
        Some(m) => m,
        None    => return Some(result),
    };
    let next_digit_position = match bank
        .iter()
        .position(|&x| x == *next_digit) 
    {
        Some(p) => p,
        None    => return None,
    };

    best_bank_joltage(&bank[next_digit_position + 1..], battery_count - 1, 10 * result + next_digit)
}

fn compute_best_joltage_sum(input: &str, compute_best_bank_joltage: fn(&[u64]) -> Option<u64>) -> Result<u64, String> {
    input
        .lines()
        .map(|line| parse_bank(line.trim()))
        .map(|res| res
            .and_then(|bank| match compute_best_bank_joltage(&bank) {
                Some(max_joltage)   => Ok(max_joltage),
                None                => Err(format!("could not find best joltage of {:?}", bank)),
        }))
        .sum()
}

pub fn part1(input: &str) -> Result<u64, String> {
    compute_best_joltage_sum(input, |bank: &[u64]| best_bank_joltage(&bank, 2, 0))
}


pub fn part2(input: &str) -> Result<u64, String> {
    compute_best_joltage_sum(input, |bank: &[u64]| best_bank_joltage(&bank, 12, 0))
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
    
    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), Ok(3121910778619));
    }
}
