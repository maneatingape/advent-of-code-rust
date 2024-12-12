use aoc::year2021::day16::*;

#[test]
fn part1_test() {
    let input = parse("8A004A801A8002F478");
    assert_eq!(part1(&input), 16);

    let input = parse("620080001611562C8802118E34");
    assert_eq!(part1(&input), 12);

    let input = parse("C0015000016115A2E0802F182340");
    assert_eq!(part1(&input), 23);

    let input = parse("A0016C880162017C3686B18A3D4780");
    assert_eq!(part1(&input), 31);
}

#[test]
fn part2_test() {
    let input = parse("C200B40A82");
    assert_eq!(part2(&input), 3);

    let input = parse("04005AC33890");
    assert_eq!(part2(&input), 54);

    let input = parse("880086C3E88112");
    assert_eq!(part2(&input), 7);

    let input = parse("CE00C43D881120");
    assert_eq!(part2(&input), 9);

    let input = parse("D8005AC2A8F0");
    assert_eq!(part2(&input), 1);

    let input = parse("F600BC2D8F");
    assert_eq!(part2(&input), 0);

    let input = parse("9C005AC2F8F0");
    assert_eq!(part2(&input), 0);

    let input = parse("9C0141080250320F1802104A08");
    assert_eq!(part2(&input), 1);
}
