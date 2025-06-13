use rand::prelude::*;

//
// roll sets of dice with dice notation and output rolls to a text file
//

enum DieType {
    D4, D6, D8, D10, D20
}

struct DiceRollParsed {
    dx: DieType,
    ct: u8
}

fn roll_string_parser() {

}

fn do_rolls() {

}
fn main() {
    let mut rng = rand::rng();
    

    let d4: Vec<u8> = (1..4).collect();
    let d6: Vec<u8> = (1..6).collect();
    let d8: Vec<u8> = (1..8).collect();
    let d10: Vec<u8> = (1..10).collect();
    let d20: Vec<u8> = (1..20).collect();
}
