use std::fs::File;
use std::io::Write;
use std::fmt;
use rand::rngs::ThreadRng;
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut rng = rand::rng();

    // Make csv output file
    let mut file = File::create("results.csv").expect("Unable to create file");
    writeln!(file, "turns,winner").expect("Unable to write header");

    println!("Starting Simulations");

    // Run 10,000 simulations
    for _ in 0..10_000 {
        let data = war_sim(&mut rng);
        writeln!(file, "{},{}", data.turns, data.winner).expect("Unable to write data");
    }

    println!("Simulations Complete")
}

fn war_sim(rng: &mut ThreadRng) -> GameData {
    // Make and shuffle deck
    let mut deck = full_deck();
    deck.shuffle(rng);
    
    // Last element represents top
    let (mut player_1, mut player_2) = deal_cards(&deck);

    // Offhands represent the pile of cards you earn
    let mut offhand_1 = Vec::with_capacity(52);
    let mut offhand_2 = Vec::with_capacity(52);
    let mut turns: u32 = 0;

    loop {
        turns += 1;
        
        match play_turn(&mut player_1, &mut offhand_1, &mut player_2, &mut offhand_2, rng) {
            TurnOutcome::Winner(winner) => return GameData { winner, turns },
            TurnOutcome::PileAwarded(Player::Player1, mut pile) => {
                offhand_1.append(&mut pile);
            },
            TurnOutcome::PileAwarded(Player::Player2, mut pile) => {
                offhand_2.append(&mut pile);
            }
        }
    }
}

fn deal_cards(deck: &Vec<CardValue>) -> (Vec<CardValue>, Vec<CardValue>) {
    let mut player_1 = Vec::with_capacity(52);
    let mut player_2 = Vec::with_capacity(52);

    // Passes deck evenly between players going back and forth
    for (i, &card) in deck.iter().enumerate() {
        if i % 2 == 0 {
            player_1.push(card);
        } else {
            player_2.push(card);
        }
    }

    (player_1, player_2)
}

enum TurnOutcome {
    Winner(Player), // Winner of the entire game
    PileAwarded(Player, Vec<CardValue>), // Holds the player who should recieve the pile, and the pile itself
}

fn play_turn(player_1: &mut Vec<CardValue>, offhand_1: &mut Vec<CardValue>, player_2: &mut Vec<CardValue>, offhand_2: &mut Vec<CardValue>, rng: &mut ThreadRng) -> TurnOutcome {
    let mut pile = Vec::new();

    loop {
        let card_1 = match get_card(player_1, offhand_1, rng) {
            Some(card) => card,
            None => return TurnOutcome::Winner(Player::Player2),
        };
        pile.push(card_1);

        let card_2 = match get_card(player_2, offhand_2, rng) {
            Some(card) => card,
            None => return TurnOutcome::Winner(Player::Player1),
        };
        pile.push(card_2);

        // If the card is higher the placer gets both cards
        if card_1 > card_2 {
            return TurnOutcome::PileAwarded(Player::Player1, pile);
        }

        if card_2 > card_1 {
            return TurnOutcome::PileAwarded(Player::Player2, pile);
        }

        if let Some(winner) = resolve_war(player_1, offhand_1, player_2, offhand_2, rng, &mut pile) {
            return TurnOutcome::Winner(winner);
        }
    }
}

fn resolve_war(player_1: &mut Vec<CardValue>, offhand_1: &mut Vec<CardValue>, player_2: &mut Vec<CardValue>, offhand_2: &mut Vec<CardValue>, rng: &mut ThreadRng, pile: &mut Vec<CardValue>) -> Option<Player> {
    // Place 3 cards if you run out you lose
    for _ in 0..3 {
        let card_1 = match get_card(player_1, offhand_1, rng) {
            Some(card) => card,
            None => return Some(Player::Player2),
        };
        pile.push(card_1);

        let card_2 = match get_card(player_2, offhand_2, rng) {
            Some(card) => card,
            None => return Some(Player::Player1),
        };
        pile.push(card_2);
    }
    None
}

fn get_card(hand: &mut Vec<CardValue>, mut offhand: &mut Vec<CardValue>, rng: &mut ThreadRng) -> Option<CardValue> {
    // Gets card, if none then shuffle offhand into hand
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