use std::fmt;
use rand::rngs::ThreadRng;
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut rng = rand::rng();
    // Start War Sim
    let game = war_sim(&mut rng);
    
    println!("The game lasted {} turns resulting in {} winning", game.turns, game.winner);
}

fn war_sim(rng: &mut ThreadRng) -> GameData {
    // Make and shuffle deck
    let mut deck = full_deck();
    deck.shuffle(rng);
    
    // Last element represents top
    let mut player_1 = Vec::with_capacity(52);
    let mut player_2 = Vec::with_capacity(52);

    // Deal Cards
    for (i, card) in deck.iter().enumerate() {
        if i % 2 == 0 {
            player_1.push(*card);
        } else {
            player_2.push(*card);
        }
    }

    // Offhands represent the pile of cards you earn
    let mut offhand_1 = Vec::with_capacity(52);
    let mut offhand_2 = Vec::with_capacity(52);
    let mut turns: u32 = 0;

    loop {
        turns += 1;
        let mut pile = Vec::new();

        loop {
            let option_card_1 = get_card(&mut player_1, &mut offhand_1, rng);
            if option_card_1.is_none() {
                return GameData { winner: Player::Player2, turns };
            }
            let card_1 = option_card_1.unwrap();
            pile.push(card_1);

            let option_card_2 = get_card(&mut player_2, &mut offhand_2, rng);
            if option_card_2.is_none() {
                return GameData { winner: Player::Player1, turns };
            }
            let card_2 = option_card_2.unwrap();
            pile.push(card_2);

            if card_1 > card_2 {
                offhand_1.append(&mut pile);
                break;
            }

            if card_1 < card_2 {
                offhand_2.append(&mut pile);
                break;
            }

            for _ in 0..3 {
                let option_card_1 = get_card(&mut player_1, &mut offhand_1, rng);
                if option_card_1.is_none() {
                    return GameData { winner: Player::Player2, turns };
                }
                pile.push(option_card_1.unwrap());

                let option_card_2 = get_card(&mut player_2, &mut offhand_2, rng);
                if option_card_2.is_none() {
                    return GameData { winner: Player::Player1, turns };
                }
                pile.push(option_card_2.unwrap());
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    Player1,
    Player2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Player::Player1 => "Player 1",
            Player::Player2 => "Player 2",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GameData {
    turns: u32,
    winner: Player,
}

impl PartialOrd for GameData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.turns.partial_cmp(&other.turns)
    }
}

impl Ord for GameData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.turns.cmp(&other.turns)
    }
}

fn get_card(hand: &mut Vec<CardValue>, mut offhand: &mut Vec<CardValue>, rng: &mut ThreadRng) -> Option<CardValue> {
    let mut option_card = hand.pop();
    if option_card.is_none() {
        if offhand.is_empty() {
            return None;
        } else {
            hand.append(&mut offhand);
            hand.shuffle(rng);
            option_card = hand.pop();
        }
    }
    option_card
}

fn full_deck() -> Vec<CardValue> {
    let mut deck = Vec::with_capacity(52);

    for value in CardValue::iter() {
        for _ in 0..4 {
            deck.push(value);
        }
    }

    deck
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CardValue::Ace => "A",
            CardValue::Two => "2",
            CardValue::Three => "3",
            CardValue::Four => "4",
            CardValue::Five => "5",
            CardValue::Six => "6",
            CardValue::Seven => "7",
            CardValue::Eight => "8",
            CardValue::Nine => "9",
            CardValue::Ten => "10",
            CardValue::Jack => "J",
            CardValue::Queen => "Q",
            CardValue::King => "K",
        })
    }
}