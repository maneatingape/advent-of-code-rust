//! # Crab Combat
//!
//! We start things off with a pun, by implementing a `Deck` of cards that is also a `Deque` or
//! [double-ended queue](https://en.wikipedia.org/wiki/Double-ended_queue) backed by a
//! circular buffer. Why use our own implementation when there is a perfectly good [`VecDeque`]
//! already available? The answer is speed. As there are at most fifty cards in the pack, the Deck
//! can use a fix sized stack allocated array, avoiding the expensive heap allocations that
//! [`VecDeque`] must use.
//!
//! `Deck` also keeps the score up to date as cards are added and removed, as this comes in useful
//! during part two. To update the score when a card is removed we subtract the card's value
//! multiplied by the size of the deck. For example if 5 is removed then the new
//! score is 67 - 5 * 4 = 47.
//!
//! | Deck ↓ | Weight | Score | Sum |
//! | - | - | -- | -- |
//! | 5 | 4 | 67 | 29 |
//! | 8 | 3 |    |    |
//! | 7 | 2 |    |    |
//! | 9 | 1 |    |    |
//!
//! When adding a card, it helps to have the sum of the existing cards. The new score is the old
//! score added to the new sum. For example if 6 is added to the deck:
//!
//! | Old Score | New Score | Difference |
//! | ----- | ----- | - |
//! | 5 * 4 | 5 * 5 | 5 |
//! | 8 * 3 | 8 * 4 | 8 |
//! | 7 * 2 | 7 * 3 | 7 |
//! | 9 * 1 | 9 * 2 | 9 |
//! | -     | 6 * 1 | 6 |
//! | Total: 67 | Total: 102 | Total: 35 |
//!
//! ## Part One
//!
//! The winner will always eventually be the player that starts with card 50 as they can never
//! lose this card in a round. However we need to play the full game in order to find out the
//! score of the winner's deck.
//!
//! ## Part Two
//!
//! We use two tricks to speed things up, one deterministic and one probabilistic.
//!
//! The deterministic trick is an observation that if Player 1 holds the high card in a recursive
//! game, then they will always eventually win. This comes from the fact that all cards are unique,
//! so the highest card is always greater than the size of the remaining deck, so a further
//! recursive game will never trigger when this card is played in a round. This means that Player 1
//! will never lose this card, so will either win outright or by triggering the repetition rule.
//!
//! This observation does *not* hold for Player 2. Although they can never lose the high card, they
//! could lose by the repetition rule, so the round needs to be played out in full.
//!
//! The probabilistic trick is an observation that the score makes a surprisingly good hash
//! function. Instead of storing the entire deck in the previously seen cache, we can store only
//! the combined hash of both decks, with a very good probability of no collisions.
//!
//! [`VecDeque`]: std::collections::VecDeque
use crate::util::hash::*;
use crate::util::parse::*;

type Input = (Deck, Deck);
type Cache = Vec<FastSet<(usize, usize)>>;

enum Winner {
    Player1,
    Player2,
}

#[derive(Clone, Copy)]
pub struct Deck {
    sum: usize,
    score: usize,
    start: usize,
    end: usize,
    cards: [u8; 50],
}

impl Deck {
    fn new() -> Deck {
        Deck { sum: 0, score: 0, start: 0, end: 0, cards: [0; 50] }
    }

    // To make things easier, `start` and `end` never wrap around, so that `end` is always
    // greater than or equal to `start`.
    fn pop_front(&mut self) -> usize {
        let card = self.cards[self.start % 50] as usize;
        self.sum -= card;
        self.score -= self.size() * card;
        self.start += 1;
        card
    }

    fn push_back(&mut self, card: usize) {
        self.cards[self.end % 50] = card as u8;
        self.sum += card;
        self.score += self.sum;
        self.end += 1;
    }

    fn max(&self) -> u8 {
        (self.start..self.end).map(|i| self.cards[i % 50]).max().unwrap()
    }

    fn non_empty(&self) -> bool {
        self.end > self.start
    }

    fn size(&self) -> usize {
        self.end - self.start
    }

    // Sneaky trick here to speed things up a little. We don't recalculate the score properly,
    // so it will be too high by a constant amount. This doesn't matter for recursive games as
    // we only need the winner, not the exact score.
    fn copy(&self, amount: usize) -> Deck {
        let mut copy = *self;
        copy.end = copy.start + amount;
        copy.sum = 0;

        for i in 0..amount {
            let card = copy.cards[(copy.start + i) % 50] as usize;
            copy.sum += card;
        }

        copy
    }
}

pub fn parse(input: &str) -> Input {
    let (mut deck1, mut deck2) = (Deck::new(), Deck::new());
    let (player1, player2) = input.split_once("\n\n").unwrap();

    player1.iter_unsigned().skip(1).for_each(|c| deck1.push_back(c));
    player2.iter_unsigned().skip(1).for_each(|c| deck2.push_back(c));

    (deck1, deck2)
}

pub fn part1(input: &Input) -> usize {
    let (mut deck1, mut deck2) = *input;

    while deck1.non_empty() && deck2.non_empty() {
        let (card1, card2) = (deck1.pop_front(), deck2.pop_front());

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.non_empty() { deck1.score } else { deck2.score }
}

pub fn part2(input: &Input) -> usize {
    let (mut deck1, mut deck2) = *input;

    match combat(&mut deck1, &mut deck2, &mut Vec::new(), 0) {
        Winner::Player1 => deck1.score,
        Winner::Player2 => deck2.score,
    }
}

fn combat(deck1: &mut Deck, deck2: &mut Deck, cache: &mut Cache, depth: usize) -> Winner {
    // Player 1 always wins recursive games if they have the high card.
    if depth > 0 && deck1.max() > deck2.max() {
        return Winner::Player1;
    }

    // Speed things up by re-using previously created caches, avoiding slow extra heap allocations.
    if cache.len() == depth {
        cache.push(FastSet::with_capacity(1_000));
    } else {
        cache[depth].clear();
    }

    while deck1.non_empty() && deck2.non_empty() {
        // This will *very probably* work! Not 100% deterministic.
        if !cache[depth].insert((deck1.score, deck2.score)) {
            return Winner::Player1;
        }

        let (card1, card2) = (deck1.pop_front(), deck2.pop_front());

        if deck1.size() < card1 || deck2.size() < card2 {
            if card1 > card2 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        } else {
            match combat(&mut deck1.copy(card1), &mut deck2.copy(card2), cache, depth + 1) {
                Winner::Player1 => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                Winner::Player2 => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        }
    }

    if deck1.non_empty() { Winner::Player1 } else { Winner::Player2 }
}
