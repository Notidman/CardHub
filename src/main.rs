/**********************************************************
*  PROJECT_NAME: CardHub; 
*  VERSION: 0.0.1; 
*  AUTHOR: Notidman;
*  DATE_OF_CREATION: 21/09/2021;
***********************************************************/
mod card_logic;

use card_logic::*;

fn main() {
    let mut deck = Vec::<Card>::new();
    let mut face = vec!["Ace", "Deuce", "Three", "Four", "Five",
                    "Six", "Seven", "Eight", "Nine", "Ten",
                    "Jack", "Queen", "King"];
    let mut suit = vec!["Hearths", "Diamonds", "Clubs", "Spades"]; 

    fill_deck(&mut deck, &mut face, &mut suit);
    shuffle(&mut deck);
    deal(&deck);
}
