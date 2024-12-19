use advent_of_code_rust::year2016::day12::*;

const EXAMPLE: &str = "\
cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 1000 c // Test value
cpy 1000 d // Test value
inc a
dec d
jnz d -2
dec c
jnz c -5";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 1317811);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 10227465);
}
