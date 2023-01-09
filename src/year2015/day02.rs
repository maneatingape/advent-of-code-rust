use crate::util::parse::to_tuple_3;

type Gift = (u32, u32, u32);

pub fn parse(input: &str) -> Vec<Gift> {
    fn helper(line: &str) -> Gift {
        let mut gift: Gift = to_tuple_3(line);
        sort(&mut gift);
        gift
    }
    input.lines().map(helper).collect()
}

pub fn part1(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let (l, w, h) = gift;
        2 * (l * w + w * h + h * l) + l * w
    }
    input.iter().map(helper).sum()
}

pub fn part2(input: &[Gift]) -> u32 {
    fn helper(gift: &Gift) -> u32 {
        let (l, w, h) = gift;
        2 * (l + w) + (l * w * h)
    }
    input.iter().map(helper).sum()
}

fn sort(gift: &mut Gift) {
    let mut tmp;

    if gift.0 > gift.1 {
        tmp = gift.0;
        gift.0 = gift.1;
        gift.1 = tmp;
    }
    if gift.1 > gift.2 {
        tmp = gift.1;
        gift.1 = gift.2;
        gift.2 = tmp;

        if gift.0 > gift.1 {
            tmp = gift.0;
            gift.0 = gift.1;
            gift.1 = tmp;
        }
    }
}
