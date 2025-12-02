use std::str::FromStr;

const RANGE_DELIMITER: char = ',';
const ID_DELIMITER: char = '-';

struct IdRange {
    min: u64,
    max: u64,
}

impl IdRange {
    fn sum_invalid_ids(&self, is_invalid_id: fn(&u64) -> bool) -> u64 {
        (self.min..=self.max)
            .filter(is_invalid_id)
            .sum()
    }
    
}

impl FromStr for IdRange {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_id_str, last_id_str) = match s.split(ID_DELIMITER).collect::<Vec<&str>>()[..] {
            [id1, id2]  => (id1, id2),
            _           => return Err("invalid number of arguments".to_owned())
        };

        return match (first_id_str.parse::<u64>(), last_id_str.parse::<u64>()) {
            (Err(err),  _       ) => Err(format!("failed to read first id: {err} [range: {s}]")),
            (Ok(_),     Err(err)) => Err(format!("failed to read last id: {err} [range: {s}]")),
            (Ok(min),   Ok(max) ) => Ok(IdRange { min, max }),
        };
    }
}

fn sum_all_invalid_ids(input: &String, is_invalid_id: fn(&u64) -> bool) -> Result<u64, String> {
    let range_invalid_id_sum = |range_str: &str|
        range_str
            .parse::<IdRange>()
            .and_then(|range| Ok(range.sum_invalid_ids(is_invalid_id)));

    return input
        .trim()
        .split(RANGE_DELIMITER)
        .map(range_invalid_id_sum)
        .sum();
}

pub fn part1(input: &String) -> Result<u64, String> {
    let is_invalid_id = |id: &u64| {
        let id_str = id.to_string();

        let first_part = id_str[..(id_str.len() / 2)].to_string();
        let second_part = id_str[(id_str.len() / 2)..].to_string();

        return first_part == second_part;
    };

    sum_all_invalid_ids(input, is_invalid_id)
}


fn is_chunk_invalid(id: &u64, chunk_size: u32) -> bool {
    let multiplier: u64 = (10 as u64).pow(chunk_size as u32);
    let mut id_leftover = id.clone();
    let mut check_against: Option<u64> = None;

    if id_leftover.checked_div(multiplier) == Some(0) {
        return false;
    }

    while id_leftover > 0 {
        let current_number = id_leftover
            .checked_rem(multiplier)
            .unwrap_or(0);

        check_against = match check_against {
            Some(val) => {
                if val != current_number {
                    return false;
                }
                Some(current_number)
            },
            None => Some(current_number),
        };

        id_leftover = id_leftover
            .checked_div(multiplier)
            .unwrap_or(0);
        }

    return true;
}

pub fn part2(input: &String) -> Result<u64, String> {
    let is_invalid_id = |id: &u64| -> bool {
        let id_str = id.to_string();

        for chunk_size in 1..=(id_str.len() / 2) {
            if id_str.len() % chunk_size != 0 {
                continue;
            }
            
            if is_chunk_invalid(id, chunk_size as u32) {
                return true;
            }
        }

        return false;
    };

    sum_all_invalid_ids(input, is_invalid_id)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE_INPUT.to_owned()), Ok(1227775554));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&EXAMPLE_INPUT.to_owned()), Ok(4174379265));
    }
}
