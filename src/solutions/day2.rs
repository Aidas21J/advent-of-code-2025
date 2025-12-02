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

pub fn part1(input: &String) -> Result<u64, String> {
    let is_invalid_id = |id: &u64| {
        let id_str = id.to_string();

        let first_part = id_str[..(id_str.len() / 2)].to_string();
        let second_part = id_str[(id_str.len() / 2)..].to_string();

        return first_part == second_part;
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&EXAMPLE_INPUT.to_owned()), Ok(1227775554));
    }
}
