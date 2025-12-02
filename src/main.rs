mod solutions;

use std::fs;

use solutions::day2;

fn main() {
    let input = fs::read_to_string("input/day2.txt")
        .expect("failed to read input");

    let answer1 = day2::part1(&input).expect("part 1 failed");

    println!("Part 1: {answer1}");
}

