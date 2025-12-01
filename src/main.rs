mod solutions;

use std::fs;

use solutions::day1;

fn main() {
    let input = fs::read_to_string("input/day1.txt")
        .expect("failed to read input");

    let answer1 = day1::part1(input).expect("part 1 failed");

    println!("Part 1: {answer1}");
}

