use advent_of_code_rust::year2022::day08::*;

const EXAMPLE: &str = "\
30373
25512
65332
33549
35390";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 21);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 8);
}
