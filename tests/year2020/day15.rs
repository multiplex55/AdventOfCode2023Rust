use advent_of_code_rust::year2020::day15::*;

const EXAMPLE: &str = "0,3,6";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 436);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 175594);
}
