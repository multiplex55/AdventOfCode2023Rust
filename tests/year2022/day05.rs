use advent_of_code_rust::year2022::day05::*;

const EXAMPLE: &str = "\
....[D]....
[N].[C]....
[Z].[M].[P]
.1...2...3.

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "CMZ");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "MCD");
}
