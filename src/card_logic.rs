use rand::{thread_rng, Rng};
use itertools::iproduct;

pub struct Card { face: String, suit: String }
impl Card {
    pub fn new(face: &str, suit: &str) -> Card {
        Card {
            face: face.to_string(),
            suit: suit.to_string(),
        }
    }

    pub fn face(&self) -> &str {
       self.face.as_str()
    }

    pub fn suit(&self) -> &str {
        self.suit.as_str()
    }
}

pub fn fill_deck(deck: &mut Vec<Card>, face: &mut Vec<&str>, suit: &mut Vec<&str>) {
    let mut _i = 0;
    while _i < 52 {
        deck.extend(iproduct![face.iter().copied(),
        suit.iter().copied()].map(|(face, suit)| Card::new(face, suit)));
        _i += 1;
    }
}

pub fn shuffle(deck: &mut Vec<Card>) {
    let mut _i = 0; let mut _j = 0;
    while _i < 52 {
        _j = thread_rng().gen_range(0..52);
        deck.swap( _i, _j);
        _i += 1;
    }
}

pub fn deal(deck: &Vec<Card>) {
    let mut _i = 0;
    while _i < 52 {
        print!("{} of {}{}", deck[_i].face(), deck[_i].suit(), match (_i + 1) % 2 == 0 
        { 
            true => '\t',
            false => '\n'
        } );

        _i += 1;
    }
}
