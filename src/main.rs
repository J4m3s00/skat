extern crate rand;

use rand::Rng;

const CARD_COLOR_DIAMONDS:  i32 = 0;
const CARD_COLOR_HEARTS:    i32 = 1;
const CARD_COLOR_SPADES:    i32 = 2;
const CARD_COLOR_CLUBS:     i32 = 3;

const CARD_NUMBER_7:        i32 = 0;
const CARD_NUMBER_8:        i32 = 1;
const CARD_NUMBER_9:        i32 = 2;
const CARD_NUMBER_QUEEN:    i32 = 3;
const CARD_NUMBER_KING:     i32 = 4;
const CARD_NUMBER_10:       i32 = 5;
const CARD_NUMBER_ACE:      i32 = 6;
const CARD_NUMBER_JACK:     i32 = 7;

const CARD_NUMBER_COUNT:    i32 = 8;
const CARD_COLOR_COUNT:     i32 = 4;

fn get_card_color_string(card_color: i32) -> String {
    match card_color {
        CARD_COLOR_DIAMONDS => String::from("Karo"),
        CARD_COLOR_HEARTS => String::from("Herz"),
        CARD_COLOR_SPADES => String::from("Piek"),
        CARD_COLOR_CLUBS => String::from("Kreuz"),
        _ => String::from("Unknown Card color")
    }
}

fn get_card_number_string(card_number: i32) -> String {
    match card_number {
        CARD_NUMBER_7 => String::from("7"),
        CARD_NUMBER_8 => String::from("8"),
        CARD_NUMBER_9 => String::from("9"),
        CARD_NUMBER_10 => String::from("10"),
        CARD_NUMBER_JACK => String::from("Bube"),
        CARD_NUMBER_QUEEN => String::from("Dame"),
        CARD_NUMBER_KING => String::from("King"),
        CARD_NUMBER_ACE => String::from("Ass"),
        _ => String::from("Unknown Card number")
    }
}

#[derive(Debug)]#[derive(Clone)]#[derive(Copy)]
struct Card {
    number: i32,
    color: i32
}

#[derive(Debug)]
struct Player {
    cards: mut Vec<Card>
}

fn shuffle() {
    let mut rng = rand::thread_rng();
    let range = full_deck.len();

    for _ in 0..1000 {
        let first_card = rng.gen_range(0, range);
        let second_card = rng.gen_range(0, range);
        
        full_deck.swap(first_card, second_card);
    }
}

fn give_players_cards() -> (Card, Card)
{
    if full_deck.len() != 32 { return (Card{number: -1, color: -1}, Card{number: -1, color: -1}); }

    let mut current_index : usize = 0;

    for p in 0..3 { 
        for _ in 0..3 { 
            players[p].cards.push(full_deck[current_index]);
            current_index+=1;
        }
    }

    let result = (full_deck[current_index], full_deck[current_index+1]);
    current_index+=2;

    for p in 0..3 { 
        for _ in 0..4 { 
            players[p].cards.push(full_deck[current_index]);
            current_index+=1;
        }
    }

    for p in 0..3 { 
        for _ in 0..3 { 
            players[p].cards.push(full_deck[current_index]);
            current_index+=1;
        }
    }

    return result; 
}

fn is_card_trump(card: &Card) -> bool {
    return card.color == trump || card.number == CARD_NUMBER_JACK;
}

fn is_card_possible(card: &Card, played_color: i32) -> bool {
    return (played_color == trump && card.number == CARD_NUMBER_JACK) || (card.color == played_color && card.number != CARD_NUMBER_JACK); 
}

fn get_possible(hand_cards: &Vec<Card>) -> Vec<Card>{
    if (table_cards.len() == 0) { return hand_cards.clone(); }
    let played_color = table_cards[0].color;
    let result = hand_cards.clone().into_iter().filter(|&card|is_card_possible(&card, played_color)).collect::<Vec<_>>();
    if (result.len() > 0) { return result; }
    return hand_cards.clone();
}

static mut full_deck : Vec<Card> = Vec::new();
static mut table_cards : Vec<Card> = Vec::new();
static mut player_index : usize = 0;
static trump : i32 = CARD_COLOR_HEARTS;

static mut players: [Player; 3] = [Player{cards: Vec::new()},
Player{cards: Vec::new()},
Player{cards: Vec::new()}];

fn do_turn(card: Card) {
    if (table_cards.len() == 0) { table_cards.push(card); players[player_index].cards.into_iter().filter(|&c|c.number == card.number && c.color == card.color).collect::<Vec<Card>>(); }
}   


fn main() {
    full_deck = Vec::new();
    player_index = 0;

    for color in 0..CARD_COLOR_COUNT {
        for number in 0..CARD_NUMBER_COUNT {
            full_deck.push(Card {number: number, color: color});
        }
    }
    shuffle();

    for card in &full_deck {
        print!("{} {}\n", get_card_color_string(card.color), get_card_number_string(card.number));
    }


    let (skat_first, skat_second) = give_players_cards();

    let table = vec![Card{number: CARD_NUMBER_9, color: CARD_COLOR_CLUBS}];

}
