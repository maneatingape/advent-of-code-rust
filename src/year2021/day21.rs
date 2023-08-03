use crate::util::iter::*;
use crate::util::parse::*;

type Pair = (u64, u64);
type State = (Pair, Pair);

const DIRAC: [Pair; 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

pub fn parse(input: &str) -> State {
    let [_, one, _, two]: [u64; 4] = input.iter_unsigned().chunk::<4>().next().unwrap();
    ((one - 1, 0), (two - 1, 0))
}

pub fn part1(input: &State) -> u64 {
    let mut state = *input;
    let mut dice = 6;
    let mut rolls = 0;

    loop {
        let ((player_position, player_score), (other_position, other_score)) = state;
        let next_position = (player_position + dice) % 10;
        let next_score = player_score + next_position + 1;

        dice = (dice + 9) % 10;
        rolls += 3;

        if next_score >= 1000 {
            return other_score * rolls;
        } else {
            state = ((other_position, other_score), (next_position, next_score));
        }
    }
}

pub fn part2(input: &State) -> u64 {
    let mut cache: [Option<Pair>; 44100] = [None; 44100];
    let (win, lose) = dirac(*input, &mut cache);
    win.max(lose)
}

fn dirac(state: State, cache: &mut [Option<Pair>]) -> Pair {
    let ((player_position, player_score), (other_position, other_score)) = state;

    // 10, 10, 21 ,21
    let index = player_position + 10 * other_position + 100 * player_score + 2100 * other_score;
    if let Some(result) = cache[index as usize] {
        return result;
    }

    let helper = |(win, lose), &(dice, frequency)| {
        let next_position = (player_position + dice) % 10;
        let next_score = player_score + next_position + 1;

        if next_score >= 21 {
            (win + frequency, lose)
        } else {
            let next_state = ((other_position, other_score), (next_position, next_score));
            let (next_lose, next_win) = dirac(next_state, cache);
            (win + frequency * next_win, lose + frequency * next_lose)
        }
    };

    let result = DIRAC.iter().fold((0, 0), helper);
    cache[index as usize] = Some(result);
    result
}
