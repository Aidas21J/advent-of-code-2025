mod solutions;

use std::fs;

use solutions::day03;

fn main() {
    let input = fs::read_to_string("input/day03.txt")
        .expect("failed to read input");

    let answer1 = day03::part1(&input).expect("part 1 failed");
    let answer2 = day03::part2(&input).expect("part 2 failed");

    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}

