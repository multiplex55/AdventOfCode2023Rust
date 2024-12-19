use advent_of_code_rust::year2016::day03::*;

const EXAMPLE: &str = "\
101 201 301
102 102 402
203 203 303";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 2);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 2);
}
