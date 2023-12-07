// alright, seems to be once again mostly a parsing challenge, but at least a fun one.
// after all, we get to write a custom compare function! let's see what we can do, shall we?

// oh neat! part 2 is not at all what I would have predicted, but should be very doable. Let's see.
// one neat trick: because of the order of hand types (and thanks to straights missing), it's
// always best to add your jokers to the most common card type. Watch out for JJJJJ, though!
// (which does exist)

// in hindsight, I think that was the smallest adjustment I had to do for a part 2 this year so far?
// Which is surprising, but I'm fine with that.

use aoc::{self, ParseLineError, PuzzlePart};
use std::error::Error;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

const CARD_NAMES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];

const CARD_NAMES_P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'
];

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, Debug)]
struct Hand {
    card_order: String,
    hand_type: HandType,
    bid: i32,
}

impl Hand {

    fn get_hand_type(cards: &str) -> Result<HandType, ParseLineError> {
        let mut map = HashMap::new();
        let mut no_jokers = 0;
        for c in cards.chars() {
            // pretty simple change
            if aoc::puzzle_part() == PuzzlePart::PartTwo && c == 'J' {
                no_jokers += 1;
                continue;
            }
            if !CARD_NAMES.contains(&c) {
                return Err(ParseLineError::new("Hand", cards));
            }
            let mut cur_val = *map.get(&c).unwrap_or(&0);
            cur_val += 1;
            map.insert(c, cur_val);
        }
        let mut freqs: Vec<i32> = map.into_values().collect();
        freqs.sort();
        freqs.reverse();
        //println!("{:?}", freqs);
        Ok(match freqs.get(0).unwrap_or(&0) + no_jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match freqs[1] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match freqs[1] {
                2 => HandType::TwoPair,
                _ => HandType::Pair,
            },
            _ => HandType::HighCard
        })
    }
    
}

impl PartialEq for Hand {

    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        for i in 0..5 {
            if self.card_order.chars().nth(i) != other.card_order.chars().nth(i) {
                return false;
            }
        }
        true
    }

}

impl Ord for Hand {

    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for i in 0..5 {
                    let names_array = match aoc::puzzle_part() {
                        PuzzlePart::PartOne => CARD_NAMES,
                        PuzzlePart::PartTwo => CARD_NAMES_P2,
                    };
                    let c1 = self.card_order.chars().nth(i).unwrap();
                    let p1 = names_array.iter().position(|&c| c == c1).unwrap();
                    let c2 = other.card_order.chars().nth(i).unwrap();
                    let p2 = names_array.iter().position(|&c| c == c2).unwrap();
                    match p1.cmp(&p2) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                Ordering::Equal
            },
        }
    }

}

impl PartialOrd for Hand {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

}

impl FromStr for Hand {
    type Err = ParseLineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 2 {
            return Err(ParseLineError::new("Hand", s));
        }
        let bid = split[1].parse::<i32>();
        if bid.is_err() {
            return Err(ParseLineError::new("Hand", s));
        }
        let bid = bid.unwrap();
        let hand_type = Hand::get_hand_type(split[0])?;
        Ok(Hand {
            card_order: String::from(split[0]),
            hand_type,
            bid
        })
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = aoc::read_input()?;
    let mut hands: Vec<Hand> = input.iter().filter_map(|s| match s.parse() {
        Ok(h) => Some(h),
        Err(e) => {
            println!("{}", e);
            None
        },
    }).collect();
    hands.sort();
    //hands.reverse();
    let res: i32 = hands.iter().enumerate().map(|(i, h)| (i + 1) as i32 * h.bid).sum();
    println!("{}", res);
    Ok(())
}
